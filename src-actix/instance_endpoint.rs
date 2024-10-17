use actix_web::{get, web, HttpResponse, Responder};
use instance_manager::modpacks::BrowseOptions;
use serde_json::json;

#[get("discover")]
pub async fn discover_modpacks(query: web::Query<BrowseOptions>) -> impl Responder {
    let options = query.into_inner();
    let results = instance_manager::modpacks::search_modpacks(options).await;
    match results {
        Ok(data) => HttpResponse::Ok().json(data),
        Err(e) => HttpResponse::BadRequest()
            .json(json!({"message": "Failed to fetch modpacks", "error":format!("{}", e)})),
    }
}
