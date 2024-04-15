//! Test the error macros.
use acadcheck::error::build_error;

pub fn message_error() -> Result<(), acadcheck::error::CheckerError> {
    Err(build_error!(message: "test"))
}

pub fn source_error() -> Result<(), acadcheck::error::CheckerError> {
    let file_path = "nonexistent_file.txt";
    let file_result = std::fs::File::open(file_path);
    if let Err(e) = file_result {
        return Err(acadcheck::error::build_error!(source: e));
    }
    Ok(())
}

fn main() {
    if let Err(e) = message_error() {
        println!("{}", e);
    }
    if let Err(e) = source_error() {
        println!("{}", e);
    }
}
