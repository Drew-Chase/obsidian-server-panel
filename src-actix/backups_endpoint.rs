use actix_web::{get, post, web, HttpMessage, HttpRequest, HttpResponse, Responder};
use authentication::data::User;
use backups::backup_item::{BackupCreationMethod, BackupItem, BackupType};
use backups::hashed_backup_item::HashedBackupItem;
use crypto::hashids::decode;
use log::error;
use serde_json::json;
use servers::server_db;
use std::path::Path;

#[get("/")]
pub async fn get_backups(id: web::Path<String>, req: HttpRequest) -> impl Responder {
    if let Some(_) = req.extensions().get::<User>() {
        let id_number: u32 = match decode(id.as_str()) {
            Ok(id_number) => id_number[0] as u32,
            Err(_) => return HttpResponse::BadRequest().json(json!({"error":"Invalid ID"})),
        };

        return HttpResponse::Ok().json(json!(BackupItem::from_server(id_number)
            .iter()
            .map(|e| { e.clone().hash() })
            .collect::<Vec<HashedBackupItem>>()));
    }

    HttpResponse::Unauthorized().json(json!({"error":"Unauthorized"}))
}

#[post("/create")]
pub async fn create_manual_backup(id: web::Path<String>, req: HttpRequest) -> impl Responder {
    if let Some(user) = req.extensions().get::<User>() {
        let id_number: u32 = match decode(id.as_str()) {
            Ok(id_number) => id_number[0] as u32,
            Err(_) => return HttpResponse::BadRequest().json(json!({"error":"Invalid ID"})),
        };

        let server = match server_db::get_owned_server_by_id(id_number, user.id) {
            Some(s) => s,
            None => {
                let msg = format!("Server with id: {} not found", id_number);
                error!("{}", msg);
                return HttpResponse::BadRequest().json(json!({"error":msg}));
            }
        };

        if let Some(server_directory) = server.directory {
            let item = match BackupItem::create_backup(
                id_number,
                Path::new(&server_directory),
                BackupCreationMethod::MANUAL,
                BackupType::Incremental,
            ) {
                Ok(b) => b.hash(),
                Err(e) => {
                    let msg = format!("Failed to create backup: {}", e);
                    error!("{}", msg);
                    return HttpResponse::BadRequest().json(json!({"error":msg}));
                }
            };
            return HttpResponse::Ok().json(json!(item));
        }
    }

    HttpResponse::Unauthorized().json(json!({"error":"Unauthorized"}))
}
