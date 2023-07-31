#[derive(std::fmt::Debug)]
#[cfg_attr(feature = "use-serde", derive(serde::Serialize, serde::Deserialize))]
pub struct CheckerConfig<I, O>
where
    I: std::fmt::Debug + Eq + std::hash::Hash,
    O: std::fmt::Debug + PartialEq<O>,
{
    pub monitor_type: MonitorType,
    pub output_type: OutputType,
    pub in_refs: std::collections::HashMap<I, O>,
}

impl<I, O> CheckerConfig<I, O>
where
    I: std::fmt::Debug + Eq + std::hash::Hash,
    O: std::fmt::Debug + PartialEq<O>,
{
    #[cfg(feature = "use-serde")]
    pub(crate) fn from_json_file<P>(_path: P) -> Result<Self, anyhow::Error>
    where
        P: AsRef<std::path::Path>,
    {
        todo!()
    }
}

/// Monitor enums for different types of monitors that could be used to analyze
/// the solution running.
#[non_exhaustive]
#[derive(std::fmt::Debug)]
#[cfg_attr(feature = "use-serde", derive(serde::Serialize, serde::Deserialize))]
pub enum MonitorType {
    /// Monitor as a command for which the argument will be the executable.
    #[cfg_attr(feature = "use-serde", serde(rename = "time"))]
    Time {
        limit: std::time::Duration,
    },
    None,
}

/// Output types
#[non_exhaustive]
#[derive(std::fmt::Debug)]
#[cfg_attr(feature = "use-serde", derive(serde::Serialize, serde::Deserialize))]
pub enum OutputType {
    /// Monitor as a command for which the argument will be the executable.
    #[cfg_attr(feature = "use-serde", serde(rename = "scored"))]
    Scored {
        per_test: usize,
    },
    None,
}

pub trait OutputPartialEq<Rhs = Self>
where
    Rhs: Sized,
{
    fn ceq(&self, other: &Rhs) -> bool;
}
