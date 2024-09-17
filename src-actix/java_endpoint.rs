use actix_web::{get, HttpResponse, Responder};
use serde_json::json;
use minecraft::minecraft_java;

#[get("/versions")]
pub async fn get_java_versions() -> impl Responder {
	match minecraft_java::get_java_versions().await {
		Ok(versions) => HttpResponse::Ok().json(versions),
		Err(e) => HttpResponse::BadRequest().json(json!({"error": e})),
	}
}
