//! Implementation of a checker that is based on diff-ing the contestants'
//! outputs and the expected outputs. This module was written for the [acadchecker](https://crates.io/crates/acadchecker) CLI tool.
#![cfg(feature = "use-serde")]
#![cfg_attr(docsrs, doc(cfg(feature = "use-serde")))]

pub mod config;

pub struct AcadChecker {}

impl AcadChecker {
    pub fn new() -> Self {
        Self {}
    }

    /// Run the checker with a given configuration and a runner.
    pub fn run<F>(
        &self,
        config: crate::acadchecker::config::Config,
        runner: F,
    ) -> crate::acadchecker::config::Output
    where
        F: Fn(
            &Vec<std::ffi::OsString>,
            std::collections::BTreeMap<usize, &std::path::PathBuf>,
        ) -> std::collections::BTreeMap<
            usize,
            Result<std::path::PathBuf, crate::checker::Error>,
        >,
    {
        // Get arguments
        let mut args: Option<Vec<std::ffi::OsString>> = None;

        // Get source.
        let source = config.solution;

        // Get exec path.
        let mut exec_path: Option<std::path::PathBuf> = None;

        let processor: Box<dyn crate::language::LanguageProcessor> = match config.processor {
            crate::acadchecker::config::SupportedProcessor::Gcc { gcc, flags, exec } => {
                // Put the args.
                args = Some(
                    flags
                        .into_iter()
                        .map(|f| (std::ffi::OsString::from(f)))
                        .collect::<Vec<_>>(),
                );

                // Put the exec path.
                exec_path = Some(exec);

                Box::new(gcc)
            }
            crate::acadchecker::config::SupportedProcessor::Python { python, flags } => {
                // Put the args.
                args = Some(
                    flags
                        .into_iter()
                        .map(|f| (std::ffi::OsString::from(f)))
                        .collect::<Vec<_>>(),
                );

                Box::new(python)
            }
            crate::acadchecker::config::SupportedProcessor::Makefile { makefile } => {
                Box::new(makefile)
            }
        };

        let solution = crate::solution::Solution::new(processor, source);

        let checker_config = config.checker;

        // Get command from the solution processor.
        let command = match solution.processor.run(args, solution.source, exec_path) {
            Ok(c) => c,
            Err(e) => {
                return crate::acadchecker::config::Output::Error(e.to_string());
            }
        };

        // Build checker
        let checker = crate::checker::Checker::new(checker_config, runner);

        // Run it and collect the result :).
        let checker_results = checker.run(&command);

        crate::acadchecker::config::Output::Tests(checker_results)
    }
}
