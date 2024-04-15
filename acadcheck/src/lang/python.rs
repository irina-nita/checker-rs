//! Python implementation as Interpreter.

use super::Interpreter;
#[derive(Debug, Clone)]
#[cfg_attr(feature = "use-serde", derive(serde::Serialize, serde::Deserialize))]
pub struct Python {
    version: Option<String>,
}

impl Default for Python {
    fn default() -> Self {
        Self::new()
    }
}

impl Python {
    /// Returns a Python instance without a version.
    pub fn new() -> Self {
        Self { version: None }
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
            Ok(self)
        } else {
            Err(anyhow::format_err!("Version of python is not valid"))
        }
    }
}

impl crate::lang::LanguageProcessor for Python {
    fn run<S, P>(
        &self,
        args: Option<Vec<S>>,
        source: crate::submission::Source<P>,
        _exec: Option<P>,
    ) -> Result<Vec<std::ffi::OsString>, crate::error::CheckerError>
    where
        S: AsRef<std::ffi::OsStr>,
        P: AsRef<std::path::Path>,
    {
        Ok(self.run_interpreted(args, source))
    }
}

impl crate::lang::Interpreter for Python {
    fn run_interpreted<S, I, P>(
        &self,
        flags: Option<I>,
        source: crate::submission::Source<P>,
    ) -> Vec<std::ffi::OsString>
    where
        S: AsRef<std::ffi::OsStr>,
        I: IntoIterator<Item = S>,
        P: AsRef<std::path::Path>,
    {
        // Python exec that should be in PATH.
        let py = match &self.version {
            Some(version) => std::ffi::OsString::from(format!("python{}", version)),
            None => std::ffi::OsString::from("python"),
        };

        // Get source os string to pass into command.
        let source = match source {
            crate::submission::Source::File(file) => std::ffi::OsString::from(file.as_ref()),
            crate::submission::Source::Directory(path) => {
                std::ffi::OsString::from(format!("{}/*", path.as_ref().to_string_lossy()))
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
