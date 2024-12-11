use actix_web::{get, post, web, HttpMessage, HttpRequest, HttpResponse, Responder};
use authentication::data::User;
use backups::backup_item::{BackupCreationMethod, BackupItem, BackupType};
use backups::hashed_backup_item::HashedBackupItem;
use crypto::hashids::decode;
use log::error;
use serde_json::json;
use servers::server::Server;
use servers::server_database::ServerDatabase;
use std::error::Error;
use std::path::Path;

#[get("")]
pub async fn get_backups(id: web::Path<String>, req: HttpRequest) -> impl Responder {
    if req.extensions().get::<User>().is_some() {
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

#[post("/create/{method}")]
pub async fn create_manual_backup(
    path: web::Path<(String, BackupType)>,
    req: HttpRequest,
) -> Result<impl Responder, Box<dyn Error>> {
    let (id, method) = path.into_inner();

    if let Some(user) = req.extensions().get::<User>() {
        let id_number = match decode(id.as_str()) {
            Ok(id_number) => id_number[0],
            Err(_) => return Ok(HttpResponse::BadRequest().json(json!({"error":"Invalid ID"}))),
        };

        let server = Server::get_owned_server(id_number, user.id as u64)?;

        let item = match BackupItem::create_backup(
            id_number as u32,
            Path::new(&server.directory),
            BackupCreationMethod::MANUAL,
            method,
        ) {
            Ok(b) => b.hash(),
            Err(e) => {
                let msg = format!("Failed to create backup: {}", e);
                error!("{}", msg);
                return Ok(HttpResponse::BadRequest().json(json!({"error":msg})));
            }
        };
        return Ok(HttpResponse::Ok().json(json!(item)));
    }

    Ok(HttpResponse::Unauthorized().json(json!({"error":"Unauthorized"})))
}
