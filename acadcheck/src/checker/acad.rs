pub struct Checker<I, O, F, T, S>
where
    I: std::fmt::Debug + Eq + std::hash::Hash,
    O: std::fmt::Debug + crate::checker::config::OutputPartialEq<O> + std::cmp::PartialEq,
    F: Fn(&T, Vec<&I>) -> std::collections::HashMap<I, Result<O, crate::checker::tests::Error>>,
    T: IntoIterator<Item = S>,
    S: AsRef<std::ffi::OsStr>,
{
    config: crate::checker::config::CheckerConfig<I, O>,
    runner: F,
    _phantom_t: std::marker::PhantomData<T>,
}

impl<I, O, F, T, S> Checker<I, O, F, T, S>
where
    I: std::fmt::Debug + Eq + std::hash::Hash,
    O: std::fmt::Debug + crate::checker::config::OutputPartialEq<O> + std::cmp::PartialEq,
    F: Fn(&T, Vec<&I>) -> std::collections::HashMap<I, Result<O, crate::checker::tests::Error>>,
    T: IntoIterator<Item = S>,
    S: AsRef<std::ffi::OsStr>,
{
    /// Gets configuration as helper for building the checker.
    pub fn new(config: crate::checker::config::CheckerConfig<I, O>, runner: F) -> Self {
        Self {
            config,
            runner,
            _phantom_t: std::marker::PhantomData::default(),
        }
    }

    /// Runs the checker against an executable based on the "runner" provided.
    pub fn run(self, command: &T) -> Vec<crate::checker::tests::Output> {
        let keys = self.config.in_refs.iter().map(|f| f.0).collect::<Vec<_>>();

        let outputs = (self.runner)(&command, keys);

        let mut scored: bool = false;
        let mut score_per_test: usize = 0;
        if let crate::checker::config::OutputType::Scored { per_test } = self.config.output_type {
            scored = true;
            score_per_test = per_test;
        }

        let results: Vec<_> = outputs
            .into_iter()
            .map(|m| match m.1 {
                Ok(output) => {
                    if output.ceq(self.config.in_refs.get(&m.0).unwrap()) {
                        return if scored {
                            crate::checker::tests::Output::Score(score_per_test)
                        } else {
                            crate::checker::tests::Output::Passed
                        };
                    } else {
                        return if scored {
                            crate::checker::tests::Output::Score(0)
                        } else {
                            crate::checker::tests::Output::Failed("Wrong output".to_string())
                        };
                    }
                }
                Err(e) => {
                    return crate::checker::tests::Output::Failed(format!(
                        "Execution failed: {}",
                        e.to_string()
                    ));
                }
            })
            .collect();

        results
    }
}
