#![allow(dead_code)]

use std::{
    fs::File,
    io::Read,
    path::{Path, PathBuf},
    str::from_utf8,
};

use acadcheck::language::{gcc::Gcc, make::Makefile};
use futures_util::{AsyncWriteExt, StreamExt, TryStreamExt};
use shiplift::{
    builder::ContainerOptionsBuilder, tty::TtyChunk, ContainerOptions, Docker, Exec,
    ExecContainerOptions,
};

pub(crate) const BUCKET_NAME: &'static str = "acadnet";
pub(crate) const TESTS_ARCHIVE: &'static str = "tests.zip";
pub(crate) const PROVIDER_NAME: &'static str = "CustomEnvironment";
pub(crate) const IN_REGEX: &'static str = "in/[0-9][0-9][0-9].in";
pub(crate) const REF_REGEX: &'static str = "ref/[0-9][0-9][0-9].ref";
pub(crate) const IMAGE: &'static str = "core";
pub(crate) const URL: &'static str = "http://0.0.0.0:3000/checker/run";

#[derive(serde::Deserialize, serde::Serialize)]
pub(crate) struct Response {
    pub(crate) message: String,
}

#[derive(serde::Deserialize, serde::Serialize)]
pub(crate) struct CreateResponse {
    pub(crate) submission_id: String,
}

#[derive(serde::Deserialize, serde::Serialize)]
pub(crate) struct StatusResponse {
    pub(crate) status: String,
}

/// Solution, problem and configuration.
#[derive(actix_multipart::form::MultipartForm)]
pub(crate) struct UploadSolution {
    pub(crate) solution: actix_multipart::form::tempfile::TempFile,
    pub(crate) problem: actix_multipart::form::text::Text<String>,
    pub(crate) config: actix_multipart::form::json::Json<UploadConfig>,
}

#[derive(serde::Deserialize, serde::Serialize, Debug)]
pub(crate) struct UploadConfig {
    pub(crate) processor: UploadSupportedProcessor,
    pub(crate) time_limit: UploadTimeLimit,
}

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

#[derive(serde::Deserialize, serde::Serialize, Debug)]
#[non_exhaustive]
pub(crate) struct UploadTimeLimit {
    pub(crate) secs: u64,
}

#[derive(serde::Deserialize, serde::Serialize, Debug)]
pub(crate) struct SandboxConfig {
    pub(crate) r#in: PathBuf,
    pub(crate) r#ref: PathBuf,
    pub(crate) out: PathBuf,
    pub(crate) cfg: PathBuf,
    pub(crate) src: PathBuf,
    pub(crate) security: acadcheck::acadchecker::config::Security,
}
