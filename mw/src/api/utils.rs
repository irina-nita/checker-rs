use std::{
    fs::File,
    io::{Read, Seek, Write},
    path::{Path, PathBuf},
    str::from_utf8,
};

use acadcheck::language::{gcc::Gcc, make::Makefile};
use futures_util::{AsyncWriteExt, StreamExt, TryStreamExt};
use shiplift::{
    builder::ContainerOptionsBuilder, tty::TtyChunk, ContainerOptions, Docker, Exec,
    ExecContainerOptions,
};
use std::collections::BTreeMap;
use tempfile::NamedTempFile;
use zip::read::ZipFile;

// TODO: Change this so it isn't constant. Need new authorized route to change configuration of
// these.
pub(crate) const BUCKET_NAME: &str = "acadnet";

pub(crate) const TESTS_ARCHIVE: &str = "tests.zip";
pub const PROVIDER_NAME: &str = "CustomEnvironment";

pub(crate) const IN_REGEX: &str = "in/[0-9][0-9][0-9].in";
pub(crate) const REF_REGEX: &str = "ref/[0-9][0-9][0-9].ref";

/// Basic response message on any response with status other than 200 OK.
#[derive(serde::Deserialize, serde::Serialize)]
pub(crate) struct Response {
    pub(crate) message: String,
}

/// Solution, problem and configuration.
#[derive(actix_multipart::form::MultipartForm)]
pub(crate) struct UploadSolution {
    pub(crate) solution: actix_multipart::form::tempfile::TempFile,
    pub(crate) problem: actix_multipart::form::text::Text<String>,
    pub(crate) config: actix_multipart::form::json::Json<UploadConfig>,
}

#[derive(serde::Deserialize, serde::Serialize, Debug)]
#[non_exhaustive]
pub(crate) struct UploadTimeLimit {
    pub(crate) secs: u64,
}

#[derive(serde::Deserialize, serde::Serialize, Debug)]
pub(crate) struct UploadConfig {
    pub(crate) processor: UploadSupportedProcessor,
    pub(crate) time_limit: UploadTimeLimit,
}

/// Fix as the client shouldn't parse the executable name.
#[derive(serde::Deserialize, serde::Serialize, Debug)]
#[non_exhaustive]
pub(crate) enum UploadSupportedProcessor {
    #[serde(rename = "gcc")]
    Gcc {
        #[serde(flatten)]
        gcc: acadcheck::language::gcc::Gcc,
        flags: Vec<String>,
    },
    #[serde(rename = "python")]
    Python {
        #[serde(flatten)]
        python: acadcheck::language::python::Python,
        flags: Vec<String>,
    },
    #[serde(rename = "makefile")]
    Makefile {
        #[serde(flatten)]
        makefile: acadcheck::language::make::Makefile,
    },
}

/// Trait to change the received config to checker-supported config.
impl Into<acadcheck::acadchecker::config::SupportedProcessor> for &UploadSupportedProcessor {
    fn into(self) -> acadcheck::acadchecker::config::SupportedProcessor {
        match self {
            UploadSupportedProcessor::Gcc { gcc, flags } => {
                return acadcheck::acadchecker::config::SupportedProcessor::Gcc {
                    gcc: gcc.clone(),
                    flags: flags.clone(),
                    exec: std::path::PathBuf::from("/restricted/home/sandbox/solution"),
                };
            }
            UploadSupportedProcessor::Python { python, flags } => {
                return acadcheck::acadchecker::config::SupportedProcessor::Python {
                    python: python.clone(),
                    flags: flags.clone(),
                };
            }
            UploadSupportedProcessor::Makefile { makefile } => {
                return acadcheck::acadchecker::config::SupportedProcessor::Makefile {
                    makefile: makefile.clone(),
                };
            }
        }
    }
}

/// Sandbox configuration. (Made with a Docker container in mind.)
#[derive(serde::Deserialize, serde::Serialize, Debug, Clone)]
pub(crate) struct SandboxConfig {
    pub(crate) image: String,
    pub(crate) r#in: PathBuf,
    pub(crate) r#ref: PathBuf,
    pub(crate) out: PathBuf,
    pub(crate) cfg: PathBuf,
    pub(crate) src: PathBuf,
    pub(crate) security: acadcheck::acadchecker::config::Security,
}

/// ---------------------------------------------------------------------------
///                         HELPERS FOR THE API
/// ---------------------------------------------------------------------------
pub trait InRefHolder {
    fn insert_in(&mut self, f: NamedTempFile, key: usize);
    fn insert_ref(&mut self, f: NamedTempFile, key: usize);
}

impl InRefHolder for BTreeMap<usize, (Option<NamedTempFile>, Option<NamedTempFile>)> {
    fn insert_in(&mut self, f: NamedTempFile, key: usize) {
        if let Some(p) = self.get_mut(&key) {
            p.0 = Some(f);
        } else {
            self.insert(key, (Some(f), None));
        }
    }

    fn insert_ref(&mut self, f: NamedTempFile, key: usize) {
        if let Some(p) = self.get_mut(&key) {
            p.1 = Some(f);
        } else {
            self.insert(key, (None, Some(f)));
        }
    }
}

pub trait To<T> {
    fn write_to(&mut self, other: &mut T) -> anyhow::Result<()>;
}

impl To<NamedTempFile> for ZipFile<'_> {
    fn write_to(&mut self, other: &mut NamedTempFile) -> anyhow::Result<()> {
        // Read contents to String.
        let mut contents = String::new();
        match self.read_to_string(&mut contents) {
            Ok(_) => {}
            Err(e) => {
                return Err(anyhow::format_err!("{}", e.to_string()));
            }
        }

        // Current patch for Windows' carriage return:
        let contents = contents.replace("\r\n", "\n");

        match other.write_all(contents.as_bytes()) {
            Ok(_) => {}
            Err(e) => {
                return Err(anyhow::format_err!("{}", e.to_string()));
            }
        }

        // Move back the cursor.
        match other.seek(std::io::SeekFrom::Start(0)) {
            Ok(_) => {}
            Err(e) => {
                return Err(anyhow::format_err!("{}", e.to_string()));
            }
        }
        Ok(())
    }
}
