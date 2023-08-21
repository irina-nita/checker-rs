//! Helpers for Acadchecker.
#[derive(serde::Deserialize, serde::Serialize, Debug)]
pub enum Output {
    /// Error that occurs before the checker can run on tests.
    #[serde(rename = "error")]
    Error(String),
    #[serde(rename = "results")]
    Tests(std::collections::BTreeMap<usize, crate::checker::Output>),
    None,
}

#[derive(serde::Deserialize, serde::Serialize, Debug, Clone)]
#[non_exhaustive]
pub enum SupportedProcessor {
    #[serde(rename = "gcc")]
    Gcc {
        #[serde(flatten)]
        gcc: crate::language::gcc::Gcc,
        flags: Vec<String>,
        exec: std::path::PathBuf,
    },
    #[serde(rename = "python")]
    Python {
        #[serde(flatten)]
        python: crate::language::python::Python,
        flags: Vec<String>,
    },
    #[serde(rename = "makefile")]
    Makefile {
        #[serde(flatten)]
        makefile: crate::language::make::Makefile,
    },
}

#[derive(serde::Deserialize, serde::Serialize, Debug, Clone)]
pub struct Security {
    pub user: String,
    pub group: String,
}

#[derive(serde::Deserialize, serde::Serialize, Debug)]
pub struct Config {
    pub checker: crate::checker::CheckerConfig<
        std::path::PathBuf,
        std::path::PathBuf,
        Vec<crate::checker::MonitorType>,
    >,
    pub processor: SupportedProcessor,
    pub solution: crate::solution::Source,
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

        // Return if file can't be opened.
        if let Err(e) = file {
            return Err(anyhow::format_err!(e.to_string()));
        }

        let reader = std::io::BufReader::new(file.unwrap());

        let config: Result<crate::acadchecker::config::Config, serde_json::Error> =
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
