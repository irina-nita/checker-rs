//! `acadcheck` is a simple to use, flexible library for building checkers used
//! in programming contests or homeworks. This crate is meant to act as a base,
//! avoiding the need of writing different scripts from scratch for different
//! types of usage.
//!
//! # __Installation__
//!
//! ```toml
//! [dependencies]
//! acadnet = "0.1.0"
//! ```
//!
//! # __Features__
//!
//! * `serde` for serialisation of outputs and errors.
//!
//! ```toml
//! acadnet = { version = "0.1.0", features = ["serde"] }
//! ```
#![deny(clippy::undocumented_unsafe_blocks)]
#![deny(clippy::panicking_unwrap)]
#![deny(clippy::serde_api_misuse)]
#![feature(marker_trait_attr)]
#![allow(dead_code)]

extern crate regex;
extern crate sanitize_filename;
extern crate thiserror;
extern crate tokio;

#[cfg(feature = "use-serde")]
extern crate serde;
#[cfg(feature = "use-serde")]
extern crate serde_json;

pub mod checker;
pub mod language;
pub mod solution;
pub(crate) mod util;
pub use checker::tests::Error as TestError;
pub use checker::tests::Output as TestOutput;
