use actix_web::{get, post, web, HttpMessage, HttpRequest, HttpResponse, Responder};
use authentication::data::User;
use crypto::hashids::decode;
use log::error;
use serde::Deserialize;
use serde_json::json;
use servers::physical_server::create_server_directory;
use servers::properties::Properties;
use servers::server_db;
use servers::server_db::{BasicHashedServer, HashedServer, Server};
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

        let servers: Vec<BasicHashedServer> = servers
            .iter()
            .map(|s| BasicHashedServer::from_server(s.clone()))
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
        return HttpResponse::Ok().json(HashedServer::from_server(server));
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

        match server_db::set_server_directory(server.id, dir.to_str().unwrap()) {
            Ok(_) => (),
            Err(e) => {
                error!("Failed to set the servers directory to {:?}, {}", &dir, e);
                return HttpResponse::BadRequest().json(json!({"error":e}));
            }
        }

        if let Err(e) = server_db::set_minecraft_version(server.id, &body.minecraft_version) {
            error!("{}", e);
            return HttpResponse::BadRequest().json(json!({"error":e}));
        }
        if let Some(loader_version) = &body.loader_version {
            if let Err(e) = server_db::set_loader(server.id, body.loader, loader_version.as_str()) {
                error!("{}", e);
                return HttpResponse::BadRequest().json(json!({"error":e}));
            }
        }
        match body.loader {
            0 => {
                match minecraft::minecraft_version::download_server_jar(
                    &body.minecraft_version,
                    &dir,
                )
                .await
                {
                    Ok(_) => {
                        if let Err(e) = server_db::set_server_executable(server.id, "server.jar") {
                            error!("{}", e);
                            return HttpResponse::BadRequest().json(json!({"error":e}));
                        }
                    }
                    Err(e) => {
                        error!("{}", e);
                        return HttpResponse::BadRequest().json(
                            json!({"error":format!("Failed to download the server jar: {}", e)}),
                        );
                    }
                }
            },
            1=>todo!("Implement the Fabric loader"),
            2=>todo!("Implement the Forge loader"),
            3=>todo!("Implement the Spigot loader"),
            4=>todo!("Implement the Paper loader"),
            5=>todo!("Implement the BungeeCord loader"),
            6=>todo!("Implement the Quilt loader"),
            7=>todo!("Implement the Mohist loader"),
            _ => (),
        }

        let mut properties: Properties =
            match Properties::new(&Path::join(&dir, Path::new("server.properties"))) {
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


#[get("/supported_loaders")]
pub async fn get_supported_loaders()->impl Responder{
    HttpResponse::Ok().json(servers::supported_loaders::SupportedLoaders::all())
}