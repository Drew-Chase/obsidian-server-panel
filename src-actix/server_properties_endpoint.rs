use actix_web::{get, post, web, HttpMessage, HttpRequest, HttpResponse, Responder};
use authentication::data::User;
use crypto::hashids::decode;
use serde_json::json;
use servers::server::Server;
use servers::server_database::ServerDatabase;
use servers::server_properties::ServerProperties;
use std::error::Error;

#[get("")]
pub async fn get_server_properties(id: web::Path<String>, req: HttpRequest) -> Result<impl Responder, Box<dyn Error>> {
    if let Some(user) = req.extensions().get::<User>() {
        let id_number = match decode(id.as_str()) {
            Ok(id_number) => id_number[0],
            Err(_) => return Ok(HttpResponse::BadRequest().json(json!({"error":"Invalid ID"}))),
        };
        let server = Server::get_owned_server(id_number, user.id as u64)?;
        return Ok(HttpResponse::Ok().json(server.get_properties()?));
    }

    Ok(HttpResponse::Unauthorized().json(json!({"error":"Unauthorized"})))
}
#[post("/{key}")]
pub async fn set_server_property(path: web::Path<(String, String)>, body: String, req: HttpRequest) -> Result<impl Responder, Box<dyn Error>> {
    if let Some(user) = req.extensions().get::<User>() {
        let (id, key) = path.into_inner();

        let id_number = match decode(id.as_str()) {
            Ok(id_number) => id_number[0],
            Err(_) => return Ok(HttpResponse::BadRequest().json(json!({"error":"Invalid ID"}))),
        };
        let server = Server::get_owned_server(id_number, user.id as u64)?;
        server.set_property(&*key, &*body)?;
        return Ok(HttpResponse::Ok().finish());
    }
    Ok(HttpResponse::Unauthorized().json(json!({"error":"Unauthorized"})))
}
