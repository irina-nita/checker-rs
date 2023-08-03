#[derive(Debug)]
#[cfg_attr(feature = "use-serde", derive(serde::Serialize, serde::Deserialize))]
pub struct Makefile {
    path: std::path::PathBuf,
    build: Option<std::ffi::OsString>,
    run: std::ffi::OsString,
}

impl crate::language::LanguageProcessor for Makefile {
    fn run(
        &self,
        _args: Option<Vec<std::ffi::OsString>>,
        _source: crate::solution::Source,
        _exec: Option<std::path::PathBuf>,
    ) -> Result<Vec<std::ffi::OsString>, crate::language::Error> {
        todo!()
    }
}

impl crate::language::Makefile for Makefile {
    fn run<S>(&self, _target: Option<S>) -> Result<Vec<std::ffi::OsString>, crate::language::Error>
    where
        S: AsRef<std::ffi::OsStr>,
    {
        todo!()
    }
}
