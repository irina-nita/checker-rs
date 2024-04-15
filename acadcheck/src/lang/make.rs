//! Generic Makefile implementation of the Make trait.

#[derive(Debug, Clone)]
#[cfg_attr(feature = "use-serde", derive(serde::Serialize, serde::Deserialize))]
pub struct Makefile {
    /// The directory where the makefile is located.
    path: std::path::PathBuf,
    /// The target name for the build rule.
    build: Option<std::ffi::OsString>,
    /// The tarhet name for the run rule.
    run: std::ffi::OsString,
}

impl crate::lang::LanguageProcessor for Makefile {
    fn run<S, P>(
        &self,
        _args: Option<Vec<S>>,
        _source: crate::submission::Source<P>,
        _exec: Option<P>,
    ) -> Result<Vec<std::ffi::OsString>, crate::error::CheckerError>
    where
        S: AsRef<std::ffi::OsStr>,
        P: AsRef<std::path::Path>,
    {
        todo!()
    }
}

impl crate::lang::Make for Makefile {
    fn run<S>(
        &self,
        _target: Option<S>,
    ) -> Result<Vec<std::ffi::OsString>, crate::error::CheckerError>
    where
        S: AsRef<std::ffi::OsStr>,
    {
        todo!()
    }
}
