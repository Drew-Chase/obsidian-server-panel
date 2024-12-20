use actix_web::{delete, get, post, web, HttpMessage, HttpRequest, HttpResponse, Responder};
use actix_web_lab::sse;
use actix_web_lab::sse::Event;
use authentication::data::User;
use crypto::hashids::decode;
use loader_manager::supported_loaders::Loader;
use log::{debug, error, info};
use minecraft::minecraft_version::download_server_jar;
use percent_encoding::percent_decode;
use serde::Deserialize;
use serde_json::json;
use servers::server::Server;
use servers::server_database::ServerDatabase;
use servers::server_filesystem::ServerFilesystem;
use servers::server_process::ServerProcess;
use servers::server_properties::ServerProperties;
use std::collections::HashMap;
use std::convert::{From, Into};
use std::error::Error;
use std::ops::RangeTo;
use std::path::PathBuf;
use std::str::FromStr;
use std::sync::{Arc, Mutex};
use std::thread::sleep;
use std::time::Duration;
use tokio::sync::mpsc::{Receiver, Sender};
use tokio::time::interval;

#[macro_export]
macro_rules! with_authenticated_user_and_server {
    ($req:expr, $id:expr, $action:expr) => {
        if let Some(user) = $req.extensions().get::<User>() {
            let server = Server::get_owned_server_from_string($id.as_ref(), user.id as u64)?;
            $action(server)
        } else {
            return Ok(HttpResponse::Unauthorized().finish());
        }
    };
}

// Retrieves servers owned by the authenticated user
#[get("")]
pub async fn get_servers(req: HttpRequest) -> Result<impl Responder, Box<dyn Error>> {
    // Check if a user is authenticated from the request extensions
    if let Some(user) = req.extensions().get::<User>() {
        let mut servers = Server::get_list_of_owned_servers(user.id as u64)?;

        servers.iter_mut().for_each(|s| s.relativize_paths());

        return Ok(HttpResponse::Ok().json(servers));
    }

    // Return Unauthorized if the user is not authenticated
    Ok(HttpResponse::Unauthorized().json(json!({"error":"Unauthorized"})))
}

// Retrieves a specific server by its ID, ensuring the server is owned by the authenticated user
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
    if let Some(icon_path) = server.get_server_icon() {
        return Ok(HttpResponse::Ok().body(web::Bytes::from(std::fs::read(&icon_path).unwrap())));
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
    java_path: String,
}

// Creates a new server for the authenticated user
#[post("")]
pub async fn create_server(
    req: HttpRequest,
    body: web::Json<CreateServerRequest>,
) -> Result<impl Responder, Box<dyn Error>> {
    // Check if a user is authenticated from the request extensions
    if let Some(user) = req.extensions().get::<User>() {
        // Initialize a new server instance with default values
        let mut server: Server<u64> = Server::default();

        // Assign server properties from the request body
        server.name = body.name.clone();
        server.owner = user.id as u64; // Set the authenticated user as the server owner
        server.loader_type = body.loader.to(); // Map the loader type to its numeric representation
        server.loader_version = body.loader_version.clone();
        server.minecraft_version = body.minecraft_version.clone();
        server.java_runtime = Some(PathBuf::from(body.java_path.clone()));
        server.min_ram = 1;
        server.max_ram = 4;

        // Create the directory for the server, ensuring a valid and unique directory name
        server.create_server_directory()?;

        // Handle the server's start script based on the selected loader type
        match body.loader {
            // For Vanilla servers, download the server JAR file
            Loader::Vanilla => {
                server.start_script = Some(download_server_jar(&body.minecraft_version, &server.directory).await?)
            }
            // For other loaders, delegate installation to the loader manager and get the start script
            _ => {
                server.start_script = Some(PathBuf::from(
                    loader_manager::install_loader(
                        body.loader.clone(),
                        &body.minecraft_version,
                        &server.directory,
                        body.loader_version.clone(),
                    )
                    .await?,
                ))
            }
        }

        // Generate a `server.properties` file in the server directory
        server.create_properties_file()?;

        // Set additional configuration properties using a range of key-value pairs from the request body
        server.set_property_range(
            [
                ("server-port", body.port.to_string()),        // Server port number
                ("difficulty", body.difficulty.clone()),       // Difficulty level
                ("gamemode", body.gamemode.clone()),           // Game mode
                ("hardcore", body.hardcore.to_string()),       // Hardcore mode status (true/false)
                ("max-players", body.max_players.to_string()), // Maximum allowed players
            ]
            .into_iter()
            .map(|(k, v)| (k.to_string(), v)) // Convert keys and values into a String format
            .collect(), // Collect into a HashMap
        )?;

        // Calculate the total size of the server directory and update the server object
        server.calculate_server_size();

        // Persist the server information to the database
        server.add()?;

        // Respond with the created server details in JSON format
        return Ok(HttpResponse::Ok().json(server));
    }

    // Respond with an "Unauthorized" error if the user is not authenticated
    Ok(HttpResponse::Unauthorized().json(json!({"error":"Unauthorized"})))
}

