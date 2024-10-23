use actix_web::{get, web, HttpResponse, Responder};
use log::error;
use serde_json::json;
use std::str::FromStr;

/// Retrieves all supported server loaders
#[get("/supported_loaders")]
pub async fn get_supported_loaders() -> impl Responder {
    // Return the list of all supported loaders as JSON response
    HttpResponse::Ok().json(servers::supported_loaders::SupportedLoaders::all())
}
#[get("/loader/{loader_id}")]
pub async fn get_loader_by_id(loader_id: web::Path<String>) -> impl Responder {
    // Fetch the loader by the given ID
    let loader = match servers::supported_loaders::SupportedLoaders::from_str(loader_id.as_str()) {
        Some(loader) => loader,
        None => {
            let msg = format!("Loader with id: {} not found", loader_id);
            error!("{}", msg);
            return HttpResponse::BadRequest().json(json!({"error":msg}));
        }
    };
    // Return the loader details as JSON response
    HttpResponse::Ok().json(loader)
}
