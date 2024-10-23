use actix_multipart::form::{json::Json as MPJson, tempfile::TempFile, MultipartForm};
use actix_web::{post, web, HttpMessage, HttpRequest, HttpResponse, Responder};
use authentication::data::User;
use crypto::hashids::decode;
use log::error;
use serde::Deserialize;
use serde_json::json;
use servers::physical_server;

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
            
        } else {
            return HttpResponse::BadRequest().json(json!({"error": "Server not found"}));
        }
    }

    HttpResponse::Unauthorized().json(json!({"error": "Unauthorized"}))
}