// Deletes a server by its ID, ensuring the server is owned by the authenticated user
#[delete("")]
pub async fn delete_server(id: web::Path<String>, req: HttpRequest) -> Result<impl Responder, Box<dyn Error>> {
    // Check if a user is authenticated from the request extensions
    if let Some(user) = req.extensions().get::<User>() {
        // Try to decode the given ID string into a `u64` integer
        let id_number: u64 = match decode(id.as_str()) {
            // On successful decoding, extract the first ID from the decoded vector
            Ok(id_number) => id_number[0],
            // If decoding fails, return a `BadRequest` response with an error message
            Err(_) => return Ok(HttpResponse::BadRequest().json(json!({"error":"Invalid ID"}))),
        };

        // Retrieve the server owned or accessible by the user using the decoded ID
        let server = Server::get_owned_server(id_number, user.id as u64)?;

        // Delete the server from both the database and the filesystem
        server.delete()?;

        // If deletion is successful, return an HTTP response with no content
        return Ok(HttpResponse::Ok().finish());
    }

    // If no authenticated user is found, return an `Unauthorized` response with an error message
    Ok(HttpResponse::Unauthorized().json(json!({"error":"Unauthorized"})))
}

// Installs a specified loader for a server, ensuring the server is owned by the authenticated user
#[post("/install_loader/{version}/{loader}/{loader_version}")]
pub async fn install_loader(
    id: web::Path<String>,
    version: web::Path<String>,
    loader: web::Path<String>,
    loader_version: web::Path<String>,
    req: HttpRequest,
) -> Result<impl Responder, Box<dyn Error>> {
    // Check if a user is authenticated from the request extensions
    if let Some(user) = req.extensions().get::<User>() {
        // Decode the given ID
        let id_number = match decode(id.as_str()) {
            Ok(id_number) => id_number[0],
            Err(_) => return Ok(HttpResponse::BadRequest().json(json!({"error":"Invalid ID"}))),
        };

        // Fetch the server by the ID and user's ID
        let mut server = Server::get_owned_server(id_number, user.id as u64)?;

        // Install the specified loader if it's not VANILLA
        let loader = match Loader::from_str(loader.as_ref().as_str()) {
            Ok(l) => l,
            Err(_) => {
                let msg = format!("Loader {} not found", loader);
                error!("{}", msg);
                return Ok(HttpResponse::BadRequest().json(json!({"error":msg})));
            }
        };
        if loader != Loader::Vanilla {
            server.start_script = Some(PathBuf::from(
                loader_manager::install_loader(
                    loader,
                    &version,
                    &server.clone().directory,
                    Some(loader_version.as_ref()),
                )
                .await?,
            ));
        } else {
            return Ok(HttpResponse::BadRequest()
                .json(json!({"message":"To install vanilla version, use the /install_minecraft endpoint"})));
        }

        // Return the updated server details as JSON response
        return Ok(HttpResponse::Ok().json(server));
    }

    // Return a bad request if there's an unexpected error
    Ok(HttpResponse::BadRequest().json(json!({"message":"Unexpected error has occurred"})))
}

