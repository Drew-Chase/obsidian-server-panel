use actix_web::web::Path;
use actix_web::{get, HttpResponse, Responder};
use minecraft::minecraft_version;
use serde_json::json;

#[get("/versions")]
pub async fn get_minecraft_versions() -> impl Responder {
    match minecraft_version::get_versions().await {
        Ok(versions) => HttpResponse::Ok().json(versions),
        Err(e) => HttpResponse::BadRequest().json(json!({"error": e})),
    }
}

#[get("/version/releases/latest")]
pub async fn get_latest_release() -> impl Responder {
    match minecraft_version::get_latest_release().await {
        Ok(version) => HttpResponse::Ok().json(version),
        Err(e) => HttpResponse::BadRequest().json(json!({"error": e})),
    }
}

#[get("/version/snapshots/latest")]
pub async fn get_latest_snapshot() -> impl Responder {
    match minecraft_version::get_latest_snapshot().await {
        Ok(version) => HttpResponse::Ok().json(version),
        Err(e) => HttpResponse::BadRequest().json(json!({"error": e})),
    }
}
#[get("/versions/snapshots")]
pub async fn get_snapshots() -> impl Responder {
    match minecraft_version::get_versions().await {
        Ok(versions) => {
            let snapshots: Vec<minecraft::minecraft_version::MinecraftVersionResponse> = versions
                .into_iter()
                .filter(|version| version.version_type == minecraft::minecraft_version::VersionType::Snapshot)
                .collect();
            HttpResponse::Ok().json(snapshots)
        }
        Err(e) => HttpResponse::BadRequest().json(json!({"error": e})),
    }
}

#[get("/versions/releases")]
pub async fn get_releases() -> impl Responder {
    match minecraft_version::get_versions().await {
        Ok(versions) => {
            let releases: Vec<minecraft::minecraft_version::MinecraftVersionResponse> = versions
                .into_iter()
                .filter(|version| version.version_type == minecraft::minecraft_version::VersionType::Release)
                .collect();
            HttpResponse::Ok().json(releases)
        }
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
#[get("/{version}/java")]
pub async fn get_java_version_by_minecraft_version(version: Path<String>) -> impl Responder {
    match minecraft_version::get_version_runtime(&version.as_str()).await {
        Ok(version) => HttpResponse::Ok().json(version),
        Err(e) => HttpResponse::BadRequest().json(json!({"error": format!("{}", e)})),
    }
}
