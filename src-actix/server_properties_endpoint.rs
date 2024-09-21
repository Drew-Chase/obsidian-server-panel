use actix_web::{get, post, web, HttpMessage, HttpRequest, HttpResponse, Responder};
use authentication::data::User;
use crypto::hashids::decode;
use log::error;
use serde_json::json;
use servers::properties::Properties;
use servers::server_db;
use std::path::Path;

#[get("/")]
pub async fn get_server_properties(id: web::Path<String>, req: HttpRequest) -> impl Responder {
    if let Some(user) = req.extensions().get::<User>() {
        let id_number: u32 = match decode(id.as_str()) {
            Ok(id_number) => id_number[0] as u32,
            Err(e) => return HttpResponse::BadRequest().json(json!({"error":"Invalid ID"})),
        };
        let server = match server_db::get_server_by_id(id_number) {
            Some(s) => s,
            None => {
                let msg = format!("Server with id: {} not found", id_number);
                error!("{}", msg);
                return HttpResponse::BadRequest().json(json!({"error":msg}));
            }
        };
        if server.owner == user.id {
            let properties: Properties = match Properties::new(&Path::join(
                Path::new(server.directory.unwrap().as_str()),
                Path::new("server.properties"),
            )) {
                Ok(p) => p,
                Err(e) => {
                    error!("{}", e);
                    return HttpResponse::BadRequest().json(json!({"error":e}));
                }
            };

            return HttpResponse::Ok().json(properties.items);
        }
    }

    HttpResponse::Unauthorized().json(json!({"error":"Unauthorized"}))
}
#[post("/{key}")]
pub async fn set_server_property(
    path: web::Path<(String, String)>,
    body: String,
    req: HttpRequest,
) -> impl Responder {
    if let Some(user) = req.extensions().get::<User>() {
        let (id, key) = path.into_inner();
        let id_number: u32 = match decode(id.as_str()) {
            Ok(id_number) => id_number[0] as u32,
            Err(e) => return HttpResponse::BadRequest().json(json!({"error":"Invalid ID"})),
        };
        let server = match server_db::get_owned_server_by_id(id_number, user.id) {
            Some(s) => s,
            None => {
                let msg = format!("Server with id: {} not found", id_number);
                error!("{}", msg);
                return HttpResponse::BadRequest().json(json!({"error":msg}));
            }
        };
        let mut properties: Properties = match Properties::new(&Path::join(
            Path::new(server.directory.unwrap().as_str()),
            Path::new("server.properties"),
        )) {
            Ok(p) => p,
            Err(e) => {
                error!("{}", e);
                return HttpResponse::BadRequest().json(json!({"error":e}));
            }
        };

        properties.set(&key, &body);
        return match properties.write() {
            Ok(_) => HttpResponse::Ok().json(json!({"success":"Property set"})),
            Err(e) => {
                error!("{}", e);
                HttpResponse::BadRequest().json(json!({"error":e}))
            }
        };
    }
    HttpResponse::Unauthorized().json(json!({"error":"Unauthorized"}))
}
