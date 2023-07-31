//! Defining tests.
#![allow(unused_variables)]

/// Errors that could occur for running the submission on a (sub)test. (Touple
/// of `in, ref` ). These are handled by `run` to avoid panic.
#[derive(thiserror::Error, Debug)]
#[non_exhaustive]
#[cfg_attr(feature = "use-serde", derive(serde::Serialize, serde::Deserialize))]
pub enum Error {
    #[error("Running solution failed: {0}")]
    RunError(#[from] crate::language::Error),

    #[error("Running test failed: {0}")]
    TestError(String),

    #[error("Unsupported output. Expected {0}, but found: {0}")]
    UnsupportedOutputError(String, String),

    #[error("Comparing output and reference failed with: {0}")]
    CompareError(String),
}

/// Enum for output of a running (sub)test.
#[non_exhaustive]
#[derive(Clone, Debug)]
#[cfg_attr(feature = "use-serde", derive(serde::Serialize, serde::Deserialize))]
pub enum Output {
    Passed,
    Failed(String),
    Score(usize),
}
