/// Generate the tests Map needed by the checker to run.
///
/// The paths are paired and numbered in the map based on __alphabetical
/// order__.
pub fn generate_tests<S>(
    input_dir: S,
    reference_dir: S,
) -> Result<
    std::collections::BTreeMap<usize, (std::path::PathBuf, std::path::PathBuf)>,
    std::io::Error,
>
where
    S: AsRef<std::ffi::OsStr>,
{
    let mut tests: std::collections::BTreeMap<usize, (std::path::PathBuf, std::path::PathBuf)> =
        std::collections::BTreeMap::new();
    let input_files = sort_files(input_dir.as_ref())?;
    let reference_files = sort_files(reference_dir.as_ref())?;

    for (i, (input, reference)) in input_files
        .into_iter()
        .zip(reference_files.into_iter())
        .enumerate()
    {
        tests.insert(i, (input, reference));
    }

    Ok(tests)
}

/// Sort the files in a directory by their name in alphabetical order.
fn sort_files<P>(directory_path: P) -> Result<Vec<std::path::PathBuf>, std::io::Error>
where
    P: AsRef<std::path::Path>,
{
    let directory = std::fs::read_dir(directory_path)?;
    let mut files: Vec<std::path::PathBuf> = vec![];
    for file in directory {
        let f = file?;
        if f.path().file_name().is_some() && f.file_type()?.is_file() {
            files.push(f.path());
        }
    }
    files.sort_by(|first, second| {
        if let (Some(first_filename), Some(second_filename)) =
            (first.file_name(), second.file_name())
        {
            first_filename
                .to_ascii_lowercase()
                .cmp(&second_filename.to_ascii_lowercase())
        } else {
            std::cmp::Ordering::Equal
        }
    });
    Ok(files)
}

#[cfg(test)]
mod tests {
    #[test]
    fn test_gen() {
        let tests = super::generate_tests("./in", "./ref").unwrap();
        assert_eq!(tests.len(), 2);
        assert_eq!(
            tests.get(&0).unwrap(),
            &(
                std::path::PathBuf::from("./in/test0.in"),
                std::path::PathBuf::from("./ref/test0.ref")
            )
        );
    }
}
