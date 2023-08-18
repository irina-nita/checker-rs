//! GCC implementation as Compiler with support for C, C++, D and Go.
use std::ffi::OsString;

use super::Compiler;

/// Most common languages supported by GCC.
#[non_exhaustive]
#[derive(Debug, Clone, Copy)]
#[cfg_attr(feature = "use-serde", derive(serde::Serialize, serde::Deserialize))]
pub enum SupportedGccLanguage {
    #[cfg_attr(feature = "use-serde", serde(rename = "c"))]
    C,
    #[cfg_attr(feature = "use-serde", serde(rename = "c++"))]
    Cpp,
    #[cfg_attr(feature = "use-serde", serde(rename = "d"))]
    D,
    #[cfg_attr(feature = "use-serde", serde(rename = "go"))]
    Go,
}

#[derive(Debug, Clone, Copy)]
#[cfg_attr(feature = "use-serde", derive(serde::Serialize, serde::Deserialize))]
pub struct Gcc {
    language: SupportedGccLanguage,
}

impl Gcc {
    pub fn new(language: SupportedGccLanguage) -> Self {
        Self { language }
    }
}

impl crate::language::LanguageProcessor for Gcc {
    fn run(
        &self,
        args: Option<Vec<std::ffi::OsString>>,
        source: crate::solution::Source,
        exec: Option<std::path::PathBuf>,
    ) -> Result<Vec<std::ffi::OsString>, crate::language::Error> {
        if exec == None {
            return Err(crate::language::Error::CompilationFailed(format!(
                "No path for the executable provided."
            )));
        }
        self.run_compiled(args, &source, exec.unwrap())
    }
}

impl crate::language::Compiler for Gcc {
    fn run_compiled<S, I>(
        &self,
        flags: Option<I>,
        source: &crate::solution::Source,
        exec: std::path::PathBuf,
    ) -> Result<Vec<std::ffi::OsString>, crate::language::Error>
    where
        S: AsRef<std::ffi::OsStr>,
        I: IntoIterator<Item = S>,
    {
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

        // Destination file (executable).
        let dest = exec.to_str().unwrap();

        // Exec name of the compiler.
        let exec_without_path = match &self.language {
            crate::language::gcc::SupportedGccLanguage::C => {
                std::ffi::OsString::from(format!("gcc"))
            }
            crate::language::gcc::SupportedGccLanguage::Cpp => {
                std::ffi::OsString::from(format!("g++"))
            }
            crate::language::gcc::SupportedGccLanguage::D => {
                std::ffi::OsString::from(format!("gdc"))
            }
            crate::language::gcc::SupportedGccLanguage::Go => {
                std::ffi::OsString::from(format!("gccgo"))
            }
            _ => {
                panic!("Not supported yet!")
            }
        };

        // Build command.
        let mut compile_command = std::process::Command::new(exec_without_path);

        let compile_command = match flags {
            Some(f) => compile_command.args(f),
            None => &mut compile_command,
        };

        let compile_command = compile_command
            .arg(source)
            .arg("-o")
            .arg(dest)
            .stderr(std::process::Stdio::null())
            .stdout(std::process::Stdio::null());

        // Execute and wait for status.
        let exit_status = compile_command.status();

        if let Err(e) = exit_status {
            return Err(crate::language::Error::CompilationFailed(format!(
                "{}. (The compiler might not be in your PATH.)",
                e.to_string()
            )));
        }

        // Get exit status without panic.
        let exit_status = exit_status.unwrap();

        if exit_status.success() {
            // Return the command of the executable on success.
            let binary = exec.canonicalize();

            if let Err(_) = binary {
                return Err(crate::language::Error::CompilationFailed(format!(
                    "Building path to executable received an error."
                )));
            }

            let mut command = Vec::new();

            command.push(OsString::from(binary.unwrap()));

            return Ok(command);
        } else {
            return Err(crate::language::Error::CompilationFailed(format!(
                "{}",
                exit_status
            )));
        }
    }
}
