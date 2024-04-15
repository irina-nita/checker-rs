/// Output for diff-ing two files.
pub enum DiffOutput {
    /// The files are identical.
    IdenticalContent,
    /// The inner vector contains the lines that are different.
    DifferentContent(Vec<usize>),
}

/// Trait for the differ that is used by checker.
pub trait Differ {
    fn diff<P>(&self, output: &P, reference: &P) -> Result<DiffOutput, std::io::Error>
    where
        P: AsRef<std::path::Path>;
}
