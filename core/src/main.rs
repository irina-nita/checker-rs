#![allow(dead_code)]
#![allow(unused_imports)]

use actix_web::{self, App, HttpRequest, HttpResponse, HttpServer};
pub mod api;

/// Configuration envs for port and host.
#[derive(Debug, serde::Deserialize)]
struct Config {
    port: u16,
    host: String,
}

/// Problem envs for path to save the ins, refs and source.
#[derive(Debug, serde::Deserialize, Clone)]
struct Problem {
    ins: std::path::PathBuf,
    refs: std::path::PathBuf,
    src: std::path::PathBuf,
    out: std::path::PathBuf,
    cfg: std::path::PathBuf,
}

#[tokio::main]
async fn main() -> std::io::Result<()> {
    // Get environment variables.
    let config = envy::from_env::<Config>().expect("Please provide PORT and HOST in .env");

    // Get problem paths for storing.
    let problem = envy::prefixed("__PROBLEM_").from_env::<Problem>().expect(
        "Please provide __PROBLEM_INS, __PROBLEM_REFS, __PROBLEM_OUT and __PROBLEM_SRC in .env",
    );

    // Start service.
    HttpServer::new(move || App::new().app_data(problem.clone()).service(api::run()))
        .bind((config.host, config.port))?
        .run()
        .await
}
