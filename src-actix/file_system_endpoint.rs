use common_lib::traits::TransformPath;
use actix_multipart::form::{json::Json as MPJson, tempfile::TempFile, MultipartForm};
use actix_web::{get, post, web, HttpMessage, HttpRequest, HttpResponse, Responder};
use authentication::data::User;
use crypto::hashids::decode;
use log::{debug, error};
use serde::Deserialize;
use serde_json::json;
use servers::physical_server;
use servers::physical_server::{get_file_type, FileSystemEntry};
use std::fs::File;
use std::io::{Read, Write};
use std::path::PathBuf;

#[post("")]
pub async fn get_server_files(
    id: web::Path<String>,
    body: Option<String>,
    req: HttpRequest,
) -> impl Responder {
    if let Some(user) = req.extensions().get::<User>() {
        let id_number = match decode(&id) {
            Ok(id) => id,
            Err(_) => {
                error!("Invalid id: {}", id);
                return HttpResponse::BadRequest()
                    .json(json!({"error": format!("Invalid id: {}", id)}));
            }
        };
        if id_number.is_empty() {
            return HttpResponse::BadRequest()
                .json(json!({"error": format!("Invalid id: {}", id)}));
        }
        return HttpResponse::Ok().json(physical_server::get_server_filesystem_entries(
            id_number[0] as u32,
            user.id,
            body,
        ));
    }

    HttpResponse::Unauthorized().json(json!({"error": "Unauthorized"}))
}

#[derive(Debug, Deserialize)]
struct UploadFileMetadata {
    directory: String,
    filename: String,
}

#[derive(Debug, MultipartForm)]
struct UploadForm {
    file: TempFile,
    json: MPJson<UploadFileMetadata>,
}

#[post("/upload")]
pub async fn upload_file_to_server(
    id: web::Path<String>,
    MultipartForm(mut payload): MultipartForm<UploadForm>,
    req: HttpRequest,
) -> impl Responder {
    let directory = payload.json.directory.clone();
    let filename = payload.json.filename.clone();
    if let Some(user) = req.extensions().get::<User>() {
        let id_number = match decode(&id) {
            Ok(id) => id,
            Err(_) => {
                error!("Invalid id: {}", id);
                return HttpResponse::BadRequest()
                    .json(json!({"error": format!("Invalid id: {}", id)}));
            }
        };
        if id_number.is_empty() {
            return HttpResponse::BadRequest()
                .json(json!({"error": format!("Invalid id: {}", id)}));
        }
        let id_number = id_number[0] as u32;

        let server = servers::server_db::get_owned_server_by_id(id_number, user.id);
        if let Some(server) = server {
            if let Some(server_dir) = server.directory {
                let path = format!("{}{}{}", server_dir, directory, filename);
                debug!("Uploading file to: {:?}", path);
                debug!(
                    "Server Directory: {:?}, Directory: {:?}, Filename: {:?}",
                    server_dir, directory, filename
                );

                let mut file = match File::create(&path) {
                    Ok(file) => file,
                    Err(e) => {
                        error!("Error creating file: {:?}", e);
                        return HttpResponse::InternalServerError()
                            .json(json!({"error": "Error creating file"}));
                    }
                };
                let temp_file = payload.file.file.as_file_mut();
                let mut temp_bytes: Vec<u8> = Vec::new();
                if let Err(e) = temp_file.read_to_end(&mut temp_bytes) {
                    error!("Error reading file: {:?}", e);
                    return HttpResponse::InternalServerError()
                        .json(json!({"error": "Error reading file"}));
                }
                if let Err(e) = file.write_all(&*temp_bytes) {
                    error!("Error writing file: {:?}", e);
                    return HttpResponse::InternalServerError()
                        .json(json!({"error": "Error writing file"}));
                }
            }
        } else {
            return HttpResponse::BadRequest().json(json!({"error": "Server not found"}));
        }
        return HttpResponse::Ok().json(json!({"success": "File uploaded"}));
    }

    HttpResponse::Unauthorized().json(json!({"error": "Unauthorized"}))
}

#[post("")]
pub async fn get_files(body: Option<String>, req: HttpRequest) -> impl Responder {
    if req.extensions().get::<User>().is_none() {
        return HttpResponse::Unauthorized().json(json!({"error": "Unauthorized"}));
    }
    let path = body.unwrap_or_default();
    let path = std::env::current_dir().unwrap().join(path);
    match std::fs::read_dir(&path) {
        Ok(entries) => {
            let mut file_system_entries: Vec<FileSystemEntry> = Vec::new();
            for entry in entries {
                let entry = entry.unwrap();
                let metadata = entry.metadata().unwrap();
                let file_system_entry = FileSystemEntry {
                    name: entry.file_name().into_string().unwrap(),
                    path: entry.path(),
                    size: metadata.len(),
                    r#type: get_file_type(
                        entry
                            .path()
                            .extension()
                            .unwrap_or_default()
                            .to_str()
                            .unwrap_or_default()
                            .to_string(),
                    ),
                    created: metadata
                        .created()
                        .unwrap_or(std::time::SystemTime::UNIX_EPOCH),
                    is_dir: metadata.is_dir(),
                    last_modified: metadata
                        .modified()
                        .unwrap_or(std::time::SystemTime::UNIX_EPOCH),
                };
                file_system_entries.push(file_system_entry);
            }
            HttpResponse::Ok().json(file_system_entries)
        }
        Err(e) => {
            error!("Error reading directory: {:?}", e);
            HttpResponse::InternalServerError().json(json!({"error": "Error reading directory"}))
        }
    }
}

#[get("/download/{file}")]
pub async fn download_file(path: web::Path<(String, String)>, req: HttpRequest) -> impl Responder {
    let (id, file) = path.into_inner();

    // decode the uri encoded file path
    let file: String = match percent_encoding::percent_decode(file.as_bytes()).decode_utf8() {
        Ok(file) => file.to_string(),
        Err(e) => {
            error!("Error decoding file path: {:?}", e);
            return HttpResponse::BadRequest().json(json!({"error": "Error decoding file path"}));
        }
    };

    if let Some(user) = req.extensions().get::<User>() {
        let id_number = match decode(&id) {
            Ok(id) => id,
            Err(_) => {
                error!("Invalid id: {}", id);
                return HttpResponse::BadRequest()
                    .json(json!({"error": format!("Invalid id: {}", id)}));
            }
        };
        if id_number.is_empty() {
            return HttpResponse::BadRequest()
                .json(json!({"error": format!("Invalid id: {}", id)}));
        }
        let id_number = id_number[0] as u32;
        if let Some(server) = servers::server_db::get_owned_server_by_id(id_number, user.id) {
            let path = format!("{}{}", server.directory.unwrap_or_default(), file);
            debug!("Downloading file: {:?}", path);
            return match File::open(&path) {
                Ok(mut file) => {
                    let mut bytes: Vec<u8> = Vec::new();
                    if let Err(e) = file.read_to_end(&mut bytes) {
                        error!("Error reading file: {:?}", e);
                        return HttpResponse::InternalServerError()
                            .json(json!({"error": "Error reading file"}));
                    }
                    HttpResponse::Ok()
                        .content_type("application/octet-stream")
                        .body(bytes)
                }
                Err(e) => {
                    error!("Error opening file: {:?}", e);
                    HttpResponse::InternalServerError().json(json!({"error": "Error opening file"}))
                }
            };
        }
    }
    HttpResponse::Unauthorized().json(json!({"error": "Unauthorized"}))
}
