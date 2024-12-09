#![deny(clippy::unwrap_used)]
#![deny(clippy::expect_used)]
#![deny(clippy::panic)]
#![deny(unused_must_use)]

use actix_web::dev::Service;
use std::path::PathBuf;
use std::process::{exit, Child, ExitStatus};
mod auth_middleware;
mod authentication_endpoint;
mod backups_endpoint;
mod configuration_endpoint;
mod file_system_endpoint;
mod instance_endpoint;
mod java_endpoint;
mod loader_endpoint;
mod minecraft_endpoint;
mod notifications_endpoint;
mod server_endpoint;
mod server_properties_endpoint;
mod server_settings_endpoint;
mod system_stats_endpoint;

use actix_files::file_extension_to_mime;
use actix_web::error::ErrorInternalServerError;
use actix_web::http::header;
use actix_web::{
    error, get, middleware, rt, web, App, Error, HttpRequest, HttpResponse, HttpServer, Responder,
};
use actix_ws::AggregatedMessage;
use awc::Client;
use configuration::config::CONFIG;
use futures_util::stream::StreamExt;
use include_dir::{include_dir, Dir};
use log::{debug, error, info};
use network_utility::{close_all_ports, open_port};
use scheduler::{start_ticking_schedules, stop_ticking_schedules};
use serde_json::json;
use std::sync::Mutex;
use sysinfo::System;

const DEBUG: bool = cfg!(debug_assertions);

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    std::env::set_var("RUST_LOG", "debug");
    env_logger::init();
    start_ticking_schedules!();
    let port = CONFIG.port; // Port to listen on

    match authentication::initialize() {
        Ok(_) => info!("Authentication initialized"),
        Err(e) => error!("Failed to initialize authentication: {}", e),
    }

    match servers::server_database::initialize_server_database() {
        Ok(_) => debug!("Server database initialized"),
        Err(e) => {
            error!("Failed to initialize the servers database: {}", e);
            exit(1);
        }
    }
    backups::initialize();

    if CONFIG.port_forward_webui {
        // This will open the webui port on the router using upnp
        // Spawn a thread to refresh the upnp port every 5 minutes
        open_port!(port, "Obsidian Minecraft Server Manager");
    }

    let sys = web::Data::new(Mutex::new(System::new_all()));
    let server = HttpServer::new(move || {
        let app = App::new()
            .wrap(middleware::Logger::default())
            .wrap(auth_middleware::AuthMiddleware)
            .wrap_fn(|req, srv| {
                let fut = srv.call(req);
                async {
                    let mut res = fut.await?;
                    res.headers_mut()
                        .insert(header::ACCESS_CONTROL_ALLOW_ORIGIN, "*".parse().unwrap());
                    res.headers_mut()
                        .insert(header::ACCESS_CONTROL_ALLOW_HEADERS, "*".parse().unwrap());
                    Ok(res)
                }
            })
            .app_data(
                web::JsonConfig::default()
                    .limit(4096)
                    .error_handler(|err, _req| {
                        let error = json!({ "error": format!("{}", err) });
                        error::InternalError::from_response(
                            err,
                            HttpResponse::BadRequest().json(error),
                        )
                        .into()
                    }),
            )
            // Handle API routes here
            .service(
                web::scope("api")
                    .service(status)
                    .service(
                        web::scope("auth")
                            .service(authentication_endpoint::create_user)
                            .service(authentication_endpoint::login)
                            .service(authentication_endpoint::login_with_token)
                            .service(authentication_endpoint::generate_access_token)
                            .service(authentication_endpoint::get_access_tokens)
                            .service(authentication_endpoint::check_if_access_token_exists)
                            .service(authentication_endpoint::get_users),
                    )
                    .service(
                        web::scope("minecraft")
                            .service(minecraft_endpoint::get_minecraft_versions)
                            .service(minecraft_endpoint::get_latest_release)
                            .service(minecraft_endpoint::get_latest_snapshot)
                            .service(minecraft_endpoint::get_version_by_id)
                            .service(minecraft_endpoint::get_releases)
                            .service(minecraft_endpoint::get_snapshots)
                            .service(minecraft_endpoint::get_java_version_by_minecraft_version),
                    )
                    .service(
                        web::scope("java")
                            .service(java_endpoint::get_java_versions)
                            .service(java_endpoint::get_installation_files)
                            .service(java_endpoint::uninstall_java_version)
                            .service(java_endpoint::install_java_version),
                    )
                    .service(
                        web::scope("system")
                            .service(system_stats_endpoint::get_system_info)
                            .service(system_stats_endpoint::get_system_usage)
                            .service(system_stats_endpoint::get_system_usage_sse)
                            .service(system_stats_endpoint::get_storage_info)
                            .app_data(sys.clone()),
                    )
                    .service(
                        web::scope("server")
                            .service(web::scope("files").service(file_system_endpoint::get_files))
                            .service(server_endpoint::get_servers)
                            .service(server_endpoint::create_server)
                            .service(
                                web::scope("{id}")
                                    .service(
                                        web::scope("properties")
                                            .service(
                                                server_properties_endpoint::get_server_properties,
                                            )
                                            .service(
                                                server_properties_endpoint::set_server_property,
                                            ),
                                    )
                                    .service(
                                        web::scope("settings")
                                            .service(server_settings_endpoint::get_server_settings)
                                            .service(server_settings_endpoint::set_java_arguments)
                                            .service(
                                                server_settings_endpoint::set_minecraft_arguments,
                                            )
                                            .service(server_settings_endpoint::set_memory_max)
                                            .service(server_settings_endpoint::set_memory_min)
                                            .service(server_settings_endpoint::set_executable)
                                            .service(server_settings_endpoint::set_name),
                                    )
                                    .service(
                                        web::scope("files")
                                            .service(file_system_endpoint::get_server_files)
                                            .service(file_system_endpoint::upload_file_to_server)
                                            .service(file_system_endpoint::download_file)
                                            .service(file_system_endpoint::create_directory)
                                            .service(file_system_endpoint::create_file)
                                            .service(file_system_endpoint::delete_path),
                                    )
                                    .service(
                                        web::scope("backups")
                                            .service(backups_endpoint::get_backups)
                                            .service(backups_endpoint::create_manual_backup),
                                    )
                                    .service(server_endpoint::get_server_by_id)
                                    .service(server_endpoint::delete_server)
                                    .service(server_endpoint::get_server_icon),
                            ),
                    )
                    .service(web::scope("instances").service(instance_endpoint::discover_modpacks))
                    .service(
                        web::scope("loaders")
                            .service(loader_endpoint::get_supported_loaders)
                            .service(loader_endpoint::get_loader_by_id),
                    ),
            );
        // Add conditional routing based on the config
        if DEBUG {
            app.default_service(web::route().to(proxy_to_vite))
                .service(web::resource("/assets/{file:.*}").route(web::get().to(proxy_to_vite)))
                .service(
                    web::resource("/node_modules/{file:.*}").route(web::get().to(proxy_to_vite)),
                )
        } else {
            app.default_service(web::route().to(index))
                .service(web::scope("assets/{file}").service(assets))
        }
    })
    .workers(4)
    .bind(format!("0.0.0.0:{port}", port = port))?
    .run();
    info!(
        "Starting {} server at http://127.0.0.1:{}...",
        if DEBUG { "development" } else { "production" },
        port
    );

    if DEBUG {
        start_vite_server().expect("Failed to start vite server");
    }

    let stop_result = server.await;
    debug!("Server stopped");

    // Stop all schedules
    // This is necessary to prevent the server from hanging on shutdown
    // because the schedules are still running
    // This is probably not necessary in a production environment
    stop_ticking_schedules!();

    // Closes all open ports
    close_all_ports!();
    stop_result
}

