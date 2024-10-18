use actix_web::{get, post, web, Error, HttpRequest, HttpResponse, Responder};
use java::versions::JavaVersion;
use serde_json::json;
use std::sync::{Arc, Mutex};

#[get("/versions")]
pub async fn get_java_versions() -> impl Responder {
    HttpResponse::Ok().json(json!(JavaVersion::list().await.unwrap()))
}
#[get("/install/{version}/ws")]
pub async fn install_java_version(
    version: web::Path<String>,
    req: HttpRequest,
    stream: web::Payload,
) -> Result<impl Responder, Error> {
    let (res, session, _stream) = actix_ws::handle(&req, stream)?;
    let session = Arc::new(Mutex::new(session));
    let version = version.clone();
    let session_clone = Arc::clone(&session);

    actix_web::rt::spawn(async move {
        match JavaVersion::from_version(&version).await {
            Ok(v) => {
                let install_fut = v.install({
                    let session_clone = Arc::clone(&session_clone);
                    move |progress| {
                        let json = serde_json::to_string(&progress).unwrap();
                        let session_clone = Arc::clone(&session_clone);
                        async move {
                            let mut session = session_clone.lock().unwrap();
                            session.text(json).await.unwrap();
                        };
                    }
                });

                if let Err(e) = install_fut.await {
                    let error_msg = json!({ "error": e.to_string() }).to_string();
                    let session_clone = Arc::clone(&session_clone);
                    actix_web::rt::spawn(async move {
                        let mut session = session_clone.lock().unwrap();
                        session.text(error_msg).await.unwrap();
                    });
                }

            }
            Err(e) => {
                let error_msg = json!({ "error": e.to_string() }).to_string();
                let session_clone = Arc::clone(&session_clone);
                actix_web::rt::spawn(async move {
                    let mut session = session_clone.lock().unwrap();
                    session.text(error_msg).await.unwrap();
                });
            }
        };
    });

    Ok(res)
}