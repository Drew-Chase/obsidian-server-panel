use actix_web::{get, web, HttpResponse, Responder};
use minecraft_modding_platforms::modpacks::BrowseOptions;

#[get("discover")]
pub async fn discover_modpacks(query: web::Query<BrowseOptions>) -> impl Responder {
    let options = query.into_inner();
    let results = minecraft_modding_platforms::modpacks::search_modpacks(options).await;
    match results {
        Ok(data) => HttpResponse::Ok().json(data),
        Err(_) => HttpResponse::BadRequest().json("Failed to search modpacks")
    }
}
