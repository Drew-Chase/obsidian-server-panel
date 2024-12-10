use crate::WWWROOT;
use actix_web::{delete, get, post, web, HttpMessage, HttpRequest, HttpResponse, Responder};
use authentication::data::User;
use common_lib::traits::TransformPath;
use crypto::hashids::decode;
use loader_manager::supported_loaders::Loader;
use log::error;
use minecraft::minecraft_version::download_server_jar;
use serde::Deserialize;
use serde_json::json;
use servers::physical_server::create_server_directory;
use servers::properties::Properties;
use servers::server::Server;
use servers::server_database::ServerDatabase;
use servers::server_db;
use servers::server_db::{BasicHashedServer, HashedServer, Server};
use std::error::Error;
use std::path::Path;
use std::str::FromStr;
use servers::server_filesystem::ServerFilesystem;

/// Retrieves servers owned by the authenticated user
#[get("")]
pub async fn get_servers(req: HttpRequest) -> Result<impl Responder, Box<dyn Error>> {
    // Check if a user is authenticated from the request extensions
    if let Some(user) = req.extensions().get::<User>() {
        let server = Server::get_list_of_owned_servers(user.id as u64)?;
        return Ok(HttpResponse::Ok().json(server));
    }

    // Return Unauthorized if the user is not authenticated
    Ok(HttpResponse::Unauthorized().json(json!({"error":"Unauthorized"})))
}

/// Retrieves a specific server by its ID, ensuring the server is owned by the authenticated user
#[get("")]
pub async fn get_server_by_id(id: web::Path<String>, req: HttpRequest) -> Result<impl Responder, Box<dyn Error>> {
    // Check if a user is authenticated from the request extensions
    if let Some(user) = req.extensions().get::<User>() {
        // Decode the given ID
        let id_number: u64 = match decode(id.as_str()) {
            Ok(id_number) => id_number[0],
            Err(_) => return Ok(HttpResponse::BadRequest().json(json!({"error":"Invalid ID"}))),
        };

        // Fetch the server by the ID and user's ID
        let server = Server::get_owned_server(id_number, user.id as u64)?;

        // Return the server details as JSON response
        return Ok(HttpResponse::Ok().json(server));
    }

    // Return Unauthorized if the user is not authenticated
    Ok(HttpResponse::Unauthorized().json(json!({"error":"Unauthorized"})))
}

#[get("icon")]
pub async fn get_server_icon(id: web::Path<String>) -> Result<impl Responder, Box<dyn Error>> {
    let id_number = match decode(id.as_str()) {
        Ok(id_number) => id_number[0],
        Err(_) => return Ok(HttpResponse::BadRequest().json(json!({"error":"Invalid ID"}))),
    };

    let server = Server::get_server(id_number)?;

    let icon_path = server.directory.join(Path::new("server-icon.png"));
    if icon_path.exists() {
        return Ok(HttpResponse::Ok().body(web::Bytes::from(std::fs::read(icon_path).unwrap())));
    }
    Ok(HttpResponse::NotFound().finish())
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
    loader: Loader,
    loader_version: Option<String>,
}

/// Creates a new server for the authenticated user
#[post("")]
pub async fn create_server(req: HttpRequest, body: web::Json<CreateServerRequest>) -> Result<impl Responder, Box<dyn Error>> {
    // Check if a user is authenticated from the request extensions
    if let Some(user) = req.extensions().get::<User>() {
        // Create a new server in the database
        let mut server: Server<u64> = Server::default();
        server.name = body.name.clone();
        server.owner = user.id as u64;
        server.loader_type = body.loader.to();
        server.loader_version = body.loader_version.clone();
        server.minecraft_version = body.minecraft_version.clone();
        
        server.create_server_directory()?;
        server.add()?;

        // Set the Minecraft version for the server
        if let Err(e) = server_db::set_minecraft_version(server.id, &body.minecraft_version) {
            error!("{}", e);
            return HttpResponse::BadRequest().json(json!({"error":e}));
        }

        // Set the server's loader if provided
        if let Some(loader_version) = &body.loader_version {
            if let Err(e) = server_db::set_loader(server.id, body.loader.to(), loader_version.as_str()) {
                error!("{}", e);
                return HttpResponse::BadRequest().json(json!({"error":e}));
            }
        }

        let loader_version = body.loader_version.clone();
        if body.loader != Loader::Vanilla {
            let executable = loader_manager::install_loader(body.loader.clone(), &body.minecraft_version, &dir, loader_version).await.unwrap();
            // Set the server executable in the database
            if let Err(e) = server_db::set_server_executable(server.id, &executable) {
                error!("{}", e);
                return HttpResponse::BadRequest().json(json!({"error":e}));
            }
        } else {
            let path = match download_server_jar(&body.minecraft_version, &dir).await {
                Ok(p) => p,
                Err(e) => {
                    error!("{}", e);
                    return HttpResponse::BadRequest().json(json!({"error":e.to_string()}));
                }
            };
            if let Err(e) = server_db::set_server_executable(server.id, path.to_str().unwrap()) {
                error!("{}", e);
                return HttpResponse::BadRequest().json(json!({"error":e}));
            }
        }

        // Create and set server properties
        let mut properties: Properties = match Properties::new(&Path::join(&dir, Path::new("server.properties"))) {
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
                return HttpResponse::BadRequest().json(json!({"Failed to save the properties file: ":e}));
            }
        }

        // Return the created server details as JSON response
        return HttpResponse::Ok().json(HashedServer::from_server(server));
    }

    // Return Unauthorized if the user is not authenticated
    HttpResponse::Unauthorized().json(json!({"error":"Unauthorized"}))
}

/// Deletes a server by its ID, ensuring the server is owned by the authenticated user
#[delete("")]
pub async fn delete_server(id: web::Path<String>, req: HttpRequest) -> impl Responder {
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

        match std::fs::remove_dir_all(server.directory.clone().unwrap()) {
            Ok(_) => (),
            Err(e) => {
                error!("{}", e);
                return HttpResponse::BadRequest().json(json!({"error":"Failed to delete server directory"}));
            }
        }

        // Delete the server directory
        if let Err(e) = server_db::delete_server(server.id) {
            error!("{}", e);
            return HttpResponse::BadRequest().json(json!({"error":e}));
        }

        // Return the deleted server details as JSON response
        return HttpResponse::Ok().json(HashedServer::from_server(server));
    }

    // Return Unauthorized if the user is not authenticated
    HttpResponse::Unauthorized().json(json!({"error":"Unauthorized"}))
}

/// Installs a specified loader for a server, ensuring the server is owned by the authenticated user
#[post("/install_loader/{version}/{loader}/{loader_version}")]
pub async fn install_loader(id: web::Path<String>, version: web::Path<String>, loader: web::Path<String>, loader_version: web::Path<String>, req: HttpRequest) -> impl Responder {
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
        let loader = match Loader::from_str(loader.as_ref().as_str()) {
            Ok(l) => l,
            Err(_) => {
                let msg = format!("Loader {} not found", loader);
                error!("{}", msg);
                return HttpResponse::BadRequest().json(json!({"error":msg}));
            }
        };
        if loader != Loader::Vanilla {
            let executable = loader_manager::install_loader(loader, &version, &server.clone().directory.unwrap(), Some(loader_version.as_ref())).await.unwrap();
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
