#![allow(unused_imports)]
#![allow(unused_variables)]

pub fn run() -> actix_web::Scope {
    actix_web::web::scope("/checker").route("/run", actix_web::web::post().to(checker_run))
}

/// Response on upload.
#[derive(serde::Deserialize, serde::Serialize)]
pub struct UploadResponse {
    message: String,
}

/// Source, ins and refs.
#[derive(actix_multipart::form::MultipartForm)]
struct UploadProblem {
    src: Vec<actix_multipart::form::tempfile::TempFile>,
    ins: Vec<actix_multipart::form::tempfile::TempFile>,
    refs: Vec<actix_multipart::form::tempfile::TempFile>,
    cfg: actix_multipart::form::json::Json<acadcheck::acadchecker::config::Config>,
}

/// Get the filename of a tempfile.
fn get_file_name(file: &actix_multipart::form::tempfile::TempFile) -> String {
    file.file_name
        .as_ref()
        .map(|m| m.as_ref())
        .unwrap_or("null")
        .to_string()
}

/// Upload inputs, refs and source.
async fn upload(
    req: &actix_web::HttpRequest,
    form: &actix_multipart::form::MultipartForm<UploadProblem>,
) -> actix_web::HttpResponse {
    let mut error = None;

    // Save inputs to local file system.
    let ins = form
        .ins
        .iter()
        .map(|f| {
            let path = f.file.path();

            let mut new_path =
                std::path::PathBuf::from(req.app_data::<crate::Problem>().unwrap().ins.clone());
            new_path.push(&sanitize_filename::sanitize(&get_file_name(f)));

            match std::fs::rename(path, new_path) {
                Ok(_) => {}
                Err(err) => {
                    error = Some(err);
                }
            }
        })
        .collect::<Vec<_>>();

    // Save references.
    let refs = form
        .refs
        .iter()
        .map(|f| {
            let path = f.file.path();

            let mut new_path =
                std::path::PathBuf::from(req.app_data::<crate::Problem>().unwrap().refs.clone());
            new_path.push(&sanitize_filename::sanitize(&get_file_name(f)));

            match std::fs::rename(path, new_path) {
                Ok(_) => {}
                Err(err) => {
                    error = Some(err);
                }
            }
        })
        .collect::<Vec<_>>();

    // Save source.
    let src = form
        .src
        .iter()
        .map(|f| {
            let path = f.file.path();

            let mut new_path =
                std::path::PathBuf::from(req.app_data::<crate::Problem>().unwrap().src.clone());
            new_path.push(&sanitize_filename::sanitize(&get_file_name(f)));

            match std::fs::rename(path, new_path) {
                Ok(_) => {}
                Err(err) => {
                    error = Some(err);
                }
            }
        })
        .collect::<Vec<_>>();

    // In case of any error during upload, return Internal Server Error.
    return if let Some(err) = error {
        actix_web::HttpResponse::InternalServerError().json(UploadResponse {
            message: err.to_string(),
        })
    } else {
        actix_web::HttpResponse::Ok().json(UploadResponse {
            message: format!("Files successfully uploaded."),
        })
    };
}

/// Run the checker.
async fn checker_run(
    req: actix_web::HttpRequest,
    form: actix_multipart::form::MultipartForm<UploadProblem>,
) -> actix_web::HttpResponse {
    // Upload the files first.
    upload(&req, &form).await;

    // Write to config file.
    let config_file = std::fs::File::create(req.app_data::<crate::Problem>().unwrap().cfg.clone());

    if let Err(err) = config_file {
        return actix_web::HttpResponse::InternalServerError().json(err.to_string());
    }

    let mut config_file = config_file.unwrap();

    use std::io::Write;

    if let Err(err) = config_file.write_all(
        serde_json::to_string_pretty(&form.cfg.0)
            .unwrap()
            .as_bytes(),
    ) {
        return actix_web::HttpResponse::InternalServerError().json(err.to_string());
    }

    // Build command and run it.
    let mut cmd = std::process::Command::new("acadchecker");

    let cmd = &mut cmd
        .arg("--config")
        .arg(req.app_data::<crate::Problem>().unwrap().cfg.clone());

    let output = cmd.output();

    if let Err(err) = output {
        return actix_web::HttpResponse::InternalServerError().json(err.to_string());
    }

    let output = output.unwrap();
    let output = String::from_utf8(output.stdout).unwrap();
    let output: acadcheck::acadchecker::config::Output = serde_json::from_str(&output).unwrap();

    return actix_web::HttpResponse::Ok().json(output);
}
