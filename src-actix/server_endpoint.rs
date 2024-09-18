use actix_web::{get, web, HttpResponse, Responder};

#[get("/")]
pub async fn get_servers() -> impl Responder {
	HttpResponse::Ok()
}

#[get("/{id}")]
pub async fn get_server_by_id(id: web::Path<String>) -> impl Responder {
	HttpResponse::Ok()
}