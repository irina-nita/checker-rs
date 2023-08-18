//! Middleware for the acadnet checker.
//!
//! Can act as an example for using the [`acadnet-checker`] crate.

use std::{fs::File, io::Read, path::PathBuf};

use actix_web::{self, App, HttpServer};
use api::utils::{DockerDaemon, Sandbox, SandboxConfig};
use serde::Deserialize;

pub mod api;

/// Cofniguration of port and host.
#[derive(Debug, Deserialize)]
struct Config {
    port: u16,
    host: String,
    sandbox_config: PathBuf,
}

/// Configuration of AWS access key configuration.
#[derive(serde::Serialize, serde::Deserialize)]
pub struct AWSConfig {
    aws_access_key_id: String,
    aws_secret_access_key: String,
    s3_secret_key: String,
}

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    // Get port and host env. variables.
    let config =
        envy::from_env::<Config>().expect("Please provide PORT, HOST and SANDBOX_CONFIG in .env");

    // Get sandbox.
    let mut sc =
        File::open(config.sandbox_config).expect("Can't open file for sandbox configuration.");
    let mut c = String::new();

    match sc.read_to_string(&mut c) {
        Ok(_) => {}
        Err(_) => {
            panic!("Can't read file for sandbox configuration.");
        }
    }

    // Start service.
    HttpServer::new(move || {
        let sandbox_config = match serde_json::from_str::<SandboxConfig>(&c) {
            Ok(s) => std::sync::Arc::new(s),
            Err(e) => {
                panic!("{}", e.to_string());
            }
        };

        App::new()
            .app_data(actix_web::web::Data::new(sandbox_config.clone()))
            .service(api::run())
    })
    .bind((config.host, config.port))?
    .run()
    .await
}
