//! GCC implementation as Compiler.

use crate::lang::Compiler;

/// Languages supported by GCC.
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
    fn get_lang(&self) -> (std::ffi::OsString, std::ffi::OsString) {
        match &self.language {
            crate::lang::gcc::SupportedGccLanguage::C => (
                std::ffi::OsString::from("gcc"),
                std::ffi::OsString::from("c"),
            ),
            crate::lang::gcc::SupportedGccLanguage::Cpp => (
                std::ffi::OsString::from("g++"),
                std::ffi::OsString::from("c++"),
            ),
            crate::lang::gcc::SupportedGccLanguage::D => (
                std::ffi::OsString::from("gdc"),
                std::ffi::OsString::from("d"),
            ),
            crate::lang::gcc::SupportedGccLanguage::Go => (
                std::ffi::OsString::from("gccgo"),
                std::ffi::OsString::from("go"),
            ),
        }
    }
}

impl Default for Gcc {
    fn default() -> Self {
        Self {
            language: SupportedGccLanguage::C,
        }
    }
}

impl crate::lang::LanguageProcessor for Gcc {
    fn run<S, P>(
        &self,
        args: Option<Vec<S>>,
        source: crate::submission::Source<P>,
        exec: Option<P>,
    ) -> Result<Vec<std::ffi::OsString>, crate::error::CheckerError>
    where
        S: AsRef<std::ffi::OsStr>,
        P: AsRef<std::path::Path>,
    {
        match exec {
            Some(e) => self.run_compiled(args, &source, e),
            None => Err(crate::error::build_error!()),
        }
    }
}

impl crate::lang::Compiler for Gcc {
    fn run_compiled<S, I, P>(
        &self,
        flags: Option<I>,
        source: &crate::submission::Source<P>,
        exec: P,
    ) -> Result<Vec<std::ffi::OsString>, crate::error::CheckerError>
    where
        S: AsRef<std::ffi::OsStr> + Sized,
        I: IntoIterator<Item = S>,
        P: AsRef<std::path::Path>,
    {
        // Get source to pass into command.
        let source = std::ffi::OsString::from(source);

        // Get executable to pass into command.
        let (compiler, lang) = self.get_lang();

        // Build command.
        let mut compile_command = std::process::Command::new(compiler);

        let compile_command = match flags {
            Some(f) => compile_command.args(f),
            None => &mut compile_command,
        };

        // Build command.
        let compile_command = compile_command
            .arg("-x")
            .arg(lang)
            .arg(source)
            .arg("-o")
            .arg(exec.as_ref())
            .stderr(std::process::Stdio::null())
            .stdout(std::process::Stdio::null());

        // Execute and wait for output and status.
        let output = compile_command.output();

        match output {
            Err(err) => Err(crate::error::build_error!(source: err)),
            Ok(out) => {
                if out.status.success() {
                    // Return the command of the executable on success.
                    // Try to canonicalize the path
                    if let Ok(ref binary) = exec.as_ref().canonicalize() {
                        Ok(vec![std::ffi::OsString::from(binary)])
                    } else {
                        Ok(vec![std::ffi::OsString::from(exec.as_ref())])
                    }
                } else {
                    Err(
                        crate::error::execution_error!(message: format!("ExitStatus: {}", out.status)),
                    )
                }
            }
        }
    }
}
