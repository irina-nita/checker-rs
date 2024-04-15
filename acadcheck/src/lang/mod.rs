//! Traits, structs and helpers related for defining language processors.

pub mod gcc;
pub mod make;
pub mod python;

/// Trait for language processors such as compilers or interpreters.
pub trait LanguageProcessor: std::fmt::Debug {
    fn run<S, P>(
        &self,
        args: Option<Vec<S>>,
        source: crate::submission::Source<P>,
        exec: Option<P>,
    ) -> Result<Vec<std::ffi::OsString>, crate::error::CheckerError>
    where
        S: AsRef<std::ffi::OsStr>,
        P: AsRef<std::path::Path>;
}

/// Compiler trait for Language Processors.
pub trait Compiler: LanguageProcessor {
    fn run_compiled<S, I, P>(
        &self,
        flags: Option<I>,
        source: &crate::submission::Source<P>,
        exec: P,
    ) -> Result<Vec<std::ffi::OsString>, crate::error::CheckerError>
    where
        S: AsRef<std::ffi::OsStr> + Sized,
        I: IntoIterator<Item = S>,
        P: AsRef<std::path::Path>;
}

/// Interpreter trait for Language Processors.
pub trait Interpreter: LanguageProcessor {
    /// Returns the command for running the executable along with the
    /// interpreter..
    fn run_interpreted<S, I, P>(
        &self,
        flags: Option<I>,
        source: crate::submission::Source<P>,
    ) -> Vec<std::ffi::OsString>
    where
        S: AsRef<std::ffi::OsStr>,
        I: IntoIterator<Item = S>,
        P: AsRef<std::path::Path>;
}

/// Make rules could be considered as a form of "language processors". It
/// depends on the use case. Each makefile should have at least a rule for
/// "run".
pub trait Make: LanguageProcessor {
    /// Returns the command that acts as a rule for run.
    /// If a build was required before-hand, the implementation should propagate
    /// the error as [`CheckerError`](crate::error::CheckerError).
    fn run<S>(
        &self,
        target: Option<S>,
    ) -> Result<Vec<std::ffi::OsString>, crate::error::CheckerError>
    where
        S: AsRef<std::ffi::OsStr>;
}
