use actix_web::{get, post, web, HttpMessage, HttpRequest, HttpResponse, Responder};
use authentication::data::User;
use log::error;
use serde::Deserialize;
use serde_json::json;
use servers::physical_server::create_server_directory;
use servers::properties::Properties;
use servers::server_db;
use servers::server_db::Server;
use std::path::Path;

#[get("/")]
pub async fn get_servers() -> impl Responder {
    HttpResponse::Ok()
}

#[get("/{id}")]
pub async fn get_server_by_id(id: web::Path<String>) -> impl Responder {
    HttpResponse::Ok()
}

#[derive(Deserialize)]
struct CreateServerRequest {
    name: String,
    port: u16,
    difficulty: String,
    gamemode: String,
    hardcore: bool,
    max_players: u16,
    minecraft_version: String,
    loader: u8,
    loader_version: Option<String>,
}
#[post("/")]
pub async fn create_server(
    req: HttpRequest,
    body: web::Json<CreateServerRequest>,
) -> impl Responder {
    if let Some(user) = req.extensions().get::<User>() {
        let server: Server = match server_db::create_server(body.name.as_str(), user.id) {
            Ok(s) => s,
            Err(e) => {
                error!("{}", e);
                return HttpResponse::BadRequest().json(json!({"error":e}));
            }
        };

        let dir = match create_server_directory(server.id) {
            Ok(d) => d,
            Err(e) => {
                error!("{}", e);
                return HttpResponse::BadRequest().json(json!({"error":e}));
            }
        };

        let mut properties: Properties =
            match Properties::new(&Path::join(&*dir, Path::new("server.properties"))) {
                Ok(p) => p,
                Err(e) => {
                    error!("{}", e);
                    return HttpResponse::BadRequest().json(json!({"error":e}));
                }
            };

        properties.set("server-port", &body.port.to_string());
        properties.set("difficulty", &body.difficulty);
        properties.set("gamemode", &body.gamemode);
        properties.set("hardcore", &body.hardcore.to_string());
        properties.set("max-players", &body.max_players.to_string());

        return HttpResponse::Ok().json(server);
    }

    HttpResponse::Unauthorized().json(json!({"error":"Unauthorized"}))
}
