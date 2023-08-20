#[cfg(test)]
pub mod tests;

use async_trait::async_trait;
use futures_util::{AsyncWriteExt, StreamExt, TryStreamExt};
use shiplift::{
    builder::ContainerOptionsBuilder, tty::TtyChunk, ContainerOptions, Docker, Exec,
    ExecContainerOptions, RmContainerOptions,
};
use std::{fs::File, io::Read, path::PathBuf};
use tempfile::NamedTempFile;

#[async_trait]
pub trait Orchestrator<'orch> {
    type SandboxType: Sandbox + Sync;
    type SandboxIdentifier;
    async fn build_sandbox_from(
        &'orch self,
        image: &str,
        command: Vec<&str>,
    ) -> anyhow::Result<Self::SandboxType>;
    async fn destroy_sandbox(
        &self,
        sandbox_identifier: Self::SandboxIdentifier,
    ) -> anyhow::Result<()>;
}

pub struct SandboxedChecker<'a> {
    image: &'a str,
    command: Vec<&'a str>,
    config: std::sync::Arc<crate::api::utils::SandboxConfig>,
}

impl<'a> SandboxedChecker<'a> {
    pub(crate) fn new(
        image: &'a str,
        command: Vec<&'a str>,
        config: std::sync::Arc<crate::api::utils::SandboxConfig>,
    ) -> Self {
        Self {
            image,
            command,
            config,
        }
    }

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
            .build_sandbox_from(self.image, self.command)
            .await
        {
            Ok(s) => s,
            Err(e) => {
                return acadcheck::acadchecker::config::Output::Error(e.to_string());
            }
        };

        let in_path = &self.config.r#in;
        let ref_path = &self.config.r#ref;
        let config_path = &self.config.r#cfg;
        let solution_path = &self.config.src;

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

        sandbox.run_checker().await.unwrap();
        acadcheck::acadchecker::config::Output::None
    }
}

#[async_trait]
pub trait Sandbox {
    async fn copy_file(
        &self,
        file: &mut tempfile::NamedTempFile,
        dir_path: &std::path::Path,
    ) -> anyhow::Result<()>;

    async fn copy_files(
        &self,
        files: &mut Vec<tempfile::NamedTempFile>,
        dir_path: &std::path::Path,
    ) -> anyhow::Result<()> {
        for mut f in files {
            if let Err(e) = self.copy_file(&mut f, dir_path).await {
                return Err(anyhow::format_err!("{:?}", e));
            }
        }
        Ok(())
    }

    async fn run_checker(&self) -> anyhow::Result<acadcheck::acadchecker::config::Output>;
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

        // Let's just print them for now.
        println!("Output: {}", output);
        eprintln!("Error: {}", error);
        Ok(acadcheck::acadchecker::config::Output::None)
    }
}

#[async_trait]
impl<'orch> Orchestrator<'orch> for shiplift::Docker {
    type SandboxType = shiplift::Container<'orch>;
    type SandboxIdentifier = String;

    async fn build_sandbox_from(
        &'orch self,
        image: &str,
        command: Vec<&str>,
    ) -> anyhow::Result<Self::SandboxType> {
        // Spawn docker container from image (on local for now)
        let id = match self
            .containers()
            .create(
                &ContainerOptions::builder(image)
                    .cmd(command)
                    .attach_stderr(true)
                    .attach_stdout(true)
                    .build(),
            )
            .await
        {
            Ok(info) => info.id,
            Err(e) => {
                return Err(anyhow::format_err!("{:?}", e.to_string()));
            }
        };

        Ok(self.containers().get(&id))
    }

    async fn destroy_sandbox(
        &self,
        sandbox_identifier: Self::SandboxIdentifier,
    ) -> anyhow::Result<()> {
        let c = self.containers().get(sandbox_identifier);
        c.kill(None).await.unwrap();
        c.remove(RmContainerOptions::builder().force(true).build())
            .await
            .unwrap();

        Ok(())
    }
}
