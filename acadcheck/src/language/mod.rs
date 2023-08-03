//! Traits, structs and helpers related for defining language processors.

pub mod gcc;
pub mod make;
pub mod python;

/// Errors regarding the language processor used during compiling or
/// interpreting.
#[derive(thiserror::Error, Debug)]
#[non_exhaustive]
pub enum Error {
    /// If the compilation failed, the inner type should keep information about
    /// the failing.
    #[error("Compilation failed: {0}")]
    CompilationFailed(String),

    /// If the build rule for Makefile failed, the inner type should keep
    /// information about the failing.
    #[error("Make build failed: {0}")]
    MakefileBuildFailed(String),
}

/// Trait for language processors such as compilers or interpreters.
pub trait LanguageProcessor: std::fmt::Debug {
    fn run(
        &self,
        args: Option<Vec<std::ffi::OsString>>,
        source: crate::solution::Source,
        exec: Option<std::path::PathBuf>,
    ) -> Result<Vec<std::ffi::OsString>, crate::language::Error>;
}
/// Compiler trait for Language Processors.
pub trait Compiler: LanguageProcessor {
    /// Compiling a program. Returns the error of the compilation.
    /// If it was successful, it returns the command for running the executable
    /// produced.
    fn run_compiled<S, I>(
        &self,
        flags: Option<I>,
        source: &crate::solution::Source,
        exec: std::path::PathBuf,
    ) -> Result<Vec<std::ffi::OsString>, crate::language::Error>
    where
        S: AsRef<std::ffi::OsStr>,
        I: IntoIterator<Item = S>;
}

/// Interpreter trait for Language Processors.
pub trait Interpreter: LanguageProcessor {
    /// Returns the command for running the executable along with the
    /// interpreter..
    fn run_interpreted<S, I>(
        &self,
        flags: Option<I>,
        source: crate::solution::Source,
    ) -> Vec<std::ffi::OsString>
    where
        S: AsRef<std::ffi::OsStr>,
        I: IntoIterator<Item = S>;
}

/// Makefiles could be considered as a form of "language processors". It depends
/// on the use case. Each makefile should have at least a rule for "run".
pub trait Makefile: LanguageProcessor {
    /// Returns the command that acts as a rule for run.
    /// If a build was required before-hand, the implementation should propagate
    /// the error as [`language::Error`](crate::language::Error).
    fn run<S>(&self, target: Option<S>) -> Result<Vec<std::ffi::OsString>, crate::language::Error>
    where
        S: AsRef<std::ffi::OsStr>;
}
