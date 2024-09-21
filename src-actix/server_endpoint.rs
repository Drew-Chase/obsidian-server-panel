use actix_web::{get, post, web, HttpMessage, HttpRequest, HttpResponse, Responder};
use authentication::data::User;
use crypto::hashids::decode;
use log::error;
use serde::Deserialize;
use serde_json::json;
use servers::physical_server::create_server_directory;
use servers::properties::Properties;
use servers::server_db::{HashedServer, Server};
use servers::{physical_server, server_db};
use std::path::Path;

#[get("/")]
pub async fn get_servers(req: HttpRequest) -> impl Responder {
    if let Some(user) = req.extensions().get::<User>() {
        let servers = match server_db::get_servers_by_owner(user.id) {
            Ok(s) => s,
            Err(e) => {
                error!("{}", e);
                return HttpResponse::BadRequest().json(json!({"error":e}));
            }
        };

        let servers: Vec<HashedServer> = servers
            .iter()
            .map(|s| HashedServer::from_server(s.clone()))
            .collect();

        return HttpResponse::Ok().json(servers);
    }

    HttpResponse::Unauthorized().json(json!({"error":"Unauthorized"}))
}

#[get("/")]
pub async fn get_server_by_id(id: web::Path<String>, req: HttpRequest) -> impl Responder {
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
            return HttpResponse::Ok().json(HashedServer::from_server(server));
        }
    }

    HttpResponse::Unauthorized().json(json!({"error":"Unauthorized"}))
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

        match server_db::set_server_directory(server.id, &dir.to_str().unwrap()) {
            Ok(_) => (),
            Err(e) => {
                error!("Failed to set the servers directory to {:?}, {}", &dir, e);
                return HttpResponse::BadRequest().json(json!({"error":e}));
            }
        }

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

        match properties.write() {
            Ok(_) => (),
            Err(e) => {
                error!("{}", e);
                return HttpResponse::BadRequest()
                    .json(json!({"Failed to save the properties file: ":e}));
            }
        }

        return HttpResponse::Ok().json(HashedServer::from_server(server));
    }

    HttpResponse::Unauthorized().json(json!({"error":"Unauthorized"}))
}
