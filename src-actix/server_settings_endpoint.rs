use actix_web::{get, web, HttpRequest, HttpResponse, Responder};
use serde_json::json;

#[get("/")]
pub async fn get_server_settings(id: web::Path<(String,)>, req: HttpRequest) -> impl Responder {
    HttpResponse::BadRequest().json(json!({"error":"Not implemented"}))
}
