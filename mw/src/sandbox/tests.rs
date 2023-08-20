use std::{io::Seek, path::Path};

use anyhow::Ok;
use shiplift::Container;

use crate::api::utils::SandboxConfig;

use super::*;

#[tokio::test]
async fn create_container() {
    let image = "sandbox:latest";
    let docker: shiplift::Docker = Docker::new();
    if let Err(e) = docker.build_sandbox_from(image, vec!["ls"]).await {
        panic!("{:?}", e.to_string());
    }
    //let c = docker.build_sandbox_from(image, vec!["ls"]).await.unwrap();
}

#[tokio::test]
async fn copy_files_in_container() {
    let image = "sandbox:latest";
    let docker: shiplift::Docker = Docker::new();

    let mut f = match tempfile::NamedTempFile::new() {
        Result::Ok(t) => t,
        Err(e) => {
            panic!("{:?}", e.to_string());
        }
    };

    let buf = "Hello World!\n";

    use std::io::Write;

    writeln!(f, "Brian was here. Briefly.").unwrap();

    f.seek(std::io::SeekFrom::Start(0)).unwrap();
    let filen = f.path().file_name().unwrap().to_str().unwrap().to_string();

    let v = vec!["cat", filen.as_str()];

    println!("{:?}", v);

    let container = match docker.build_sandbox_from(image, v).await {
        Result::Ok(c) => c,
        Err(e) => {
            panic!("{:?}", e.to_string());
        }
    };

    let path = Path::new("/restricted/home/sandbox");
    container.copy_file(&mut f, path).await.unwrap();

    // container.start().await.unwrap();
    container.run_checker().await.unwrap();
}

#[tokio::test]
async fn copy_config() {
    let mut fp = File::open("sandbox.json").unwrap();
    let mut buf = String::new();

    let _ = fp.read_to_string(&mut buf).unwrap();

    let sandbox_config = serde_json::from_str::<SandboxConfig>(buf.as_str()).unwrap();
    println!("{:?}", sandbox_config);
    let s_conf = std::sync::Arc::new(sandbox_config);
    // Run the checker. (This should destroy the sandbox)
    let sandbox =
        crate::sandbox::SandboxedChecker::new("sandbox:latest", vec!["tree", "."], s_conf.clone());
    //sandbox.run_once(&Docker::new(), , refs, config)
}
