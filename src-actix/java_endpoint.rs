use actix_web::{get, post, HttpResponse, Responder};
use java::versions::JavaVersion;
use serde_json::json;

#[get("/versions")]
pub async fn get_java_versions() -> impl Responder {
    HttpResponse::Ok().json(json!(JavaVersion::list().await.unwrap()))
}

#[post("/install/{version}")]
pub async fn install_java_version() -> impl Responder {
    HttpResponse::Ok().json(json!({ "message": "Java version installed" }))
}
