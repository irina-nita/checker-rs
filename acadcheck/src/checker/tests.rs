//! Defining tests.
#![allow(unused_variables)]

use std::ffi::OsString;

/// A test acts as a set of subtests that can be grouped by expectations of
/// results and their format.
pub struct Test<I, O, C, F>
where
    C: Fn(O, O) -> Result<self::Output, self::Error>,
    F: Fn(OsString, I) -> Result<O, self::Error>,
    I: AsRef<std::ffi::OsStr> + std::marker::Sync + Clone,
    O: AsRef<std::ffi::OsStr> + std::marker::Sync + Clone,
{
    /// Closure that specifies the result depending on actual and wanted output.
    pub compare: C,

    /// Closure that runs a command against the input and gives the necessary
    /// output.
    pub runner: F,

    /// Vec of (input, output) tuples that act as (sub)tests to run the solution
    /// on.
    pub in_refs: Vec<(I, O)>,
}

impl<I, O, C, F> Test<I, O, C, F>
where
    C: Fn(O, O) -> Result<self::Output, self::Error>,
    F: Fn(OsString, I) -> Result<O, self::Error>,
    I: AsRef<std::ffi::OsStr> + std::marker::Sync + Clone,
    O: AsRef<std::ffi::OsStr> + std::marker::Sync + Clone,
{
    /// Creates a new Test struct. Tests can't have default values, for obvious
    /// reasons.
    pub fn new(runner: F, compare: C, in_refs: Vec<(I, O)>) -> Self {
        Self {
            compare,
            runner,
            in_refs,
        }
    }

    /// Runs the Test against an executable and provides the output mapped for
    /// each (sub)test. This output can be serialized if the `use-serde`
    /// feature is enabled.
    pub fn run(&self, cmd: OsString) -> Vec<self::Output> {
        // Copy of self.in_refs.
        let in_refs = self.in_refs.clone();

        // Get all join handles.
        let join_handles: std::collections::HashMap<_, _> = in_refs
            .into_iter()
            .enumerate()
            .map( |e| (e.0, (self.runner)(cmd.clone(), e.1.0)))
            .collect();

        // Join threads and get outputs.
        let outputs: std::collections::HashMap<_, _> = join_handles
            .into_iter()
            .map(|j| {
                let res = match j.1 {
                    Ok(o) => { Ok(o)
                    },
                    Err(e) => Err(self::Error::TestError(format!(
                        "Error while joining threads"
                    ))),
                };
                (j.0, res)
            })
            .collect();

        // Run tests on outputs received.
        outputs
            .into_iter()
            .map(|o|  { match o.1 {
                Ok(out) => {
                    let idx = o.0;
                    let res = (self.compare)(out.clone(), self.in_refs[idx].1.clone());
                    match res {
                        Ok(r) => r,
                        Err(e) => self::Output::Failed(e.to_string()),
                    }
                }
                Err(e) => self::Output::Failed(e.to_string()),
            } })
            .collect()
    }
}

/// Errors that could occur for running the submission on a (sub)test. (Touple
/// of `in, ref` ). These are handled by `run` to avoid panic.
#[derive(thiserror::Error, Debug)]
#[non_exhaustive]
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
#[derive(Clone)]
pub enum Output {
    Passed,
    Failed(String),
    Score(u32),
}

/// Monitor enums for different types of monitors that could be used to analyze
/// the executable in different scenarios.
#[non_exhaustive]
pub(crate) enum Monitor {
    /// Monitor as a command for which the argument will be the executable.
    Command(std::process::Command),
    /// Monitor that is integrated in the executable and acts as a flag for it.
    /// Will be parsed as an argument.
    Flag(String),
    None,
}
