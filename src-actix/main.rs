mod auth_middleware;
mod authentication_endpoint;
mod file_system_endpoint;
mod java_endpoint;
mod minecraft_endpoint;
mod server_endpoint;
mod server_properties_endpoint;
mod server_settings_endpoint;
mod system_stats_endpoint;

use actix_files::Files;
use actix_files::NamedFile;
use actix_web::dev::WebService;
use actix_web::error::ErrorInternalServerError;
use actix_web::{error, middleware, web, App, HttpResponse, HttpServer, Responder};
use log::{error, info};
use serde_json::json;
use std::sync::Mutex;
use sysinfo::System;

// Function to serve the index.html file
async fn index() -> Result<impl Responder, actix_web::Error> {
    match NamedFile::open_async("wwwroot/index.html").await {
        Ok(file) => Ok(file),
        Err(_) => Err(ErrorInternalServerError("Error serving index.html")),
    }
}
#[actix_web::main]
async fn main() -> std::io::Result<()> {
    std::env::set_var("RUST_LOG", "trace");
    env_logger::init();
    let port = 1420; // Port to listen on

    match authentication::init_auth() {
        Ok(_) => info!("Authentication initialized"),
        Err(e) => error!("Failed to initialize authentication: {}", e),
    }

    servers::server_db::initialize();

    let sys = web::Data::new(Mutex::new(System::new_all()));

    let server = HttpServer::new(move || {
        App::new()
            .wrap(middleware::Logger::default())
            .wrap(auth_middleware::AuthMiddleware)
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
                            .service(minecraft_endpoint::get_version_by_id),
                    )
                    .service(web::scope("java").service(java_endpoint::get_java_versions))
                    .service(
                        web::scope("system")
                            .service(system_stats_endpoint::get_system_info)
                            .service(system_stats_endpoint::get_system_usage)
                            .service(system_stats_endpoint::get_storage_info)
                            .app_data(sys.clone()),
                    )
                    .service(
                        web::scope("server")
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
                                            .service(file_system_endpoint::get_server_files),
                                    )
                                    .service(server_endpoint::get_server_by_id),
                            )
                            .service(server_endpoint::get_servers)
                            .service(server_endpoint::create_server),
                    ),
            )
            // Serve static files from the wwwroot directory
            .service(Files::new("/", "wwwroot").index_file("index.html"))
            // Handle all other routes by serving the index.html file
            .default_service(web::route().to(index))
    })
    .workers(4)
    .bind(format!("0.0.0.0:{port}", port = port))?
    .run();
    info!("Starting server at http://127.0.0.1:{port}...");
    server.await
}
