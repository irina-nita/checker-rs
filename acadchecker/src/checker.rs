#![allow(dead_code)]
#![allow(unused_variables)]
#![allow(unused_assignments)]

pub struct AcadChecker {}

impl AcadChecker {
    pub fn new() -> Self {
        Self {}
    }

    pub fn run<F>(
        &self,
        config: crate::utils::Config,
        runner: F,
    ) -> crate::utils::Output
    where
        F: Fn(
            &Vec<std::ffi::OsString>,
            std::collections::BTreeMap<usize, &std::path::PathBuf>,
        ) -> std::collections::BTreeMap<
            usize,
            Result<std::path::PathBuf, acadcheck::checker::Error>,
        >,
    {
        // Get arguments
        let mut args: Option<Vec<std::ffi::OsString>> = None;

        // Get source.
        let source = config.solution;

        // Get exec path.
        let mut exec_path: Option<std::path::PathBuf> = None;

        let processor: Box<dyn acadcheck::language::LanguageProcessor> = match config.processor {
            crate::utils::SupportedProcessor::Gcc { gcc, flags, exec } => {
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
            crate::utils::SupportedProcessor::Python { python, flags } => {
                // Put the args.
                args = Some(
                    flags
                        .into_iter()
                        .map(|f| (std::ffi::OsString::from(f)))
                        .collect::<Vec<_>>(),
                );

                Box::new(python)
            }
            crate::utils::SupportedProcessor::Makefile { makefile } => Box::new(makefile),
        };

        let solution = acadcheck::solution::Solution::new(processor, source);

        let checker_config = config.checker;

        // Get command from the solution processor.
        let command = match solution.processor.run(args, solution.source, exec_path) {
            Ok(c) => c,
            Err(e) => {
                return crate::utils::Output::Error(e.to_string());
            }
        };

        // Build checker
        let checker = acadcheck::checker::Checker::new(checker_config, runner);

        // Run it and collect the result :).
        let checker_results = checker.run(&command);

        crate::utils::Output::Tests(checker_results)
    }
}
