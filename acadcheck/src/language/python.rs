//! Python implementation as Interpreter with support for specific versions.

use super::Interpreter;
#[derive(Debug)]
#[cfg_attr(feature = "use-serde", derive(serde::Serialize, serde::Deserialize))]
pub struct Python {
    version: Option<String>,
}

impl Python {
    /// Returns a Python instance without a version.
    pub fn new() -> Self {
        return Self { version: None };
    }

    /// Adds a version. If the format is invalid, it will return an error.
    pub fn with_version<S>(mut self, version: S) -> Result<Self, anyhow::Error>
    where
        S: AsRef<str>,
    {
        // Regex to match the version.
        let version_reg = regex::Regex::new(r"^([1-3](\.[0-9]{1,2})?)$").unwrap();

        if version_reg.is_match(version.as_ref()) {
            self.version = Some(String::from(version.as_ref()));
            return Ok(self);
        } else {
            return Err(anyhow::format_err!("Version of python is not valid"));
        }
    }
}

impl crate::language::LanguageProcessor for Python {
    fn run(
        &self,
        args: Option<Vec<std::ffi::OsString>>,
        source: crate::solution::Source,
        _exec: Option<std::path::PathBuf>,
    ) -> Result<Vec<std::ffi::OsString>, crate::language::Error> {
        Ok(self.run_interpreted(args, source))
    }
}

impl crate::language::Interpreter for Python {
    fn run_interpreted<S, I>(
        &self,
        flags: Option<I>,
        source: crate::solution::Source,
    ) -> Vec<std::ffi::OsString>
    where
        S: AsRef<std::ffi::OsStr>,
        I: IntoIterator<Item = S>,
    {
        // Python exec that should be in PATH.
        let py = match &self.version {
            Some(version) => std::ffi::OsString::from(format!("python{}", version)),
            None => std::ffi::OsString::from(format!("python")),
        };

        // Get source os string to pass into command.
        let source = match source {
            crate::solution::Source::File(file) => std::ffi::OsString::from(file.to_str().unwrap()),
            crate::solution::Source::Directory(dir) => {
                std::ffi::OsString::from(format!("{}/*", dir.to_str().unwrap()))
            }
            crate::solution::Source::Regex { regex } => {
                std::ffi::OsString::from(format!("{}", regex.as_str()))
            }
            _ => {
                panic!("Source type not supported yet!")
            }
        };

        // Build command vector.
        let mut command: Vec<std::ffi::OsString> = Vec::new();
        command.push(py);
        if let Some(flags) = flags {
            let _ = flags.into_iter().map(|f| {
                let f = std::ffi::OsString::from(&f);
                command.push(f);
            });
        }
        command.push(source);

        command
    }
}
