use actix_web::{get, HttpResponse, Responder};
use minecraft::{minecraft_java, minecraft_version};
use serde_json::json;

#[get("/versions")]
pub async fn get_minecraft_versions() -> impl Responder {
    match minecraft_version::get_versions().await {
        Ok(versions) => HttpResponse::Ok().json(versions),
        Err(e) => HttpResponse::BadRequest().json(json!({"error": e})),
    }
}

#[get("/latest-release")]
pub async fn get_latest_release() -> impl Responder {
    match minecraft_version::get_latest_release().await {
        Ok(version) => HttpResponse::Ok().json(version),
        Err(e) => HttpResponse::BadRequest().json(json!({"error": e})),
    }
}

#[get("/latest-snapshot")]
pub async fn get_latest_snapshot() -> impl Responder {
    match minecraft_version::get_latest_snapshot().await {
        Ok(version) => HttpResponse::Ok().json(version),
        Err(e) => HttpResponse::BadRequest().json(json!({"error": e})),
    }
}


#[get("/version/{version}")]
pub async fn get_version_by_id(version: actix_web::web::Path<String>) -> impl Responder {
    match minecraft_version::get_version_by_id(&version, None).await {
        Ok(version) => HttpResponse::Ok().json(version),
        Err(e) => HttpResponse::BadRequest().json(json!({"error": e})),
    }
}