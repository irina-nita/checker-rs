//! Traits, structs and helpers related to solutions/submissions.

/// `Solution` is defined by the language processor used and the path to the
/// file or directory that contains it.
#[derive(Debug)]
pub struct Solution<L>
where
    L: crate::language::LanguageProcessor,
{
    /// Language processor used to run the solution.
    pub processor: L,

    /// Path to the file or directory where the solution is stored.
    pub source: Source,
}

impl<L> Solution<L>
where
    L: crate::language::LanguageProcessor,
{
    pub fn new(processor: L, source: Source) -> Self {
        Self { processor, source }
    }
}

/// Multiple variants for the solution source type regarding the way it is
/// structured. Currently supports three types, but is non-exhaustive.
#[cfg_attr(feature = "use-serde", derive(serde::Serialize, serde::Deserialize))]
#[derive(Debug)]
#[non_exhaustive]
pub enum Source {
    /// A single source file.
    File(std::path::PathBuf),
    /// A directory that contains only the source files.
    Directory(std::path::PathBuf),
    /// A directory that may contain other files we want to omit.
    DirectoryRegex {
        dir: std::path::PathBuf,
        regex: String,
    },
    None,
}
