#[cfg(test)]
pub mod tests;

use anyhow::anyhow;
use async_trait::async_trait;
use futures_util::{StreamExt, TryStreamExt};
use shiplift::{ContainerOptions, PullOptions, RmContainerOptions};
use std::{
    io::{Read, Seek},
    path::PathBuf,
};
use tempfile::NamedTempFile;

pub(crate) const MEMORY_LIMIT: u64 = 300 * 1024 * 1024;
pub(crate) const MEMORY_SWAP: i64 = 600 * 1024 * 1024;
pub(crate) const NANO_CPUS: u64 = 500 * 1000 * 1000;

/// Trait for orchestrators that deal with creating and destroying sandboxes.
#[async_trait]
pub trait Orchestrator<'orch> {
    /// Isolated environment that the checker could be ran
    /// into.
    type SandboxType: Sandbox + Sync;

    /// Build a sandbox from an image. Return the sandbox.
    async fn build_sandbox_from(
        &'orch self,
        image: &str,
        command: Vec<&str>,
    ) -> anyhow::Result<Self::SandboxType>;

    /// Destroy the sandbox. This is crucial if the sandbox can't exit properly on its own.
    async fn destroy_sandbox(&self, sandbox_identifier: String) -> anyhow::Result<()>;
}

#[async_trait]
pub trait Sandbox {
    /// Copy a temporary file that can be accessed by path into sandbox.
    async fn copy_file(
        &self,
        file: &mut tempfile::NamedTempFile,
        dir_path: &std::path::Path,
    ) -> anyhow::Result<()>;

    /// Copy multiple temporary files that can be accessed by paths into sandbox.
    async fn copy_files(
        &self,
        files: &mut Vec<tempfile::NamedTempFile>,
        dir_path: &std::path::Path,
    ) -> anyhow::Result<()> {
        for f in files {
            if let Err(e) = self.copy_file(f, dir_path).await {
                return Err(anyhow::format_err!("{:?}", e));
            }
        }
        Ok(())
    }

    /// Run the checker inside the container and get output.
    async fn run_checker(&self) -> anyhow::Result<acadcheck::acadchecker::config::Output>;

    /// Get identifier.
    fn get_identifier(&self) -> String;
}

#[async_trait]
impl Sandbox for shiplift::Container<'_> {
    async fn copy_file(
        &self,
        file: &mut tempfile::NamedTempFile,
        dir_path: &std::path::Path,
    ) -> anyhow::Result<()> {
        // Buffer for file contents.
        let mut buf = vec![];

        match file.read_to_end(&mut buf) {
            Result::Ok(_) => {}
            Err(e) => {
                return Err(anyhow::format_err!("{:?}", e.to_string()));
            }
        }

        // Take filename and build path.
        let filename = match file.path().file_name() {
            Some(f) => f,
            None => {
                return Err(anyhow::format_err!("Can't move files to sandbox."));
            }
        };

        let mut path = PathBuf::from(dir_path);
        path.push(filename);

        file.seek(std::io::SeekFrom::Start(0)).unwrap();
        // Copy file in path.
        match self.copy_file_into(path, &buf).await {
            Result::Ok(_) => {
                return anyhow::Ok(());
            }
            Err(e) => {
                return Err(anyhow::format_err!("{:?}", e.to_string()));
            }
        }
    }

    async fn run_checker(&self) -> anyhow::Result<acadcheck::acadchecker::config::Output> {
        use shiplift::tty::TtyChunk;

        let (read, _) = self.attach().await.unwrap().split();

        self.start().await.unwrap();

        let mut output = String::new();
        let mut error = String::new();

        let mut get_res = |chunks: Vec<TtyChunk>| {
            for chunk in chunks {
                match chunk {
                    TtyChunk::StdOut(bytes) => {
                        output.push_str(std::str::from_utf8(&bytes).unwrap());
                    }
                    TtyChunk::StdErr(bytes) => {
                        error.push_str(std::str::from_utf8(&bytes).unwrap());
                    }
                    TtyChunk::StdIn(_) => unreachable!(),
                }
            }
        };

        match read.try_collect::<Vec<_>>().await {
            Ok(spec_result) => (get_res)(spec_result),
            Err(e) => {
                return Err(anyhow::format_err!(
                    "Could not get the logs from docker container in the current system: {}",
                    e
                ));
            }
        }

        if !error.is_empty() {
            return Err(anyhow::format_err!("{}", error));
        } else {
            // Try to get result from output.
            let res = match serde_json::from_str::<acadcheck::acadchecker::config::Output>(
                output.as_str(),
            ) {
                Ok(r) => r,
                Err(e) => {
                    return Err(anyhow::format_err!("{}", e.to_string()));
                }
            };

            return Result::Ok(res);
        }
    }

    fn get_identifier(&self) -> String {
        self.id().to_string()
    }
}

#[async_trait]
impl<'orch> Orchestrator<'orch> for shiplift::Docker {
    type SandboxType = shiplift::Container<'orch>;

