//! Traits, structs and helpers related to solutions/submissions.

use std::ffi::OsStr;

/// `Submission` is defined by the language processor used and the path to the
/// file or directory that contains it.
pub mod source;

#[derive(Debug)]
pub struct Submission<'b, L, P, S>
where
    L: crate::lang::LanguageProcessor,
    P: AsRef<std::path::Path>,
    S: AsRef<std::ffi::OsStr>,
    [S]: ToOwned<Owned = Vec<S>>
{
    /// Language processor used to run the solution.
    pub processor: L,

    /// Path to the file or directory where the solution is stored.
    pub source: Source<P>,

    /// Build args for compiled solutions.
    compilation_flags: Option<std::borrow::Cow<'b, [S]>>,

    /// Run args
    run_args: Option<std::borrow::Cow<'b, [S]>>,
}

impl<'b, L, P, S> Submission<'b, L, P, S>
where
    L: crate::lang::LanguageProcessor,
    P: AsRef<std::path::Path>,
    S: AsRef<std::ffi::OsStr>,
    [S]: ToOwned<Owned = Vec<S>>
{
    /// Creates a new submission.
    pub fn new(
        processor: L,
        source: Source<P>,
        compilation_flags: Option<std::borrow::Cow<'b, [S]>>,
        run_args: Option<std::borrow::Cow<'b, [S]>>,
    ) -> Self {
        Self {
            processor,
            source,
            compilation_flags,
            run_args,
        }
    }

    /// Runs the submission on a test.
    pub fn run_test(&self) -> Result<(), crate::error::CheckerError> {
        todo!()
    }
}

// Multiple variants for the solution source type regarding the way it is
// structured. Currently supports three types, but is non-exhaustive.
#[cfg_attr(feature = "use-serde", derive(serde::Serialize, serde::Deserialize))]
#[derive(Debug)]
#[non_exhaustive]
pub enum Source<P>
where
    P: AsRef<std::path::Path>,
{
    /// A single source file.
    #[cfg_attr(feature = "use-serde", serde(rename = "file"))]
    File(P),
    /// A directory that contains only the source files.
    #[cfg_attr(feature = "use-serde", serde(rename = "dir"))]
    Directory(P),
}

impl<P> From<Source<P>> for std::ffi::OsString
where
    P: AsRef<std::path::Path>,
{
    fn from(_value: Source<P>) -> Self {
        todo!()
    }
}

impl<P> AsRef<OsStr> for Source<P>
where
    P: AsRef<std::path::Path>,
{
    fn as_ref(&self) -> &OsStr {
        todo!()
    }
}
