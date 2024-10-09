mod auth_middleware;
mod authentication_endpoint;
mod backups_endpoint;
mod file_system_endpoint;
mod java_endpoint;
mod minecraft_endpoint;
mod server_endpoint;
mod server_properties_endpoint;
mod server_settings_endpoint;
mod system_stats_endpoint;
use actix_files::file_extension_to_mime;
use actix_web::error::ErrorInternalServerError;
use actix_web::{error, get, middleware, web, App, HttpResponse, HttpServer, Responder};
use easy_upnp::PortMappingProtocol::TCP;
use easy_upnp::{add_ports, UpnpConfig};
use include_dir::{include_dir, Dir};
use log::{debug, error, info, trace};
use scheduler::{
    add_schedule, duration, remove_schedule, start_ticking_schedules, stop_ticking_schedules,
};
use serde_json::json;
use std::sync::Mutex;
use std::time::SystemTime;
use sysinfo::System;

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    std::env::set_var("RUST_LOG", "trace");
    env_logger::init();

    add_schedule!(duration::Duration::from_days(1), true, true, |schedule| {
        let end_time: SystemTime = schedule.get_end_time();
let datetime: chrono::DateTime<chrono::Local> = end_time.into();
info!(
    "Ticking schedule: {:?} - {}",
    schedule.id,
    datetime.format("%m-%d-%Y %H:%M:%S")
);
    });

    start_ticking_schedules!();
    loop {
        std::thread::sleep(std::time::Duration::from_secs(1));
    }

    let port = 1420; // Port to listen on

    match authentication::initialize() {
        Ok(_) => info!("Authentication initialized"),
        Err(e) => error!("Failed to initialize authentication: {}", e),
    }

    servers::server_db::initialize();
    backups::initialize();

    // This will open the webui port on the router using upnp
    // Spawn a thread to refresh the upnp port every 5 minutes
    std::thread::spawn(move || {
        let duration = 60 * 5; // 5 minutes
        loop {
            for item in add_ports([UpnpConfig {
                address: None,
                port,
                comment: "Obsidian Minecraft Server Manager".to_string(),
                protocol: TCP,
                duration,
            }]) {
                match item {
                    Ok(_) => trace!("Webui port upnp refreshed!"),
                    Err(e) => error!("Failed to forward port: {}", e),
                }
            }
            std::thread::sleep(std::time::Duration::from_secs(duration as u64));
        }
    });

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
                            .service(minecraft_endpoint::get_version_by_id)
                            .service(minecraft_endpoint::get_releases)
                            .service(minecraft_endpoint::get_snapshots)
                            .service(minecraft_endpoint::get_java_version_by_minecraft_version),
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
                                    .service(server_endpoint::get_server_by_id)
                                    .service(
                                        web::scope("backups")
                                            .service(backups_endpoint::get_backups)
                                            .service(backups_endpoint::create_manual_backup),
                                    ),
                            )
                            .service(server_endpoint::get_servers)
                            .service(server_endpoint::create_server),
                    ),
            )
            .service(web::scope("assets/{file}").service(assets))
            // Handle all other routes by serving the index.html file
            .default_service(web::route().to(index))
    })
    .workers(4)
    .bind(format!("0.0.0.0:{port}", port = port))?
    .run();
    info!("Starting server at http://127.0.0.1:{port}...");
    let stop_result = server.await;
    debug!("Server stopped");

    // Stop all schedules
    // This is necessary to prevent the server from hanging on shutdown
    // because the schedules are still running
    // This is probably not necessary in a production environment
    stop_ticking_schedules!();

    stop_result
}

// Include the wwwroot directory from the OUT_DIR
static WWWROOT: Dir = include_dir!("target/wwwroot");

// Function to serve the index.html file
async fn index() -> Result<impl Responder, actix_web::Error> {
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
