use actix_web::{get, HttpResponse, Responder};
use serde_json::json;

#[get("/java/versions")]
pub async fn get_java_versions() -> impl Responder {
    
}