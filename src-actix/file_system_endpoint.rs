use actix_web::{post, web, HttpMessage, HttpRequest, HttpResponse, Responder};
use authentication::data::User;
use crypto::hashids::decode;
use log::error;
use serde_json::json;
use servers::physical_server;

#[post("/")]
pub async fn get_server_files(
	id: web::Path<String>,
	body: Option<String>,
	req: HttpRequest,
) -> impl Responder {
	if let Some(user) = req.extensions().get::<User>() {
		let id_number = match decode(&id) {
			Ok(id) => id,
			Err(e) => {
				error!("Invalid id: {}", id);
				return HttpResponse::BadRequest()
					.json(json!({"error": format!("Invalid id: {}", id)}));
			}
		};
		if id_number.len() <= 0 {
			return HttpResponse::BadRequest()
				.json(json!({"error": format!("Invalid id: {}", id)}));
		}
		return HttpResponse::Ok().json(physical_server::get_server_filesystem_entries(
			id_number[0] as u32,
			user.id,
			body,
		));
	}

	HttpResponse::Unauthorized().json(json!({"error": "Unauthorized"}))
}