#[post("/install_minecraft/{version}")]
pub async fn install_minecraft(
    id: web::Path<String>,
    version: web::Path<String>,
    req: HttpRequest,
) -> Result<impl Responder, Box<dyn Error>> {
    if let Some(user) = req.extensions().get::<User>() {
        // Decode the given ID
        let id_number = match decode(id.as_str()) {
            Ok(id_number) => id_number[0],
            Err(_) => return Ok(HttpResponse::BadRequest().json(json!({"error":"Invalid ID"}))),
        };

        // Fetch the server by the ID and user's ID
        let mut server = Server::get_owned_server(id_number, user.id as u64)?;
        download_server_jar(version.as_ref(), server.directory.clone()).await?;
        server.minecraft_version = version.clone();
        server.update()?;
        return Ok(HttpResponse::Ok().json(server));
    }
    // Return a bad request if there's an unexpected error
    Ok(HttpResponse::BadRequest().json(json!({"message":"Unexpected error has occurred"})))
}

#[post("/settings")]
pub async fn set_setting(id: web::Path<String>, req: HttpRequest) -> Result<impl Responder, Box<dyn Error>> {
    if let Some(user) = req.extensions().get::<User>() {
        let parameters: HashMap<String, String> = req
            .query_string()
            .split('&')
            .filter_map(|s| s.split_once('='))
            .map(|(k, v)| {
                (
                    k.to_string(),
                    percent_decode(v.to_string().as_bytes())
                        .decode_utf8()
                        .unwrap()
                        .to_string(),
                )
            })
            .collect();

        debug!("Parameters: {:?}", parameters);

        let id_number = decode(id.as_str()).map(|id_number| id_number[0])?;
        let mut server = Server::get_owned_server(id_number, user.id as u64)?;

        // Attempt to get the "name" parameter from the parameters map.
        // If it exists, update the server's name with its value.
        if let Some(v) = parameters.get("name") {
            server.name = v.clone();
        }

        // Try to retrieve the "min-ram" parameter from the parameters map.
        // If the value can be parsed into a `u64`, update the server's min_ram attribute.
        if let Some(v) = parameters.get("min-ram").and_then(|v| u64::from_str(v).ok()) {
            server.min_ram = v;
            println!("Min RAM: {}", server.min_ram);
        }

        // Attempt to retrieve the "max-ram" parameter from the `parameters` map.
        // If the value exists, try to parse it as an unsigned 64-bit integer (`u64`).
        if let Some(v) = parameters.get("max-ram").and_then(|v| u64::from_str(v).ok()) {
            server.max_ram = v;
            println!("Min RAM: {}", server.max_ram);
        }

        // Retrieve the "auto-start" parameter, if present, convert it to lowercase,
        // and check if it equals "true". Update the server's auto_start flag accordingly.
        if let Some(v) = parameters.get("auto-start") {
            server.auto_start = v.to_lowercase() == "true";
        }

        // Check for the "start-script" parameter.
        // If present, convert it to a `PathBuf` and update the server's start_script value.
        if let Some(v) = parameters.get("start-script") {
            server.start_script = Some(PathBuf::from(v));
        }

        // Attempt to get the "minecraft-arguments" parameter.
        // If it exists, clone the value and update the server's minecraft_arguments attribute.
        if let Some(v) = parameters.get("minecraft-arguments") {
            server.minecraft_arguments = Some(v.clone());
        }

        // Check for the "java-arguments" parameter.
        // If found, clone the value and update the server's java_arguments attribute.
        if let Some(v) = parameters.get("java-arguments") {
            server.java_arguments = Some(v.clone());
        }

        server.update()?;
        return Ok(HttpResponse::Ok().finish());
    }
    Ok(HttpResponse::Unauthorized().json(json!({"message":"User not authenticated"})))
}

