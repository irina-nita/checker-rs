//! `acadchecker` is a CLI tool for building checkers used in programming contests or homeworks.
//! The checker is configured from a json file.
//! 
//! # Installation
//! 
//! ```shell
//! cargo install acadchecker
//! ```
//! 
//! # Usage
//! 
//! ```shell
//! acadchecker --config config.json
//! ```
//! 
//! # __Config Example__
//! ```json
//! {
//!   "checker": {
//!     "monitors": [
//!       {
//!         "time": {
//!           "limit": {
//!             "secs": 5,
//!             "nanos": 0
//!           }
//!         }
//!       }
//!     ],
//!     "output_type": {
//!       "scored": {
//!         "per_test": 5
//!       }
//!     },
//!     "in_refs": {
//!       "1": [
//!         "/binary/tests/in/001.in",
//!         "/binary/tests/ref/001.ref"
//!       ],
//!     }
//!   },
//!   "processor": {
//!     "gcc": {
//!       "language": "c++",
//!       "flags": [
//!         "-Werror",
//!         "-Wall"
//!       ],
//!       "exec": "/binary/solution"
//!     }
//!   },
//!   "solution": {
//!     "file": "/binary/solution.cpp"
//!   },
//!   "out_dir": "/binary/tests/out",
//!   "security": {
//!     "user": "sandbox",
//!     "group": "restricted"
//!   }
//! }
//! 
//! ```
//! 
//! 
use std::os::unix::process::CommandExt;

mod checker;
mod utils;

