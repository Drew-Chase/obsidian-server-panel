use actix_web::{get, post, web, HttpMessage, HttpRequest, HttpResponse, Responder};
use authentication::data::User;
use crypto::hashids::decode;
use serde_json::json;
use servers::server_db;
use servers::server_db::ServerSettings;

#[get("")]
pub async fn get_server_settings(id: web::Path<String>, req: HttpRequest) -> impl Responder {
    if let Some(user) = req.extensions().get::<User>() {
        let id_number: u32 = match decode(id.as_str()) {
            Ok(id_number) => id_number[0] as u32,
            Err(_) => return HttpResponse::BadRequest().json(json!({"error":"Invalid ID"})),
        };
        return match server_db::get_owned_server_by_id(id_number, user.id) {
            Some(server) => HttpResponse::Ok().json(ServerSettings::from_server(server)),
            None => HttpResponse::NotFound().json(json!({"error":"Server not found"})),
        };
    }

    HttpResponse::Unauthorized().json(json!({"error":"Unauthorized"}))
}

#[post("/memory/min")]
pub async fn set_memory_min(
    id: web::Path<String>,
    body: String,
    req: HttpRequest,
) -> impl Responder {
    if let Some(user) = req.extensions().get::<User>() {
        let id_number: u32 = match decode(id.as_str()) {
            Ok(id_number) => id_number[0] as u32,
            Err(_) => return HttpResponse::BadRequest().json(json!({"error":"Invalid ID"})),
        };
        let server = match server_db::get_owned_server_by_id(id_number, user.id) {
            Some(server) => server,
            None => return HttpResponse::NotFound().json(json!({"error":"Server not found"})),
        };
        let min = match body.parse::<u64>() {
            Ok(min) => min,
            Err(e) => return HttpResponse::BadRequest().json(json!({"error":e.to_string()})),
        };
        return match server_db::set_memory_min(server.id, min) {
            Ok(_) => HttpResponse::Ok().json(json!({"success":"Memory min updated"})),
            Err(e) => HttpResponse::BadRequest().json(json!({"error":e})),
        };
    }

    HttpResponse::Unauthorized().json(json!({"error":"Unauthorized"}))
}

#[post("/memory/max")]
pub async fn set_memory_max(
    id: web::Path<String>,
    body: String,
    req: HttpRequest,
) -> impl Responder {
    if let Some(user) = req.extensions().get::<User>() {
        let id_number: u32 = match decode(id.as_str()) {
            Ok(id_number) => id_number[0] as u32,
            Err(_) => return HttpResponse::BadRequest().json(json!({"error":"Invalid ID"})),
        };
        let server = match server_db::get_owned_server_by_id(id_number, user.id) {
            Some(server) => server,
            None => return HttpResponse::NotFound().json(json!({"error":"Server not found"})),
        };
        let min = match body.parse::<u64>() {
            Ok(min) => min,
            Err(e) => return HttpResponse::BadRequest().json(json!({"error":e.to_string()})),
        };
        return match server_db::set_memory_max(server.id, min) {
            Ok(_) => HttpResponse::Ok().json(json!({"success":"Memory max updated"})),
            Err(e) => HttpResponse::BadRequest().json(json!({"error":e})),
        };
    }

    HttpResponse::Unauthorized().json(json!({"error":"Unauthorized"}))
}

#[post("/args/minecraft")]
pub async fn set_minecraft_arguments(
    id: web::Path<String>,
    body: String,
    req: HttpRequest,
) -> impl Responder {
    if let Some(user) = req.extensions().get::<User>() {
        let id_number: u32 = match decode(id.as_str()) {
            Ok(id_number) => id_number[0] as u32,
            Err(_) => return HttpResponse::BadRequest().json(json!({"error":"Invalid ID"})),
        };
        let server = match server_db::get_owned_server_by_id(id_number, user.id) {
            Some(server) => server,
            None => return HttpResponse::NotFound().json(json!({"error":"Server not found"})),
        };
        return match server_db::set_minecraft_arguments(server.id, body.as_str()) {
            Ok(_) => HttpResponse::Ok().json(json!({"success":"minecraft arguments updated"})),
            Err(e) => HttpResponse::BadRequest().json(json!({"error":e})),
        };
    }

    HttpResponse::Unauthorized().json(json!({"error":"Unauthorized"}))
}
#[post("/args/java")]
pub async fn set_java_arguments(
    id: web::Path<String>,
    body: String,
    req: HttpRequest,
) -> impl Responder {
    if let Some(user) = req.extensions().get::<User>() {
        let id_number: u32 = match decode(id.as_str()) {
            Ok(id_number) => id_number[0] as u32,
            Err(_) => return HttpResponse::BadRequest().json(json!({"error":"Invalid ID"})),
        };
        let server = match server_db::get_owned_server_by_id(id_number, user.id) {
            Some(server) => server,
            None => return HttpResponse::NotFound().json(json!({"error":"Server not found"})),
        };
        return match server_db::set_java_arguments(server.id, body.as_str()) {
            Ok(_) => HttpResponse::Ok().json(json!({"success":"JVM arguments updated"})),
            Err(e) => HttpResponse::BadRequest().json(json!({"error":e})),
        };
    }

    HttpResponse::Unauthorized().json(json!({"error":"Unauthorized"}))
}
#[post("/executable")]
pub async fn set_executable(
    id: web::Path<String>,
    body: String,
    req: HttpRequest,
) -> impl Responder {
    if let Some(user) = req.extensions().get::<User>() {
        let id_number: u32 = match decode(id.as_str()) {
            Ok(id_number) => id_number[0] as u32,
            Err(_) => return HttpResponse::BadRequest().json(json!({"error":"Invalid ID"})),
        };
        let server = match server_db::get_owned_server_by_id(id_number, user.id) {
            Some(server) => server,
            None => return HttpResponse::NotFound().json(json!({"error":"Server not found"})),
        };
        return match server_db::set_server_executable(server.id, body.as_str()) {
            Ok(_) => HttpResponse::Ok().json(json!({"success":"executable updated"})),
            Err(e) => HttpResponse::BadRequest().json(json!({"error":e})),
        };
    }

    HttpResponse::Unauthorized().json(json!({"error":"Unauthorized"}))
}

#[post("/server-name")]
pub async fn set_name(id: web::Path<String>, body: String, req: HttpRequest) -> impl Responder {
    if let Some(user) = req.extensions().get::<User>() {
        let id_number: u32 = match decode(id.as_str()) {
            Ok(id_number) => id_number[0] as u32,
            Err(_) => return HttpResponse::BadRequest().json(json!({"error":"Invalid ID"})),
        };
        let server = match server_db::get_owned_server_by_id(id_number, user.id) {
            Some(server) => server,
            None => return HttpResponse::NotFound().json(json!({"error":"Server not found"})),
        };
        return match server_db::set_server_name(server.id, body.as_str()) {
            Ok(_) => HttpResponse::Ok().json(json!({"success":"server name updated"})),
            Err(e) => HttpResponse::BadRequest().json(json!({"error":e})),
        };
    }

    HttpResponse::Unauthorized().json(json!({"error":"Unauthorized"}))
}
