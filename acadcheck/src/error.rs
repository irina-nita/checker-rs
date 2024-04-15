/// Stages at which the checker could fail.s
#[derive(Debug)]
pub enum CheckerStage {
    Precheck,
    Build,
    Execution,
    Diff,
}

impl std::fmt::Display for CheckerStage {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(
            f,
            "{}",
            match self {
                CheckerStage::Build => "Build",
                CheckerStage::Execution => "Execution",
                CheckerStage::Diff => "Diff",
                CheckerStage::Precheck => "Precheck",
            }
        )
    }
}

/// Errors that could occur during the different stages of the checker.
#[derive(Debug)]
pub struct CheckerError {
    stage: CheckerStage,
    source: Option<std::io::Error>,
    message: Option<String>,
}

impl CheckerError {
    pub fn new(stage: CheckerStage) -> Self {
        Self {
            stage,
            source: None,
            message: None,
        }
    }

    pub fn with_source(mut self, source: std::io::Error) -> Self {
        // If the error has a message, the source should not be set.
        if self.message.is_none() {
            self.source = Some(source);
        }
        self
    }

    pub fn with_message<S>(mut self, message: S) -> Self
    where
        S: ToString,
    {
        // If the error has a source, the message should not be set.
        if self.source.is_none() {
            self.message = Some(message.to_string());
        }
        self
    }
}

impl std::fmt::Display for CheckerError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match (&self.source, &self.message) {
            (Some(source), _) => write!(f, "{} failed from: {}", self.stage, source),
            (_, Some(message)) => write!(f, "{} failed: {}", self.stage, message),
            (_, _) => write!(f, "{} failed", self.stage),
        }
    }
}

impl std::error::Error for CheckerError {
    fn source(&self) -> Option<&(dyn std::error::Error + 'static)> {
        if let Some(ref s) = self.source {
            Some(s)
        } else {
            None
        }
    }

    fn cause(&self) -> Option<&dyn std::error::Error> {
        self.source()
    }
}

/// Creates a [`CheckerError`](crate::error::CheckerError) from the diff stage.
///
/// - Create an error with no message or source:
/// ```
/// use acadcheck::error::diff_error;
/// let err: acadcheck::error::CheckerError = diff_error!();
/// ```
/// - Create an error with a message:
/// ```
/// use acadcheck::error::diff_error;
/// let err: acadcheck::error::CheckerError = diff_error!(message: "example");
/// ```
/// - Create an error with a source:
/// ```
/// use acadcheck::error::diff_error;
/// let source_err: std::io::Error = std::io::Error::new(std::io::ErrorKind::Other, "example");
/// let err: acadcheck::error::CheckerError = diff_error!(source: source);
/// ```
#[macro_export]
macro_rules! diff_error {
    () => {
        CheckerError::new($crate::error::CheckerStage::Diff)
    };

    (message: $message:expr) => {
        CheckerError::new($crate::error::CheckerStage::Diff).with_message($message)
    };

    (source: $source:expr) => {
        CheckerError::new($crate::error::CheckerStage::Diff).with_source($source)
    };
}

/// Creates a [`CheckerError`](crate::error::CheckerError) from the build stage.
///
/// - Create an error with no message or source:
/// ```
/// use acadcheck::error::build_error;
/// let err: acadcheck::error::CheckerError = build_error!();
/// ```
/// - Create an error with a message:
/// ```
/// use acadcheck::error::build_error;
/// let err: acadcheck::error::CheckerError = build_error!(message: "example");
/// ```
/// - Create an error with a source:
/// ```
/// use acadcheck::error::build_error;
/// let source_err: std::io::Error = std::io::Error::new(std::io::ErrorKind::Other, "example");
/// let err: acadcheck::error::CheckerError = build_error!(source: source);
/// ```
#[macro_export]
macro_rules! build_error {
    () => {
        $crate::error::CheckerError::new($crate::error::CheckerStage::Build)
    };

    (message: $message:literal) => {
        $crate::error::CheckerError::new($crate::error::CheckerStage::Build).with_message($message)
    };

    (source: $source:expr) => {
        $crate::error::CheckerError::new($crate::error::CheckerStage::Build).with_source($source)
    };
}

/// Creates a [`CheckerError`](crate::error::CheckerError) from the submission
/// execution stage.
///
/// - Create an error with no message or source:
/// ```
/// use acadcheck::error::execution_error;
/// let err: acadcheck::error::CheckerError = execution_error!();
/// ```
/// - Create an error with a message:
/// ```
/// use acadcheck::error::execution_error;
/// let err: acadcheck::error::CheckerError = execution_error!(message: "example");
/// ```
/// - Create an error with a source:
/// ```
/// use acadcheck::error::execution_error;
/// let source_err: std::io::Error = std::io::Error::new(std::io::ErrorKind::Other, "example");
/// let err: acadcheck::error::CheckerError = execution_error!(source: source);
/// ```
#[macro_export]
macro_rules! execution_error {
    () => {
        $crate::error::CheckerError::new($crate::error::CheckerStage::Execution)
    };

    (message: $message:expr) => {
        $crate::error::CheckerError::new($crate::error::CheckerStage::Execution)
            .with_message($message)
    };

    (source: $source:expr) => {
        $crate::error::CheckerError::new($crate::error::CheckerStage::Execution)
            .with_source($source)
    };
}

/// Creates a [`CheckerError`](crate::error::CheckerError) from the prechecker
/// stage.
///
/// - Create an error with no message or source:
/// ```
/// use acadcheck::error::precheck_error;
/// let err: acadcheck::error::CheckerError = precheck_error!();
/// ```
/// - Create an error with a message:
/// ```
/// use acadcheck::error::precheck_error;
/// let err: acadcheck::error::CheckerError = precheck_error!(message: "example");
/// ```
/// - Create an error with a source:
/// ```
/// use acadcheck::error::precheck_error;
/// let source_err: std::io::Error = std::io::Error::new(std::io::ErrorKind::Other, "example");
/// let err: acadcheck::error::CheckerError = precheck_error!(source: source);
/// ```
#[macro_export]
macro_rules! precheck_error {
    () => {
        $crate::error::CheckerError::new($crate::error::CheckerStage::Precheck)
    };

    (message: $message:literal) => {
        $crate::error::CheckerError::new($crate::error::CheckerStage::Precheck)
            .with_message($message)
    };

    (source: $source:expr) => {
        $crate::error::CheckerError::new($crate::error::CheckerStage::Precheck).with_source($source)
    };
}

pub use build_error;
pub use diff_error;
pub use execution_error;
pub use precheck_error;

#[cfg(test)]
mod error_tests {
    use super::CheckerError;
    use super::CheckerStage;
    #[test]
    fn test_staged_error() -> Result<(), CheckerError> {
        // Force a std::io::Error
        let file_path = "nonexistent_file.txt";
        let file_result = std::fs::File::open(file_path);
        if let Err(e) = file_result {
            let err = CheckerError::new(CheckerStage::Build).with_source(e);
            eprintln!("{}", err);
            return Err(err);
        }
        Ok(())
    }
}