fn main() {
    // Get args and validate.
    let args = std::env::args().collect::<Vec<_>>();
    let args = utils::get_args(args);

    let mut config_file = std::path::PathBuf::new();

    // FromJson is the only `Argument` accepted for now.
    for arg in args {
        if let utils::Arguments::FromJson(path) = arg {
            config_file = path;
        } else {
            crate::utils::exit();
        }
    }

    // Get configuration from file :).
    let config = match utils::Config::from_json(std::path::PathBuf::from(config_file)) {
        Ok(c) => c,
        Err(err) => {
            println!("{}", err.to_string());
            crate::utils::exit();
            return;
        }
    };

    // Get necessary fields for the runner.

    // Output directory
    let out_dir = config.out_dir.clone();

    // Security options.
    let security = &config.security;

    // Get current uid and gid.
    let mut uid = nix::unistd::getuid().as_raw();
    let mut gid = nix::unistd::getgid().as_raw();

    if let Some(s) = security {
        // Get UID and GUID meant for the command.
        let user = match nix::unistd::User::from_name(&s.user) {
            Ok(u) => u,
            Err(_) => None,
        };

        let group = match nix::unistd::Group::from_name(&s.group) {
            Ok(g) => g,
            Err(_) => None,
        };

        if let Some(u) = user {
            uid = u.uid.into();
        }

        if let Some(g) = group {
            gid = g.gid.into();
        }
    }

    // Create output directory in case it doesn't exist.
    if !out_dir.exists() {
        let _ = std::fs::create_dir_all(&out_dir);
    }

    // Get monitors.
    let mut timeout_limit: Option<std::time::Duration> = None;
    let mut _timefootprint = false;

    for monitor in &config.checker.monitors {
        match monitor {
            acadcheck::checker::MonitorType::Timeout { limit } => {
                timeout_limit = Some(*limit)
            },
            acadcheck::checker::MonitorType::TimeFootprint => {
                _timefootprint = true;
            }
            _ => {}
        }
    }

    // Command is done; Now it needs to be ran for all tests.
    let _runner = |command: &Vec<std::ffi::OsString>,
                   inputs: std::collections::BTreeMap<usize, &std::path::PathBuf>|
     -> std::collections::BTreeMap<
        usize,
        Result<std::path::PathBuf, acadcheck::checker::Error>,
    > {
        // Build the command.
        let mut cmd = std::process::Command::new(&command[0]);

        // Check if args need to be added.
        if command.len() > 1 {
            cmd.args(&command[1..]);
        }

        // Output files are "$out_dir/$key.out".
        let out_file = |index: usize| -> std::path::PathBuf {
            let out = std::path::PathBuf::from(format!("{}.out", index));

            std::path::PathBuf::from(&out_dir).join(out)
        };

        // Run the commands for all tests.
        let children = inputs
            .into_iter()
            .map(|i| {
                // Collect in a hashmap of children.
                let input_file = std::fs::File::open(i.1);

                match input_file {
                    // If input can be opened, create output file.
                    Ok(f) => {
                        let out = std::fs::File::create((out_file)(i.0));

                        if let Err(e) = &out {
                            // Translate Error to acadcheck::checker::Error.
                            println!("Here bro!: {}", std::line!());
                            return (
                                i.0,
                                Err(acadcheck::checker::Error::TestError(e.to_string())),
                            );
                        }

                        let out = out.unwrap();

                        // Spawn children :).
                        let cmd = cmd
                            .stdin(std::process::Stdio::from(f))
                            .stdout(std::process::Stdio::from(out))
                            .uid(uid)
                            .gid(gid);
                            //.spawn();

                        let child = cmd.spawn();

                        match child {
                            Ok(c) => {
                                return (i.0, Ok(c));
                            }
                            Err(e) => {
                                return (
                                    i.0,
                                    Err(acadcheck::checker::Error::TestError(e.to_string())),
                                );
                            }
                        }
                    }
                    Err(e) => {
                        // Translate Error to acadcheck::checker::Error.
                        return (
                            i.0,
                            Err(acadcheck::checker::Error::TestError(e.to_string())),
                        );
                    }
                }
            })
            .collect::<std::collections::BTreeMap<
                usize,
                Result<std::process::Child, acadcheck::checker::Error>,
            >>();

        // The final results:
        let mut h: std::collections::BTreeMap<
            usize,
            Result<std::path::PathBuf, acadcheck::checker::Error>,
        > = std::collections::BTreeMap::new();

        // Wait for all the children.
        for child in children {
            match child.1 {
                Ok(mut c) => {
                    use wait_timeout::ChildExt;
                    
                    let mut get_status = || {
                        if let Some(duration) = timeout_limit {
                            let status = c.wait_timeout(duration);
                            match status {
                                Ok(s) => {
                                    match s {
                                        Some(st) => { return Ok(st); },
                                        None => {
                                            return Err(anyhow::format_err!("Time exceeded!"));
                                        }
                                    }
                                },
                                Err(e) => {
                                    return Err(anyhow::format_err!("{}", e.to_string()));
                                }
                            }
                        } else {
                            match c.wait() {
                                Ok(s) => { Ok(s) },
                                Err(e) => { Err(anyhow::format_err!("{}", e.to_string())) }
                            }
                        }
                    };
                    
                    if let Err(e) = (get_status)() {
                        h.insert(
                            child.0,
                            Err(acadcheck::checker::Error::TestError(e.to_string())),
                        );
                        continue;
                    }

                    let status = (get_status)().unwrap();

                    if !status.success() {
                        h.insert(
                            child.0,
                            Err(acadcheck::checker::Error::TestError(format!(
                                "Exit status: {}",
                                status
                            ))),
                        );
                        continue;
                    }

                    let output_path = (out_file)(child.0);
                    h.insert(child.0, Ok(output_path));
                    continue;
                }
                Err(e) => {
                    h.insert(
                        child.0,
                        Err(acadcheck::checker::Error::TestError(e.to_string())),
                    );
                    continue;
                }
            }
        }

        h
    };

    // Pray to God.
    let acadchecker = checker::AcadChecker::new();
    let result = acadchecker.run(config, _runner);

    println!("{}", serde_json::to_string_pretty(&result).unwrap());
}