#[post("/start")]
pub async fn start_server(id: web::Path<String>, req: HttpRequest) -> Result<impl Responder, Box<dyn Error>> {
    if let Some(user) = req.extensions().get::<User>() {
        let id = decode(id.as_str()).map(|id_number| id_number[0])?;
        let mut server = Server::get_owned_server(id, user.id as u64)?;
        server.start_server()?;
        return Ok(HttpResponse::Ok().finish());
    }
    Ok(HttpResponse::Unauthorized().finish())
}

#[post("/send-command")]
pub async fn send_command(
    id: web::Path<String>,
    body: String,
    req: HttpRequest,
) -> Result<impl Responder, Box<dyn Error>> {
    if let Some(user) = req.extensions().get::<User>() {
        let server = Server::get_owned_server_from_string(id.as_ref(), user.id as u64)?;
        server.send_command_to_server(body)?;
        return Ok(HttpResponse::Ok().finish());
    }
    Ok(HttpResponse::Unauthorized().finish())
}

#[get("/console")]
pub async fn get_server_console(
    id: web::Path<String>,
    log_file: web::Path<String>,
    req: HttpRequest,
) -> Result<impl Responder, Box<dyn Error>> {
    if let Some(user) = req.extensions().get::<User>() {
        let server = Server::get_owned_server_from_string(id.as_ref(), user.id as u64)?;
        let output = server.read_log_file(log_file.as_ref(), |_| false)?;
        return Ok(HttpResponse::Ok().content_type("text/plain").body(output));
    }

    Ok(HttpResponse::Unauthorized().finish())
}

#[get("/console/sse")]
pub async fn get_server_console_sse(
    id: web::Path<String>,
    query: web::Query<HashMap<String, String>>,
    req: HttpRequest,
) -> Result<impl Responder, Box<dyn Error>> {
    let query = query.0;
    let log_file = query.get("log_file").cloned().unwrap_or_default();
    let (sender, receiver) = tokio::sync::mpsc::channel(2);
    if let Some(user) = req.extensions().get::<User>() {
        let server = Server::get_owned_server_from_string(id.as_ref(), user.id as u64)?;

        actix_web::rt::spawn(async move {
            server.read_log_file(log_file, move |line| {
                let msg = sse::Data::new(line).event("update_console");
                info!("Sending message: {}", line);
                if sender.try_send(msg.into()).is_err() {
                    return false;
                }

                true
            })
        });
    }
    Ok(sse::Sse::from_infallible_receiver(receiver).with_keep_alive(Duration::from_secs(3)))
}

#[get("/state/sse")]
pub async fn get_server_state_updates(
    id: web::Path<String>,
    req: HttpRequest,
) -> Result<impl Responder, Box<dyn Error>> {
    let (sender, receiver) = tokio::sync::mpsc::channel(2);

    if let Some(user) = req.extensions().get::<User>() {
        let server = Server::get_owned_server_from_string(id.as_ref(), user.id as u64)?;

        let user_id = user.id as u64;
        actix_web::rt::spawn(async move {
            let last_recorded_server_struct = Arc::new(Mutex::new(server));
            let mut ticker = interval(Duration::from_secs(1));
            loop {
                if let Ok(server) = Server::get_owned_server_from_string(id.as_ref(), user_id) {
                    if let Ok(mut last_recorded_server_struct) = last_recorded_server_struct.lock() {
                        let last_recorded_server = last_recorded_server_struct.clone();
                        if last_recorded_server != server {
                            *last_recorded_server_struct = server.clone();
                            let msg = sse::Data::new(serde_json::to_string(&server).unwrap()).event("update_state");

                            // Try sending, break the loop if the receiver is dropped
                            if sender.send(msg.into()).await.is_err() {
                                // Receiver disconnected, exit the task
                                drop(sender);
                                break;
                            }
                        } else {
                            let msg = sse::Data::new("").event("ping");

                            // Try sending, break the loop if the receiver is dropped
                            if sender.send(msg.into()).await.is_err() {
                                // Receiver disconnected, exit the task
                                drop(sender);
                                break;
                            }
                        }
                    }
                }

                // Wait for the next tick
                ticker.tick().await;
            }
        });
    }
    Ok(sse::Sse::from_infallible_receiver(receiver).with_keep_alive(Duration::from_secs(3)))
}
