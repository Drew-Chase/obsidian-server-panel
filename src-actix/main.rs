mod authentication_endpoint;
mod minecraft_endpoint;

use actix_files::Files;
use actix_files::NamedFile;
use actix_web::error::ErrorInternalServerError;
use actix_web::{error, middleware, web, App, HttpResponse, HttpServer, Responder};
use serde_json::json;

// Function to serve the index.html file
async fn index() -> Result<impl Responder, actix_web::Error> {
	match NamedFile::open_async("wwwroot/index.html").await {
		Ok(file) => Ok(file),
		Err(_) => Err(ErrorInternalServerError("Error serving index.html")),
	}
}
#[actix_web::main]
async fn main() -> std::io::Result<()> {
	std::env::set_var("RUST_LOG", "actix_web=trace");
	env_logger::init();

	let port = 1420; // Port to listen on


	println!(
		"Starting server at http://127.0.0.1:{port}",
		port = port,
	);


	HttpServer::new(move || {
		App::new()
			.wrap(middleware::Logger::default())
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
							.service(authentication_endpoint::get_users)
					)
					.service(
						web::scope("minecraft")
							.service(minecraft_endpoint::get_minecraft_versions)
							.service(minecraft_endpoint::get_java_versions)
					)
			)
			// Serve static files from the wwwroot directory
			.service(
				Files::new("/", "wwwroot")
					.index_file("index.html"),
			)
			// Handle all other routes by serving the index.html file
			.default_service(web::route().to(index))
	})
		.workers(4)
		.bind(format!("0.0.0.0:{port}", port = port))?
		.run()
		.await
}