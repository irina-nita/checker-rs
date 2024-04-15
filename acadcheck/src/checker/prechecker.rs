//! The prechecker is meant to be used to test the source code of a submission
//! before running the checker.

pub trait Prechecker {
    type PrecheckOutput;
    // fn precheck<L, P>(
    //     &self,
    //     submission: crate::submission::Submission<L, P>,
    // ) -> Result<Self::PrecheckOutput, crate::error::CheckerError>
    // where
    //     L: crate::lang::LanguageProcessor,
    //     P: AsRef<std::path::Path>;
}
