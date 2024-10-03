use actix_web::{get, HttpResponse, Responder};
use serde_json::json;

#[get("/versions")]
pub async fn get_java_versions() -> impl Responder {
    match java::versions::list().await {
        Ok(versions) => HttpResponse::Ok().json(versions),
        Err(e) => HttpResponse::BadRequest().json(json!({"error": e})),
    }
}

#[get("/versions/{os}")]
pub async fn get_os_version(os: actix_web::web::Path<java::data::OS>) -> impl Responder {
    let all = match java::versions::list().await {
        Ok(versions) => versions,
        Err(e) => return HttpResponse::BadRequest().json(json!({"error": e})),
    };
    match java::versions::get_os_version(os.into_inner(), &all) {
        Some(versions) => HttpResponse::Ok().json(versions),
        None => HttpResponse::BadRequest().json(json!({"error": "OS not found"})),
    }
}
#[get("/versions/{os}/{runtime}")]
pub async fn get_runtime_version(
    os: actix_web::web::Path<java::data::OS>,
    runtime: actix_web::web::Path<java::data::RuntimeVersion>,
) -> impl Responder {
    let all = match java::versions::list().await {
        Ok(versions) => versions,
        Err(e) => return HttpResponse::BadRequest().json(json!({"error": e})),
    };
    let os = match java::versions::get_os_version(os.into_inner(), &all) {
        Some(versions) => versions,
        None => return HttpResponse::BadRequest().json(json!({"error": "OS not found"})),
    };
    match java::versions::get_runtime_version(&os, &runtime.into_inner()) {
        Some(version) => HttpResponse::Ok().json(version),
        None => HttpResponse::BadRequest().json(json!({"error": "Runtime not found"})),
    }
}
