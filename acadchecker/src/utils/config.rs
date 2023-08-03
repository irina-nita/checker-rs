#![allow(dead_code)]
#![allow(unused_variables)]
#![allow(unused_assignments)]

use std::path::PathBuf;

use acadcheck::{
    checker::{CheckerConfig, MonitorType},
    language::{gcc::Gcc, LanguageProcessor},
};

#[derive(serde::Deserialize, serde::Serialize, Debug)]
pub enum Output {
    /// Error that occurs before the checker can run on tests.
    #[serde(rename = "error")]
    Error(String),
    #[serde(rename = "results")]
    Tests(std::collections::BTreeMap<usize, acadcheck::checker::Output>),
    None,
}

#[derive(serde::Deserialize, serde::Serialize, Debug)]
#[non_exhaustive]
pub enum SupportedProcessor {
    #[serde(rename = "gcc")]
    Gcc {
        #[serde(flatten)]
        gcc: acadcheck::language::gcc::Gcc,
        flags: Vec<String>,
        exec: std::path::PathBuf,
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

#[derive(serde::Deserialize, serde::Serialize, Debug)]
pub struct Security {
    pub user: String,
    pub group: String,
}

#[derive(serde::Deserialize, serde::Serialize, Debug)]
pub struct Config {
    pub checker: CheckerConfig<PathBuf, PathBuf, Vec<MonitorType>>,
    pub processor: SupportedProcessor,
    pub solution: acadcheck::solution::Source,
    pub out_dir: std::path::PathBuf,
    pub security: Option<Security>,
}

impl Config {
    pub fn from_json(path: std::path::PathBuf) -> Result<Self, anyhow::Error> {
        // Check if path exists.
        if !path.exists() {
            return Err(anyhow::format_err!("{:?}: not a valid path!", &path));
        }

        // Open the file from the path.
        let file = std::fs::File::open(path);

        // Return if file can't be open.
        if let Err(e) = file {
            return Err(anyhow::format_err!(e.to_string()));
        }

        let reader = std::io::BufReader::new(file.unwrap());

        let config: Result<crate::utils::Config, serde_json::Error> =
            serde_json::from_reader(reader);

        // Return if configuration cannot be deserialized.
        if let Err(e) = config {
            return Err(anyhow::format_err!(
                "Deserialization from file failed: {}",
                e.to_string()
            ));
        }

        return Ok(config.unwrap());
    }
}

#[cfg(test)]
mod test {
    const TEST_GCC: &str = "/Users/irinanita/acadchecker-testing/config_gcc.json";

    const SOURCE_PATH_GCC: &str =
        "/Users/irinanita/acadchecker-testing/submission_gcc/binary-corect.cpp";

    const IN_PATH: &str = "/Users/irinanita/acadchecker-testing/submission_gcc/tests/in/00";

    const REF_PATH: &str = "/Users/irinanita/acadchecker-testing/submission_gcc/tests/ref/00";

    const OUT_DIR: &str = "/Users/irinanita/acadchecker-testing/submission_gcc/tests/out";

    use std::io::Read;
    use std::os::unix::process::CommandExt;
    use std::time::Duration;
    use std::{collections::HashMap, fmt::format, fs::File, io::Write, path::PathBuf};

    use acadcheck::checker::MonitorType;

    use crate::utils::config::Security;
    use crate::utils::{Config, SupportedProcessor};

    use crate::checker::AcadChecker;

    #[test]
    fn test_build_to_json() {
        // Build compiler
        let gcc =
            acadcheck::language::gcc::Gcc::new(acadcheck::language::gcc::SupportedGccLanguage::Cpp);

        let flags = vec!["-Werror".to_string(), "-Wall".to_string()];

        let exec = PathBuf::from("acadsol".to_string());

        let proc = super::SupportedProcessor::Gcc { gcc, flags, exec };

        let source = acadcheck::solution::Source::File(std::path::PathBuf::from(SOURCE_PATH_GCC));

        let mut h = std::collections::BTreeMap::new();

        for i in 1..6 {
            let input = std::path::PathBuf::from(format!("{}{}.in", IN_PATH, i));
            let refer = std::path::PathBuf::from(format!("{}{}.ref", REF_PATH, i));
            h.insert(i, (input, refer));
        }

        let checker_config = acadcheck::checker::CheckerConfig {
            monitors: vec![MonitorType::Timeout { limit: Duration::from_secs(5) }],
            in_refs: h,
            output_type: acadcheck::checker::OutputType::Scored { per_test: 5 },
        };

        let out_dir = std::path::PathBuf::from(OUT_DIR);

        let user = Security {
            user: "irina".to_string(),
            group: "irina".to_string(),
        };

        let config = Config {
            checker: checker_config,
            processor: proc,
            solution: source,
            out_dir,
            security: Some(user),
        };

        let json = serde_json::to_string_pretty(&config).unwrap();
        let mut f = File::create(TEST_GCC).unwrap();
        let _ = f.write_all(json.as_bytes()).unwrap();
        println!("{}", json);
    }

    #[test]
    fn test_conf() {
        let test_path: std::path::PathBuf = std::path::PathBuf::from(TEST_GCC);
        let builder = AcadChecker::new();

        let config = Config::from_json(test_path).unwrap();

        let security = config.security;

        // Get uid of the user provided. TODO!
        let uid = 501;
        let gid = 20;

        // let checker = acadcheck::checker::Checker::new(config.checker, runner);

        // checker.run(&vec![std::ffi::OsString::from("echo")]);
    }

    #[test]
    fn test_user() {
        let security = Security {
            user: "irinanita".to_string(),
            group: "irina".to_string(),
        };

        let user = nix::unistd::User::from_name(&security.user);
        let group = nix::unistd::Group::from_gid(20.into());
        println!("{:?}", user);
    }
}
