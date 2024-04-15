/// The checker configuration is based on the monitors used for the running
/// processes, the output type of the tests.
///
/// The differ is not included in the configuration, because some batch of tests
/// could be run with less constraints than others.
#[derive(Debug)]
#[cfg_attr(feature = "use-serde", derive(serde::Serialize, serde::Deserialize))]
pub struct Configuration<T>
where
    T: IntoIterator<Item = MonitorType>,
{
    pub monitors: Option<T>,
    pub output_type: TestOutputType,
}

/// Types of monitors that could be used to analyze the solution running.
#[non_exhaustive]
#[derive(Debug)]
#[cfg_attr(feature = "use-serde", derive(serde::Serialize, serde::Deserialize))]
pub enum MonitorType {
    /// Stops the execution of the solution on limit reached.
    #[cfg_attr(feature = "use-serde", serde(rename = "time"))]
    Timeout { limit: std::time::Duration },
    #[cfg_attr(feature = "use-serde", serde(rename = "time.footprint"))]
    TimeFootprint,
}

/// Output types for the test.
#[non_exhaustive]
#[derive(Debug)]
#[cfg_attr(feature = "use-serde", derive(serde::Serialize, serde::Deserialize))]
pub enum TestOutputType {
    #[cfg_attr(feature = "use-serde", serde(rename = "scored"))]
    Scored { per_test: usize },
    #[cfg_attr(feature = "use-serde", serde(rename = "binary"))]
    Binary,
}

/// Output of a running test.
#[non_exhaustive]
#[derive(Clone, Debug)]
#[cfg_attr(feature = "use-serde", derive(serde::Serialize, serde::Deserialize))]
pub enum TestOutput {
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
