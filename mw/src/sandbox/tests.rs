use shiplift::Docker;
use std::{io::Seek, path::Path};

use super::*;

const LOCAL_IMAGE: &str = "sandbox:latest";
const HEALTH_CHECK_CMD: &str = "ls";

#[tokio::test]
async fn create_container() {
    let docker: shiplift::Docker = Docker::new();
    if let Err(e) = docker
        .build_sandbox_from(LOCAL_IMAGE, vec![HEALTH_CHECK_CMD])
        .await
    {
        panic!("{:?}", e.to_string());
    }
}

#[tokio::test]
async fn copy_files_in_container() {
    let docker: shiplift::Docker = Docker::new();

    let mut f = match tempfile::NamedTempFile::new() {
        Result::Ok(t) => t,
        Err(e) => {
            panic!("{:?}", e.to_string());
        }
    };

    use std::io::Write;
    writeln!(f, "Brian was here. Briefly").unwrap();
    f.seek(std::io::SeekFrom::Start(0)).unwrap();
    let filen = f.path().file_name().unwrap().to_str().unwrap().to_string();
    let v = vec!["cat", filen.as_str()];

    let container = match docker.build_sandbox_from(LOCAL_IMAGE, v).await {
        Result::Ok(c) => c,
        Err(e) => {
            panic!("{:?}", e.to_string());
        }
    };
    let path = Path::new("/");
    container.copy_file(&mut f, path).await.unwrap();

    container.start().await.unwrap();
}
