//! `acadcheck` is a simple to use, flexible library for building checkers used
//! in programming contests or homeworks. This crate is meant to act as a base,
//! avoiding the need of writing different scripts from scratch for different
//! types of usage.
//!
//! # __Installation__
//!
//! ```toml
//! [dependencies]
//! acadcheck = "0.1.7"
//! ```
//!
//! # __Features__
//!
//! * `use-serde` for serialisation of tests output and checker configuration.
//!
//! ```toml
//! acadcheck = { version = "0.1.7", features = ["use-serde"] }
//! ```
#![deny(clippy::undocumented_unsafe_blocks)]
#![deny(clippy::panicking_unwrap)]
#![deny(clippy::serde_api_misuse)]
// #![feature(marker_trait_attr)]
#![allow(unreachable_patterns)]
#![allow(dead_code)]
#![cfg_attr(docsrs, feature(doc_cfg))]

extern crate regex;
extern crate sanitize_filename;
extern crate thiserror;
extern crate tokio;

#[cfg(feature = "use-serde")]
extern crate serde;
#[cfg(feature = "use-serde")]
extern crate serde_json;

pub mod acadchecker;
pub mod checker;
pub mod language;
pub mod solution;
pub(crate) mod util;
