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

/// Retrieves servers owned by the authenticated user
#[get("")]
pub async fn get_servers(req: HttpRequest) -> impl Responder {
    // Check if a user is authenticated from the request extensions
    if let Some(user) = req.extensions().get::<User>() {
        // Fetch servers by the user's ID
        let servers = match server_db::get_servers_by_owner(user.id) {
            Ok(s) => s,
            Err(e) => {
                error!("{}", e);
                return HttpResponse::BadRequest().json(json!({"error":e}));
            }
        };

        // Convert the servers to BasicHashedServer format for response
        let servers: Vec<BasicHashedServer> = servers
            .iter()
            .map(|s| BasicHashedServer::from_server(s.clone()))
            .collect();

        // Return the list of servers as JSON response
        return HttpResponse::Ok().json(servers);
    }

    // Return Unauthorized if the user is not authenticated
    HttpResponse::Unauthorized().json(json!({"error":"Unauthorized"}))
}

/// Retrieves a specific server by its ID, ensuring the server is owned by the authenticated user
#[get("")]
pub async fn get_server_by_id(id: web::Path<String>, req: HttpRequest) -> impl Responder {
    // Check if a user is authenticated from the request extensions
    if let Some(user) = req.extensions().get::<User>() {
        // Decode the given ID
        let id_number: u32 = match decode(id.as_str()) {
            Ok(id_number) => id_number[0] as u32,
            Err(_) => return HttpResponse::BadRequest().json(json!({"error":"Invalid ID"})),
        };

        // Fetch the server by the ID and user's ID
        let server = match server_db::get_owned_server_by_id(id_number, user.id) {
            Some(s) => s,
            None => {
                let msg = format!("Server with id: {} not found", id_number);
                error!("{}", msg);
                return HttpResponse::BadRequest().json(json!({"error":msg}));
            }
        };
        // Return the server details as JSON response
        return HttpResponse::Ok().json(HashedServer::from_server(server));
    }

    // Return Unauthorized if the user is not authenticated
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

/// Creates a new server for the authenticated user
#[post("")]
pub async fn create_server(
    req: HttpRequest,
    body: web::Json<CreateServerRequest>,
) -> impl Responder {
    // Check if a user is authenticated from the request extensions
    if let Some(user) = req.extensions().get::<User>() {
        // Create a new server in the database
        let server: Server = match server_db::create_server(body.name.as_str(), user.id) {
            Ok(s) => s,
            Err(e) => {
                error!("{}", e);
                return HttpResponse::BadRequest().json(json!({"error":e}));
            }
        };

        // Create a directory for the server
        let dir = match create_server_directory(server.id) {
            Ok(d) => d,
            Err(e) => {
                error!("{}", e);
                return HttpResponse::BadRequest().json(json!({"error":e}));
            }
        };

        // Set the server's directory in the database
        match server_db::set_server_directory(server.id, dir.to_str().unwrap()) {
            Ok(_) => (),
            Err(e) => {
                error!("Failed to set the servers directory to {:?}, {}", &dir, e);
                return HttpResponse::BadRequest().json(json!({"error":e}));
            }
        }

        // Set the Minecraft version for the server
        if let Err(e) = server_db::set_minecraft_version(server.id, &body.minecraft_version) {
            error!("{}", e);
            return HttpResponse::BadRequest().json(json!({"error":e}));
        }

        // Set the server's loader if provided
        if let Some(loader_version) = &body.loader_version {
            if let Err(e) = server_db::set_loader(server.id, body.loader, loader_version.as_str()) {
                error!("{}", e);
                return HttpResponse::BadRequest().json(json!({"error":e}));
            }
        }

        // Install the server loader if it's not VANILLA
        let loader = loader_manager::from_u8(body.loader)
            .ok_or("Invalid loader")
            .unwrap();
        let loader_version = body.loader_version.clone();
        if loader != loader_manager::Loaders::VANILLA {
            let executable = loader_manager::install_loader(
                loader,
                &body.minecraft_version,
                &dir,
                loader_version,
            )
            .await
            .unwrap();
            // Set the server executable in the database
            if let Err(e) = server_db::set_server_executable(server.id, &executable) {
                error!("{}", e);
                return HttpResponse::BadRequest().json(json!({"error":e}));
            }
        }

        // Create and set server properties
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

        // Save the properties to the file
        match properties.write() {
            Ok(_) => (),
            Err(e) => {
                error!("{}", e);
                return HttpResponse::BadRequest()
                    .json(json!({"Failed to save the properties file: ":e}));
            }
        }

        // Return the created server details as JSON response
        return HttpResponse::Ok().json(HashedServer::from_server(server));
    }

    // Return Unauthorized if the user is not authenticated
    HttpResponse::Unauthorized().json(json!({"error":"Unauthorized"}))
}

/// Installs a specified loader for a server, ensuring the server is owned by the authenticated user
#[post("/install_loader/{version}/{loader}/{loader_version}")]
pub async fn install_loader(
    id: web::Path<String>,
    version: web::Path<String>,
    loader: web::Path<u8>,
    loader_version: web::Path<String>,
    req: HttpRequest,
) -> impl Responder {
    // Check if a user is authenticated from the request extensions
    if let Some(user) = req.extensions().get::<User>() {
        // Decode the given ID
        let id_number: u32 = match decode(id.as_str()) {
            Ok(id_number) => id_number[0] as u32,
            Err(_) => return HttpResponse::BadRequest().json(json!({"error":"Invalid ID"})),
        };

        // Fetch the server by the ID and user's ID
        let server = match server_db::get_owned_server_by_id(id_number, user.id) {
            Some(s) => s,
            None => {
                let msg = format!("Server with id: {} not found", id_number);
                error!("{}", msg);
                return HttpResponse::BadRequest().json(json!({"error":msg}));
            }
        };

        // Install the specified loader if it's not VANILLA
        let loader = loader_manager::from_u8(*loader)
            .ok_or("Invalid loader")
            .unwrap();
        if loader != loader_manager::Loaders::VANILLA {
            let executable = loader_manager::install_loader(
                loader,
                &version,
                &server.clone().directory.unwrap(),
                Some(loader_version.as_ref()),
            )
            .await
            .unwrap();
            // Set the server executable in the database
            if let Err(e) = server_db::set_server_executable(id_number, &executable) {
                error!("{}", e);
                return HttpResponse::BadRequest().json(json!({"error":e}));
            }
        }

        // Return the updated server details as JSON response
        return HttpResponse::Ok().json(HashedServer::from_server(server));
    }

    // Return a bad request if there's an unexpected error
    HttpResponse::BadRequest().json(json!({"message":"Unexpected error has occurred"}))
}
