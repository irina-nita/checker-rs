//! Acadnet specific implementation of the checker.

use crate::checker::{configuration::MonitorType, diff::Differ, Checker};

/// Acadnet differ used to compare the output of the solution with the
/// reference.
pub struct AcadnetDiffer {
    /// The precision for tests that require a float comparison.
    float_precision: Option<usize>,
}

impl Differ for AcadnetDiffer {
    fn diff<P>(
        &self,
        _output: &P,
        _reference: &P,
    ) -> Result<crate::checker::diff::DiffOutput, std::io::Error>
    where
        P: AsRef<std::path::Path>,
    {
        todo!()
    }
}

// pub type AcadChecker = Checker<Vec<MonitorType>, AcadnetDiffer>;
