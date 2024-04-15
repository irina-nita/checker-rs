//! Generic checker and its components.

pub(crate) mod configuration;
pub(crate) mod diff;
pub(crate) mod prechecker;

pub use configuration::Configuration;

use crate::submission;

/// Checker used for running tests on a submission.
/// 
/// The checker is composed of a configuration, a differ and a prechecker.
pub struct Checker<M, D, C>
where
    M: IntoIterator<Item = crate::checker::configuration::MonitorType>,
    D: crate::checker::diff::Differ,
    C: crate::checker::prechecker::Prechecker,
{
    config: crate::checker::configuration::Configuration<M>,
    differ: D,
    prechecker: Option<C>,
}

impl<M, D, C> Checker<M, D, C>
where
    M: IntoIterator<Item = crate::checker::configuration::MonitorType>,
    D: crate::checker::diff::Differ,
    C: crate::checker::prechecker::Prechecker,
{
    pub fn new(
        config: crate::checker::configuration::Configuration<M>,
        differ: D,
        prechecker: Option<C>,
    ) -> Self {
        Self {
            config,
            differ,
            prechecker,
        }
    }

    /// Runs the checker for a set of tests.
    pub fn run<P>(
        &self,
        _tests: std::collections::BTreeMap<usize, (P, P)>,
        // submission: submission::Submission,
    ) -> std::collections::BTreeMap<usize, crate::checker::configuration::TestOutput>
    where
        P: AsRef<std::path::Path>,
    {
        todo!()
    }
}
