//! Middleware for the acadnet checker.
//!
//! Can act as an example for using the [`acadchecker`] package.
use std::{fs::File, io::Read, path::PathBuf};

use actix_web::{self, App, HttpServer};
use api::utils::{SandboxConfig, PROVIDER_NAME};
#[allow(unused_imports)]
use futures_util::{StreamExt, TryStreamExt};
use serde::Deserialize;

pub mod api;

pub mod sandbox;

/// Config of port and host.
#[derive(Debug, Deserialize)]
struct Config {
    port: u16,
    host: String,
    sandbox_config: PathBuf,
}

/// Config of AWS access.
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

    let sandbox_config = match serde_json::from_str::<SandboxConfig>(&c) {
        Ok(s) => s,
        Err(e) => {
            panic!("{}", e.to_string());
        }
    };
    // Load AWS Configuration as
    let aws_conf = envy::from_env::<AWSConfig>()
        .expect("Please provide AWS_ACCESS_KEY_ID, AWS_SECRET_ACCESS_KEY and S3_SECRET_KEY");

    let cred = aws_sdk_s3::Credentials::new(
        &aws_conf.aws_access_key_id,
        &aws_conf.aws_secret_access_key,
        None,
        None,
        PROVIDER_NAME,
    );

    let region = aws_sdk_s3::Region::new("eu-west-2".to_string());

    // Setup builder.
    let builder = aws_sdk_s3::config::Builder::new()
        .credentials_provider(cred)
        .region(region);
    let aws_config = builder.build();
    let client = aws_sdk_s3::Client::from_conf(aws_config);

    // Logger.
    env_logger::init_from_env(env_logger::Env::new().default_filter_or("info"));

    // Start service.
    HttpServer::new(move || {
        App::new()
            .app_data(actix_web::web::Data::new(client.clone()))
            .app_data(actix_web::web::Data::new(sandbox_config.clone()))
            .wrap(tracing_actix_web::TracingLogger::default())
            .service(api::run())
            .service(api::health())
    })
    .bind((config.host, config.port))?
    .run()
    .await
}