fn start_vite_server() -> Result<Child, Box<dyn std::error::Error>> {
    #[cfg(target_os = "windows")]
    let find_cmd = "where";
    #[cfg(not(target_os = "windows"))]
    let find_cmd = "which";

    let vite = std::process::Command::new(find_cmd)
        .arg("vite")
        .stdout(std::process::Stdio::piped())
        .output()?
        .stdout;

    let vite = String::from_utf8(vite);
    let vite = vite.unwrap();
    let vite = vite.as_str().trim();

    if vite.is_empty() {
        error!("vite not found, make sure its installed with npm install -g vite");
        return Err(std::io::Error::new(
            std::io::ErrorKind::NotFound,
            "vite not found",
        ))?;
    }

    // Get the first occurrence
    let vite = vite
        .split("\n")
        .collect::<Vec<_>>()
        .last()
        .expect("Failed to get vite executable")
        .trim();

    debug!("found vite at: {:?}", vite);

    // Start the vite server
    Ok(std::process::Command::new(vite)
        .current_dir(r#"../../"#)
        .spawn()
        .expect("Failed to start vite server"))
}

/// The maximum payload size allowed for forwarding requests and responses.
///
/// This constant defines the maximum size (in bytes) for the request and response payloads
/// when proxying. Any payload exceeding this size will result in an error.
///
/// Currently, it is set to 1 GB.
const MAX_PAYLOAD_SIZE: usize = 1024 * 1024 * 1024; // 1 GB

/// Static directory including all files under `target/wwwroot`.
///
/// This static directory is used to embed files into the binary at compile time.
/// The `WWWROOT` directory will be used to serve static files such as `index.html`.
pub static WWWROOT: Dir = include_dir!("target/wwwroot");
/// Handles the request for the index.html file.
///
/// This function serves the `index.html` file from the embedded directory
/// if it exists, and returns an internal server error if the file is not found.
///
/// # Arguments
///
/// * `_req` - The HTTP request object.
///
/// # Returns
///
/// An `impl Responder` which can either be a successful HTTP response containing
/// the `index.html` file, or an internal server error.
async fn index(_req: HttpRequest) -> Result<impl Responder, Error> {
    if let Some(file) = WWWROOT.get_file("index.html") {
        let body = file.contents();
        return Ok(HttpResponse::Ok().content_type("text/html").body(body));
    }
    Err(ErrorInternalServerError("Failed to find index.html"))
}

#[get("")]
async fn assets(file: web::Path<String>) -> impl Responder {
    if let Some(file) = WWWROOT.get_file(format!("assets/{}", file.as_str())) {
        let body = file.contents();
        return Ok(HttpResponse::Ok()
            .content_type(file_extension_to_mime(
                file.path().extension().unwrap().to_str().unwrap(),
            ))
            .body(body));
    }
    Err(ErrorInternalServerError(format!("Failed to find {}", file)))
}

/// Proxies requests to the Vite development server.
///
/// This function forwards incoming requests to a local Vite server running on port 3000.
/// It buffers the entire request payload and response payload to avoid partial transfers.
/// Requests and responses larger than the maximum payload size will result in an error.
///
/// # Arguments
///
/// * `req` - The HTTP request object.
/// * `payload` - The request payload.
///
/// # Returns
///
/// An `HttpResponse` which contains the response from the Vite server,
/// or an error response in case of failure.
async fn proxy_to_vite(req: HttpRequest, mut payload: web::Payload) -> Result<HttpResponse, Error> {
    let client = Client::new();
    let forward_url = format!("http://localhost:3000{}", req.uri());

    // Buffer the entire payload
    let mut body_bytes = web::BytesMut::new();
    while let Some(chunk) = payload.next().await {
        let chunk = chunk?;
        if (body_bytes.len() + chunk.len()) > MAX_PAYLOAD_SIZE {
            return Err(error::ErrorPayloadTooLarge("Payload overflow"));
        }
        body_bytes.extend_from_slice(&chunk);
    }

    let mut forwarded_resp = client
        .request_from(forward_url.as_str(), req.head())
        .no_decompress()
        .send_body(body_bytes)
        .await
        .map_err(|err| ErrorInternalServerError(format!("Failed to forward request: {}", err)))?;

    // Buffer the entire response body
    let mut resp_body_bytes = web::BytesMut::new();
    while let Some(chunk) = forwarded_resp.next().await {
        let chunk = chunk?;
        if (resp_body_bytes.len() + chunk.len()) > MAX_PAYLOAD_SIZE {
            return Err(error::ErrorPayloadTooLarge("Response payload overflow"));
        }
        resp_body_bytes.extend_from_slice(&chunk);
    }

    // Build the response
    let mut res = HttpResponse::build(forwarded_resp.status());

    // Copy headers
    for (header_name, header_value) in forwarded_resp.headers().iter() {
        res.insert_header((header_name.clone(), header_value.clone()));
    }

    Ok(res.body(resp_body_bytes))
}

/// Handles requests to check the server status.
///
/// This endpoint responds to GET requests with a JSON object indicating
/// that the server is running correctly. It can be used for health checks
/// or monitoring server status.
///
/// # Returns
///
/// A JSON object with a `status` field set to "ok".
#[get("/")]
async fn status(req: HttpRequest, stream: web::Payload) -> Result<HttpResponse, Error> {
    let (res, mut session, stream) = actix_ws::handle(&req, stream)?;

    let mut stream = stream
        .aggregate_continuations()
        // aggregate continuation frames up to 1MiB
        .max_continuation_size(2_usize.pow(20));

    // start task but don't wait for it
    rt::spawn(async move {
        // receive messages from websocket
        while let Some(msg) = stream.next().await {
            match msg {
                Ok(AggregatedMessage::Text(text)) => {
                    // echo text message
                    session.text(text).await.unwrap();
                }

                Ok(AggregatedMessage::Binary(bin)) => {
                    // echo binary message
                    session.binary(bin).await.unwrap();
                }

                Ok(AggregatedMessage::Ping(msg)) => {
                    // respond to PING frame with PONG frame
                    session.pong(&msg).await.unwrap();
                }

                _ => {}
            }
        }
    });

    // respond immediately with response connected to WS session
    Ok(res)
}
