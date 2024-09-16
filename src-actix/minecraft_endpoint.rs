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

#[get("/java-versions")]
pub async fn get_java_versions() -> impl Responder {
	match minecraft_java::get_java_versions().await {
		Ok(versions) => HttpResponse::Ok().json(versions),
		Err(e) => HttpResponse::BadRequest().json(json!({"error": e})),
	}
}