use actix_web::web::Query;
use actix_web::{get, web, HttpResponse, Responder};
use loader_manager::supported_loaders::Loader;
use log::error;
use serde_json::json;
use std::collections::HashMap;
use std::str::FromStr;

/// Retrieves all supported server loaders
#[get("/supported_loaders/{minecraft_version}")]
pub async fn get_supported_loaders(
    minecraft_version: web::Path<String>,
    query: Query<HashMap<String, String>>,
) -> impl Responder {
    let minecraft_version = minecraft_version.into_inner();
    let is_snapshot = query
        .get("snapshot")
        .map(|v| v.eq_ignore_ascii_case("true"))
        .unwrap_or(false);
    // Return the list of all supported loaders as JSON response
    if minecraft_version.eq_ignore_ascii_case("all") {
        HttpResponse::Ok().json(Loader::all())
    } else {
        HttpResponse::Ok().json(
            Loader::all()
                .iter()
                .filter(|loader| loader.supported_by_minecraft_version(minecraft_version.as_str(), is_snapshot))
                .collect::<Vec<&Loader>>(),
        )
    }
}
#[get("/{loader_id}/{minecraft_version}")]
pub async fn get_loader_by_id(params: web::Path<(String, String)>) -> impl Responder {
    let (loader_id, minecraft_version) = params.into_inner();
    // Fetch the loader by the given ID
    let loader = match Loader::from_str(loader_id.as_str()) {
        Ok(loader) => loader,
        Err(e) => {
            let msg = format!("Loader with id: {} not found", loader_id);
            error!("{}", msg);
            return HttpResponse::BadRequest().json(json!({"message":msg, "error":e}));
        }
    };
    match loader {
        Loader::Fabric => match loader_manager::fabric::versions().await {
            Ok(versions) => HttpResponse::Ok().json(json!(versions)),
            Err(_) => HttpResponse::InternalServerError().json(json!({"message":"Failed to fetch versions"})),
        },
        Loader::Forge => match loader_manager::forge::versions(minecraft_version).await {
            Ok(versions) => HttpResponse::Ok().json(json!(versions)),
            Err(_) => HttpResponse::InternalServerError().json(json!({"message":"Failed to fetch versions"})),
        },

        _ => HttpResponse::Ok().json(json!([])),
    }
}
