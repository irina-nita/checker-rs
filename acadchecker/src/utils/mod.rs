#![allow(unused)]
#![allow(dead_code)]

use std::{
    fmt::Debug,
    io::{stdin, stdout, Stdin},
    path::PathBuf,
};

pub const HELP_ARG: &str = "--help";

pub const STANDALONE_ARGS: [&str; 1] = ["--memory-footprint"];

pub const KEY_PATH_ARGS: [&str; 6] = [
    "--config",
    "--solution-file", // TODO
    "--solution-dir",  // TODO
    "--inrefs",        // TODO
    "--ins",           // TODO
    "--refs",          // TODO
];

pub const KEY_STRING_ARGS: [&str; 1] = ["--language"];

pub const KEY_NUM_ARGS: [&str; 1] = ["--timeout"];

const ERROR_MSG: &str = "Try 'acadchecker --help' for more information.\n";

/// Arguments for the building of checker.
#[derive(Debug)]
pub enum Arguments {
    /// Path to a json file that contains the in-refs hashmap.
    InRefs(std::path::PathBuf),
    /// Path to directory where the input files are stored.
    /// They will be matched to the refs as pairs in alphabetic order.
    Ins(std::path::PathBuf),
    /// Path to directory where the ref files are stored.
    /// They will be matched to the refs as pairs in alphabetic order.
    Refs(std::path::PathBuf),
    /// Puts a timeout for running the solution.
    Timeout(usize),
    /// Printing out to stdout the memory footprint of the program.
    MemoryFootprint(usize),
    /// Gets the full configuration from a JSON file.
    FromJson(std::path::PathBuf),
    /// The Language that the solution is written in.
    /// For now, only C, C++, Python, Go and D are accepted.
    Language(std::ffi::OsString),
    /// The source file for the solution.
    SolutionFile(std::path::PathBuf),
    /// The source dir for the solution.
    SolutionDir(std::path::PathBuf),
}

/// Message to stdout on wrong arguments.
pub fn exit_with_arg(arg: &str) {
    print!("unrecognized option: '{}'\n{}", arg, ERROR_MSG);
}

/// Message to stdout.
pub fn exit() {
    print!("{}", ERROR_MSG);
    std::process::exit(0);
}

pub fn get_args<S, I>(args: I) -> Vec<Arguments>
where
    S: AsRef<str> + Debug + PartialEq,
    I: IntoIterator<Item = S>,
{
    // Iterator over args.
    let mut iter = args.into_iter();

    // Get over the "acadchecker" arg.
    iter.next();

    // Argument with index.
    let mut arg = iter.next();

    // Sanity variable to check if the checker will accept the "--config" option.
    let mut accept_config = true;

    // First check the arg is "--help".
    if arg == None {
        exit();
    }

    // Check to see if the arg
    if arg.unwrap().as_ref().eq(KEY_PATH_ARGS[0]) {
        arg = iter.next();
        if arg != None {
            return vec![Arguments::FromJson(PathBuf::from(arg.unwrap().as_ref()))];
        }
    } else {
        exit();
    }
    vec![]
}
