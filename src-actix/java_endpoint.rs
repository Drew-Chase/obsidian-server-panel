use actix_web::{delete, get, web, Error, HttpRequest, HttpResponse, Responder};
use actix_web_lab::sse;
use java::versions::JavaVersion;
use log::error;
use serde_json::json;
use std::time::Duration;
use tokio::sync::mpsc;

#[get("/versions")]
pub async fn get_java_versions() -> impl Responder {
    HttpResponse::Ok().json(json!(JavaVersion::list().await.unwrap()))
}

#[get("/versions/{runtime}/files")]
pub async fn get_installation_files(runtime: web::Path<String>) -> impl Responder {
    let runtime = runtime.into_inner();
    match JavaVersion::from_runtime(&runtime).await {
        Ok(v) => match v.get_installation_files().await {
            Ok(files) => HttpResponse::Ok().json(json!(files)),
            Err(e) => HttpResponse::BadRequest().json(json!({ "error": e.to_string() })),
        },
        Err(e) => HttpResponse::BadRequest().json(json!({ "error": e.to_string() })),
    }
}

#[delete("/versions/{runtime}")]
pub async fn uninstall_java_version(runtime: web::Path<String>) -> impl Responder {
    let runtime = runtime.into_inner();
    match JavaVersion::from_runtime(&runtime).await {
        Ok(v) => match v.uninstall() {
            Ok(_) => HttpResponse::Ok().json(json!({ "message": "Uninstalled" })),
            Err(e) => HttpResponse::BadRequest().json(json!({ "error": e.to_string() })),
        },
        Err(e) => HttpResponse::BadRequest().json(json!({ "error": e.to_string() })),
    }
}

#[get("/install/{runtime}/sse")]
pub async fn install_java_version(
    runtime: web::Path<String>,
    _req: HttpRequest,
) -> Result<impl Responder, Error> {
    let (sender, receiver) = mpsc::channel(2);
    let runtime = runtime.into_inner();
    actix_web::rt::spawn(async move {
        match JavaVersion::from_runtime(&runtime).await {
            Ok(v) => {
                let install_fut = v.install({
                    let sender = sender.clone();
                    move |progress| {
                        let json = serde_json::to_string(&progress).unwrap();
                        let sender = sender.clone();
                        // Spawn a new task for each progress update
                        tokio::spawn(async move {
                            if sender
                                .send(sse::Data::new(json).event("progress").into())
                                .await
                                .is_err()
                            {
                                // Handle error or log it
                                error!("Failed to send progress update");
                            }
                        });
                    }
                });
                if let Err(e) = install_fut.await {
                    let error_msg = json!({ "error": e.to_string() }).to_string();
                    if sender
                        .send(sse::Data::new(error_msg).event("error").into())
                        .await
                        .is_err()
                    {
                        // Handle error or log it
                        error!("Failed to send error message");
                        return;
                    }
                }
            }
            Err(e) => {
                let error_msg = json!({ "error": e.to_string() }).to_string();
                if sender
                    .send(sse::Data::new(error_msg).event("error").into())
                    .await
                    .is_err()
                {
                    // Handle error or log it
                    error!("Failed to send error message");
                    return;
                }
            }
        };
        // Close the channel after the task is done
        let _ = sender
            .send(sse::Data::new("done").event("done").into())
            .await;
    });
    Ok(sse::Sse::from_infallible_receiver(receiver).with_keep_alive(Duration::from_secs(3)))
}