    async fn build_sandbox_from(
        &'orch self,
        image: &str,
        command: Vec<&str>,
    ) -> anyhow::Result<Self::SandboxType> {
        // TODO: Add namespace.
        // Spawn docker container from image.

        let id = match self
            .containers()
            .create(
                &ContainerOptions::builder(image)
                    .cmd(command.clone())
                    .memory(MEMORY_LIMIT)
                    .memory_swap(MEMORY_SWAP)
                    .nano_cpus(NANO_CPUS)
                    .privileged(false)
                    .attach_stdin(false)
                    .attach_stderr(true)
                    .attach_stdout(true)
                    .build(),
            )
            .await
        {
            Ok(info) => info.id,
            Err(e) => {
                // Fault 404 means the image is not found most of the times.
                if let shiplift::Error::Fault {
                    code: http::status::StatusCode::NOT_FOUND,
                    ..
                } = e
                {
                    // Try to pull image & run again.
                    let mut stream = self
                        .images()
                        .pull(&PullOptions::builder().image(image).build());
                    if let Some(res) = stream.next().await {
                        match res {
                            Ok(_) => {
                                return self.build_sandbox_from(image, command).await;
                            }
                            Err(e) => {
                                return Err(anyhow::format_err!(
                                    "Could not even pull image.{:?}",
                                    e.to_string()
                                ));
                            }
                        }
                    }
                }
                return Err(anyhow::format_err!("{:?}", e.to_string()));
            }
        };

        Ok(self.containers().get(&id))
    }

    async fn destroy_sandbox(&self, sandbox_identifier: String) -> anyhow::Result<()> {
        let c = self.containers().get(sandbox_identifier.clone());
        if let Ok(container_state) = c.inspect().await {
            // If the container is still running for some reason.
            if container_state.state.running {
                // Try to stop it gracefully.
                if c.stop(Some(std::time::Duration::from_secs(5)))
                    .await
                    .is_err()
                {
                    // If it can't, kill it.
                    if let Err(e) = c.kill(None).await {
                        return Err(anyhow::format_err!(
                            "Container with id {} is going to sh*t with {}.",
                            sandbox_identifier.as_str(),
                            e.to_string()
                        ));
                    }
                }
            }
        }

        if let Err(e) = c
            .remove(RmContainerOptions::builder().force(true).build())
            .await
        {
            return Err(anyhow::format_err!(
                "Container with id {} is going to sh*t with {}.",
                sandbox_identifier.as_str(),
                e.to_string()
            ));
        }

        Ok(())
    }
}

/// Sandbox that can run a checker.
pub struct SandboxedChecker<'a> {
    command: Vec<&'a str>,
    config: std::sync::Arc<crate::api::utils::SandboxConfig>,
}

impl<'a> SandboxedChecker<'a> {
    /// Constructor for SandboxedChecker.
    pub(crate) fn new(
        command: Vec<&'a str>,
        config: std::sync::Arc<crate::api::utils::SandboxConfig>,
    ) -> Self {
        Self { command, config }
    }

    /// Run the checker once. Before drop, the orchestrator will try to destroy the sandbox.
    /// __Will panic if it can't kill the sandbox.__
    pub async fn run_once<'orch, O>(
        self,
        orchestrator: &'orch O,
        ins: &mut Vec<NamedTempFile>,
        solution: &mut NamedTempFile,
        refs: &mut Vec<NamedTempFile>,
        config: &mut NamedTempFile,
    ) -> acadcheck::acadchecker::config::Output
    where
        O: Orchestrator<'orch>,
    {
        let sandbox = match orchestrator
            .build_sandbox_from(self.config.image.as_str(), self.command)
            .await
        {
            Ok(s) => s,
            Err(e) => {
                return acadcheck::acadchecker::config::Output::Error(e.to_string());
            }
        };

        let (in_path, ref_path, config_path, solution_path) = (
            &self.config.r#in,
            &self.config.r#ref,
            &self.config.cfg,
            &self.config.src,
        );

        if let Err(e) = sandbox.copy_files(ins, in_path).await {
            return acadcheck::acadchecker::config::Output::Error(e.to_string());
        }
        if let Err(e) = sandbox.copy_files(refs, ref_path).await {
            return acadcheck::acadchecker::config::Output::Error(e.to_string());
        }
        if let Err(e) = sandbox.copy_file(config, config_path).await {
            return acadcheck::acadchecker::config::Output::Error(e.to_string());
        }
        if let Err(e) = sandbox.copy_file(solution, solution_path).await {
            return acadcheck::acadchecker::config::Output::Error(e.to_string());
        }

        let output = match sandbox.run_checker().await {
            Ok(out) => out,
            Err(e) => acadcheck::acadchecker::config::Output::Error(e.to_string()),
        };

        match orchestrator.destroy_sandbox(sandbox.get_identifier()).await {
            Ok(_) => output,
            Err(e) => acadcheck::acadchecker::config::Output::Error(e.to_string()),
        }
    }
}
