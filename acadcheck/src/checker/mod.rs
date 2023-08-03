//! Traits, structs and helpers related to the checker.
#![allow(dead_code)]

pub(crate) mod config;

pub use config::{CheckerConfig, MonitorType, OutputType, PartialEq};
/// Errors that could occur running a test.
#[derive(thiserror::Error, Debug)]
#[non_exhaustive]
pub enum Error {
    #[error("Running solution failed: {0}")]
    RunError(#[from] crate::language::Error),

    #[error("Running test failed: {0}")]
    TestError(String),

    #[error("Comparing output and reference failed with: {0}")]
    CompareError(String),
}

/// Output of a running test.
#[non_exhaustive]
#[derive(Clone, Debug)]
#[cfg_attr(feature = "use-serde", derive(serde::Serialize, serde::Deserialize))]
pub enum Output {
    #[cfg_attr(feature = "use-serde", serde(rename = "passed"))]
    Passed,
    #[cfg_attr(feature = "use-serde", serde(rename = "failed"))]
    Failed(String),
    #[cfg_attr(feature = "use-serde", serde(rename = "score"))]
    Score {
        #[cfg_attr(feature = "use-serde", serde(rename = "points"))]
        score: usize,
        #[cfg_attr(feature = "use-serde", serde(skip_serializing_if = "Option::is_none"))]
        message: Option<String>,
    },
}

/// Checker is defined by a [CheckerConfig](crate::checker::CheckerConfig) and a
/// runner (a closure a closure that defines the way a command should run.
pub struct Checker<I, O, F, T, S, P>
where
    I: std::fmt::Debug + Eq + std::hash::Hash,
    O: std::fmt::Debug + crate::checker::PartialEq<O> + std::cmp::PartialEq,
    F: Fn(
        &T,
        std::collections::BTreeMap<usize, &I>,
    ) -> std::collections::BTreeMap<usize, Result<O, crate::checker::Error>>,
    T: IntoIterator<Item = S>,
    S: AsRef<std::ffi::OsStr>,
    P: IntoIterator<Item = crate::checker::config::MonitorType>,
{
    config: crate::checker::config::CheckerConfig<I, O, P>,
    runner: F,
    _phantom_t: std::marker::PhantomData<T>,
}

impl<I, O, F, T, S, P> Checker<I, O, F, T, S, P>
where
    I: std::fmt::Debug + Eq + std::hash::Hash,
    O: std::fmt::Debug + crate::checker::PartialEq<O> + std::cmp::PartialEq,
    F: Fn(
        &T,
        std::collections::BTreeMap<usize, &I>,
    ) -> std::collections::BTreeMap<usize, Result<O, crate::checker::Error>>,
    T: IntoIterator<Item = S>,
    S: AsRef<std::ffi::OsStr>,
    P: IntoIterator<Item = crate::checker::config::MonitorType>,
{
    /// Gets configuration as helper for building the checker.
    /// The runner Fn is a closure that defines the way a command should be ran
    /// with the inputs given.
    pub fn new(config: crate::checker::config::CheckerConfig<I, O, P>, runner: F) -> Self {
        Self {
            config,
            runner,
            _phantom_t: std::marker::PhantomData::default(),
        }
    }

    /// Runs the checker for a command given.
    pub fn run(self, command: &T) -> std::collections::BTreeMap<usize, crate::checker::Output> {
        let keys = self
            .config
            .in_refs
            .iter()
            .map(|m| (*(m.0), &m.1.0))
            .collect::<std::collections::BTreeMap<_, _>>();

        let outputs = (self.runner)(&command, keys);

        let mut scored: bool = false;
        let mut score_per_test: usize = 0;
        if let crate::checker::config::OutputType::Scored { per_test } = self.config.output_type {
            scored = true;
            score_per_test = per_test;
        }

        let results: std::collections::BTreeMap<_, _> = outputs
            .into_iter()
            .map(|m| match m.1 {
                Ok(output) => {
                    let mut output_inner = String::new();
                    let mut ref_inner = String::new();

                    if output.ceq(
                        &self.config.in_refs.get(&m.0).unwrap().1,
                        &mut output_inner,
                        &mut ref_inner,
                    ) {
                        return if scored {
                            (
                                m.0,
                                crate::checker::Output::Score {
                                    score: score_per_test,
                                    message: None,
                                },
                            )
                        } else {
                            (m.0, crate::checker::Output::Passed)
                        };
                    } else {
                        return if scored {
                            (
                                m.0,
                                crate::checker::Output::Score {
                                    score: 0,
                                    message: Some(format!(
                                        "Expected: {}\nBut got: {}\n",
                                        ref_inner, output_inner
                                    )),
                                },
                            )
                        } else {
                            (
                                m.0,
                                crate::checker::Output::Failed(format!(
                                    "Expected: {}\nBut got: {}\n",
                                    ref_inner, output_inner
                                )),
                            )
                        };
                    }
                }
                Err(e) => {
                    return if scored {
                        (
                            m.0,
                            crate::checker::Output::Score {
                                score: 0,
                                message: Some(format!("{}", e.to_string())),
                            },
                        )
                    } else {
                        (
                            m.0,
                            crate::checker::Output::Failed(format!("{}", e.to_string())),
                        )
                    };
                }
            })
            .collect();

        results
    }

    /// Runs the checker for a command given and consumes the checker.
    pub fn run_once(
        self,
        command: &T,
    ) -> std::collections::BTreeMap<usize, crate::checker::Output> {
        self.run(command)
    }
}
