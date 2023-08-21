#![allow(unused_imports)]
#![allow(unused_variables)]
#![allow(dead_code)]

use std::{
    fmt::{format, Display},
    io::{Read, Seek, Write},
    os::fd::AsFd,
    path::PathBuf,
    sync::Arc,
};

use acadcheck::{
    acadchecker::config::SupportedProcessor, checker::MonitorType, language::gcc::Gcc,
};

use actix_web::{
    trace,
    web::{self, Data},
    Error, HttpRequest, HttpResponse, HttpResponseBuilder,
};
use aws_config::imds::client;
use aws_sdk_s3::{Client, Region};
use futures::{StreamExt, TryStreamExt};
use reqwest::Method;
use shiplift::Docker;
use std::fs::File;
use tempfile::NamedTempFile;

use crate::AWSConfig;

pub mod utils;

use utils::*;

pub fn run() -> actix_web::Scope {
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

    actix_web::web::scope("/submission")
        .app_data(actix_web::web::Data::new(client))
        .route("/run", actix_web::web::post().to(submission_run))
}

pub fn write_tests() {

}

/// Run a submission.
async fn submission_run(
    req: actix_web::HttpRequest,
    form: actix_multipart::form::MultipartForm<UploadSolution>,
) -> actix_web::HttpResponse {
    // Get AWS Client from app data.
    let client: Data<Client> = match req.app_data::<web::Data<Client>>() {
        Some(c) => c.clone(),
        None => {
            return HttpResponse::InternalServerError().json(Response {
                message: "AWS Client not found. Can not pull the problem.".to_string(),
            });
        }
    };

    // Get SandboxConfig.
    let sandbox_config: Data<SandboxConfig> = match req.app_data::<web::Data<SandboxConfig>>() {
        Some(s) => s.clone(),
        None => {
            return HttpResponse::InternalServerError().json(Response {
                message: "Sandbox Configuration not found. Can not run the checker".to_string(),
            });
        }
    };

    // Save tests archive in file.
    let mut tmp = match tempfile::tempfile() {
        Ok(t) => t,
        Err(e) => {
            return HttpResponse::InternalServerError().json(Response {
                message: e.to_string(),
            });
        }
    };

    // Tests should be stored in `{problem_id}/tests.zip`.
    let tests_path = format!("{}/{}", form.problem.to_string(), TESTS_ARCHIVE);

    // Get tests from bucket and problem id and store it in a tempdir.
    let mut tests = match client
        .get_object()
        .bucket(BUCKET_NAME)
        .key(tests_path)
        .send()
        .await
    {
        Ok(t) => t,
        Err(e) => {
            return HttpResponse::InternalServerError().json(Response {
                message: e.to_string(),
            });
        }
    };

    // Write to temp file.
    let mut byte_count = 0_usize;

    while let Some(bytes) = tests.body.try_next().await.unwrap() {
        let bytes = match tmp.write(&bytes) {
            Ok(b) => b,
            Err(e) => {
                return HttpResponse::InternalServerError().json(Response {
                    message: e.to_string(),
                });
            }
        };
        byte_count += bytes;
    }

    let reader = std::io::BufReader::new(tmp);

    // Send files.
    let mut archive = match zip::ZipArchive::new(reader) {
        Ok(a) => a,
        Err(e) => {
            return HttpResponse::InternalServerError().json(Response {
                message: e.to_string(),
            });
        }
    };

    let x = String::new();

    // All tests should have these files in the following format:
    // .
    // tests
    // ├── in
    // │   ├── 001.in
    // │   └── 002.in
    // └── ref
    //     ├── 001.ref
    //     └── 002.ref
    let in_reg = match regex::Regex::new(IN_REGEX) {
        Ok(r) => r,
        Err(e) => {
            return HttpResponse::InternalServerError().json(Response {
                message: e.to_string(),
            });
        }
    };

    let ref_reg = match regex::Regex::new(REF_REGEX) {
        Ok(r) => r,
        Err(e) => {
            return HttpResponse::InternalServerError().json(Response {
                message: e.to_string(),
            });
        }
    };

    let in_dir = match tempfile::tempdir() {
        Ok(d) => d,
        Err(e) => {
            return HttpResponse::InternalServerError().json(Response {
                message: e.to_string(),
            });
        }
    };

    let ref_dir = match tempfile::tempdir() {
        Ok(d) => d,
        Err(e) => {
            return HttpResponse::InternalServerError().json(Response {
                message: e.to_string(),
            });
        }
    };

    // Stored files.
    let mut in_refs_files =
        std::collections::BTreeMap::<usize, (Option<NamedTempFile>, Option<NamedTempFile>)>::new();

    for i in 0..archive.len() {
        // For each file in zip.
        let mut file = archive.by_index(i).unwrap();

        let outpath = match file.enclosed_name() {
            Some(path) => path,
            None => {
                return HttpResponse::InternalServerError().json(Response {
                    message: "Path in tests archive is corrupted.".to_string(),
                });
            }
        };

        if in_reg.is_match(file.name()) {
            let mut f = match tempfile::NamedTempFile::new_in(&in_dir) {
                Ok(t) => t,
                Err(e) => {
                    return HttpResponse::InternalServerError().json(Response {
                        message: e.to_string(),
                    });
                }
            };

            // Write to temp file the input.
            let mut contents = String::new();
            match file.read_to_string(&mut contents) {
                Ok(_) => {}
                Err(e) => {
                    return HttpResponse::InternalServerError().json(Response {
                        message: e.to_string(),
                    });
                }
            }

            match f.write_all(contents.as_bytes()) {
                Ok(_) => {}
                Err(e) => {
                    return HttpResponse::InternalServerError().json(Response {
                        message: e.to_string(),
                    });
                }
            }

            // Move back the cursor.
            match f.seek(std::io::SeekFrom::Start(0)) {
                Ok(_) => {}
                Err(e) => {
                    return HttpResponse::InternalServerError().json(Response {
                        message: e.to_string(),
                    });
                }
            }

            if let Some(p) =
                in_refs_files.get_mut(&file.name()[3..6].to_string().parse::<usize>().unwrap())
            {
                p.0 = Some(f);
            } else {
                in_refs_files.insert(
                    file.name()[3..6].to_string().parse::<usize>().unwrap(),
                    (Some(f), None),
                );
            }
        }

        if ref_reg.is_match(&file.name()) {
            let mut f = match tempfile::NamedTempFile::new_in(&ref_dir) {
                Ok(t) => t,
                Err(e) => {
                    return HttpResponse::InternalServerError().json(Response {
                        message: e.to_string(),
                    });
                }
            };

            // Write to temp file the ref.
            let mut contents = String::new();
            match file.read_to_string(&mut contents) {
                Ok(_) => {}
                Err(e) => {
                    return HttpResponse::InternalServerError().json(Response {
                        message: e.to_string(),
                    });
                }
            }

            match f.write_all(contents.as_bytes()) {
                Ok(_) => {}
                Err(e) => {
                    return HttpResponse::InternalServerError().json(Response {
                        message: e.to_string(),
                    });
                }
            }

            // Move back the cursor.
            match f.seek(std::io::SeekFrom::Start(0)) {
                Ok(_) => {}
                Err(e) => {
                    return HttpResponse::InternalServerError().json(Response {
                        message: e.to_string(),
                    });
                }
            }

            if let Some(p) =
                in_refs_files.get_mut(&file.name()[4..7].to_string().parse::<usize>().unwrap())
            {
                p.1 = Some(f);
            } else {
                in_refs_files.insert(
                    *&file.name()[3..6].to_string().parse::<usize>().unwrap(),
                    (None, Some(f)),
                );
            }
        }
    }

    let in_refs = in_refs_files
        .iter()
        .map(|(k, v)| {
            if v.0.is_some() && v.1.is_some() {
                (
                    *k,
                    (
                        PathBuf::from(format!(
                            "{}{}",
                            sandbox_config.r#in.to_str().unwrap(),
                            v.0.as_ref()
                                .unwrap()
                                .path()
                                .file_name()
                                .unwrap()
                                .to_str()
                                .unwrap()
                        )),
                        PathBuf::from(format!(
                            "{}{}",
                            sandbox_config.r#ref.to_str().unwrap(),
                            v.1.as_ref()
                                .unwrap()
                                .path()
                                .file_name()
                                .unwrap()
                                .to_str()
                                .unwrap()
                        )),
                    ),
                )
            } else {
                (*k, (std::path::PathBuf::new(), std::path::PathBuf::new()))
            }
        })
        .collect::<std::collections::BTreeMap<usize, (PathBuf, PathBuf)>>();

    //   let f = in_refs.get(&1).unwrap().1.to_str().unwrap().to_string();
    // Finally, build config and add to tempfile.
    let config = acadcheck::acadchecker::config::Config {
        checker: acadcheck::checker::CheckerConfig {
            monitors: {
                let v = vec![MonitorType::Timeout {
                    limit: std::time::Duration::from_secs(form.config.time_limit.secs),
                }];
                v
            },
            output_type: acadcheck::checker::OutputType::None,
            in_refs,
        },
        processor: (&form.config.processor).into(),
        solution: acadcheck::solution::Source::File(PathBuf::from(format!(
            "{}{}",
            sandbox_config.src.to_str().unwrap(),
            form.solution
                .file
                .path()
                .file_name()
                .unwrap()
                .to_str()
                .unwrap()
        ))),
        out_dir: sandbox_config.out.clone(),
        security: Some(sandbox_config.security.clone()),
    };

    // Tempfile to send to checker.
    let mut config_json = match tempfile::NamedTempFile::new() {
        Ok(f) => f,
        Err(e) => {
            return HttpResponse::InternalServerError().json(Response {
                message: format!("Can't parse configuration for checker: {:?}", e.to_string()),
            });
        }
    };

    let buf = match serde_json::to_string_pretty(&config) {
        Ok(j) => j,
        Err(e) => {
            return HttpResponse::InternalServerError().json(Response {
                message: format!("Can't parse configuration for checker: {:?}", e.to_string()),
            });
        }
    };

    match config_json.write_all(buf.as_bytes()) {
        Ok(_) => {}
        Err(e) => {
            return HttpResponse::InternalServerError().json(Response {
                message: format!("Can't parse configuration for checker: {:?}", e.to_string()),
            });
        }
    }

    config_json.seek(std::io::SeekFrom::Start(0)).unwrap();
    //let f = File::create("")
    let filename = String::from(
        config_json
            .path()
            .file_name()
            .unwrap()
            .clone()
            .to_str()
            .unwrap(),
    );

    let sandbox = crate::sandbox::SandboxedChecker::new(
        "sandbox:latest",
        vec!["acadchecker", "--config", filename.as_str()],
        sandbox_config.into_inner(),
    );

    let mut ins = vec![];
    let mut refs = vec![];
    for f in in_refs_files.into_iter() {
        ins.push(f.1 .1.unwrap());
        refs.push(f.1 .0.unwrap());
    }

    let mut sol = form.into_inner().solution.file;

    let res = sandbox
        .run_once(
            &Docker::new(),
            &mut refs,
            &mut sol,
            &mut ins,
            &mut config_json,
        )
        .await;
    HttpResponse::Ok().json(res)
}
