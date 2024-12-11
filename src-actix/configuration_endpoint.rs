use actix_web::{get, post, web, HttpResponse, Responder};

#[get("")]
pub async fn get_configuration() -> impl Responder {
    HttpResponse::Ok().json("Configuration")
}
#[post("")]
pub async fn update_configuration(body: web::Json<configuration::config::ObsidianConfig>) -> impl Responder {
    HttpResponse::Ok().json("Configuration")
}
