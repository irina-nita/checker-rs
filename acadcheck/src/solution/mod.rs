//! Traits, structs and helpers related to solutions/submissions.

/// `Solution` is defined by the language processor used and the path to the
/// file or directory that contains it.
#[derive(Debug)]
pub struct Solution {
    /// Language processor used to run the solution.
    pub processor: Box<dyn crate::language::LanguageProcessor>,

    /// Path to the file or directory where the solution is stored.
    pub source: Source,
}

impl Solution {
    pub fn new(processor: Box<dyn crate::language::LanguageProcessor>, source: Source) -> Self {
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
    #[cfg_attr(feature = "use-serde", serde(rename = "file"))]
    File(std::path::PathBuf),
    /// A directory that contains only the source files.
    #[cfg_attr(feature = "use-serde", serde(rename = "dir"))]
    Directory(std::path::PathBuf),
    /// A directory that may contain other files we want to omit.
    #[cfg_attr(feature = "use-serde", serde(rename = "regex"))]
    Regex { regex: String },
}
