//! Defines the configuration of a checker.

use std::io::{Read, Seek};
#[derive(std::fmt::Debug)]
#[cfg_attr(feature = "use-serde", derive(serde::Serialize, serde::Deserialize))]
pub struct CheckerConfig<I, O, T>
where
    I: std::fmt::Debug + Eq + std::hash::Hash,
    O: std::fmt::Debug + crate::checker::config::PartialEq<O> + std::cmp::PartialEq,
    T: IntoIterator<Item = MonitorType>,
{
    pub monitors: T,
    pub output_type: OutputType,
    pub in_refs: std::collections::BTreeMap<usize, (I, O)>,
}

impl<I, O, T> CheckerConfig<I, O, T>
where
    I: std::fmt::Debug + Eq + std::hash::Hash,
    O: std::fmt::Debug + crate::checker::config::PartialEq<O> + std::cmp::PartialEq,
    T: IntoIterator<Item = MonitorType>,
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
    /// Stops the execution of the solution on limit reached.
    #[cfg_attr(feature = "use-serde", serde(rename = "time"))]
    Timeout { limit: std::time::Duration },
    #[cfg_attr(feature = "use-serde", serde(rename = "memory.footprint"))]
    TimeFootprint,
}

/// Output types
#[non_exhaustive]
#[derive(std::fmt::Debug)]
#[cfg_attr(feature = "use-serde", derive(serde::Serialize, serde::Deserialize))]
pub enum OutputType {
    /// Monitor as a command for which the argument will be the executable.
    #[cfg_attr(feature = "use-serde", serde(rename = "scored"))]
    Scored { per_test: usize },
    #[cfg_attr(feature = "use-serde", serde(rename = "none"))]
    None,
}

/// Trait for equality comparisons.
/// x.ceq(y) can __not__ be written x == y.
pub trait PartialEq<Rhs = Self>
where
    Rhs: Sized,
{
    /// Equality comparison between self and other.
    /// self_inner and other_inner should be the string representations of the
    /// values that are being compared, needed for test output messages.
    fn ceq(&self, other: &Rhs, self_inner: &mut String, other_inner: &mut String) -> bool;
}

impl PartialEq<std::path::PathBuf> for std::path::PathBuf {
    fn ceq(
        &self,
        other: &std::path::PathBuf,
        self_inner: &mut String,
        other_inner: &mut String,
    ) -> bool {
        let mut f = match std::fs::File::open(self) {
            Ok(o) => o,
            Err(_) => {
                return false;
            }
        };

        let mut g = match std::fs::File::open(other) {
            Ok(o) => o,
            Err(_) => {
                return false;
            }
        };

        let val = file_diff::diff_files(&mut f, &mut g);

        f.rewind().unwrap();
        g.rewind().unwrap();

        match g.read_to_string(other_inner) {
            Ok(_) => {}
            Err(_) => {
                return false;
            }
        }

        match f.read_to_string(self_inner) {
            Ok(_) => {}
            Err(_) => {
                return false;
            }
        }

        val
    }
}
