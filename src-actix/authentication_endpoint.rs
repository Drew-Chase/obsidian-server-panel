use actix_web::{get, post, web, HttpRequest, HttpResponse, Responder};
use authentication::access_tokens::{
    generate_unique_registration_access_token, get_all_access_tokens,
};
use authentication::data::{UserLogin, UserRegistration};
use serde_json::json;
use std::collections::HashMap;

#[post("/login")]
pub async fn login(body: web::Json<UserLogin>, req: HttpRequest) -> impl Responder {
    match authentication::management::login(
        body.into_inner(),
        req.connection_info().realip_remote_addr().unwrap(),
    ) {
        Ok(user) => HttpResponse::Ok().json(user),
        Err(e) => HttpResponse::BadRequest().json(json!({"error": e})),
    }
}
#[derive(serde::Deserialize)]
struct Token {
    token: String,
}

#[post("/login/token")]
pub async fn login_with_token(token: web::Json<Token>, req: HttpRequest) -> impl Responder {
    match authentication::management::login_with_token(
        token.token.as_str(),
        req.connection_info().realip_remote_addr().unwrap(),
    ) {
        Ok(user) => HttpResponse::Ok().json(user),
        Err(e) => HttpResponse::BadRequest()
            .json(json!({ "error": format!("Failed to login with token: {}", e).as_str() })),
    }
}

#[post("/register")]
pub async fn create_user(body: web::Json<UserRegistration>) -> impl Responder {
    match authentication::management::create_user(body.into_inner(), false) {
        Ok(_) => HttpResponse::Ok().json(json!({"message":"User created"})),
        Err(e) => HttpResponse::BadRequest().json(json!({"error": e})),
    }
}

#[post("/register/generate-access-token")]
pub async fn generate_access_token(req: HttpRequest) -> impl Responder {
    let query = req.query_string();
    let parts: HashMap<&str, &str> = query
        .split('&')
        .filter_map(|part| {
            let mut split = part.split('=');
            let key = split.next()?;
            let val = split.next()?;
            Some((key, val))
        })
        .collect();

    let message = parts.get("message").unwrap_or(&"");

    // decode the message
    let message: String = match urlencoding::decode(message) {
        Ok(message) => message.to_string(),
        Err(e) => return HttpResponse::BadRequest().json(json!({"error": e.to_string()})),
    };

    match generate_unique_registration_access_token(message.as_str()) {
        Ok(token) => {
            HttpResponse::Ok().json(json!({"message": "Access token generated", "token": token}))
        }
        Err(e) => HttpResponse::BadRequest().json(json!({"error": e})),
    }
}

#[get("/register/access-tokens")]
pub async fn get_access_tokens() -> impl Responder {
    match get_all_access_tokens() {
        Ok(tokens) => HttpResponse::Ok().json(tokens),
        Err(e) => HttpResponse::BadRequest().json(json!({"error": e})),
    }
}

#[post("/register/validate-access-token")]
pub async fn validate_access_token(body: web::Json<Token>) -> impl Responder {
    match authentication::access_tokens::does_token_exist(body.token.as_str()) {
        Ok(exists) => {
            if exists {
                HttpResponse::Ok().json(json!({"message": "Token exists"}))
            } else {
                HttpResponse::BadRequest().json(json!({"error": "Token does not exist"}))
            }
        }
        Err(e) => HttpResponse::BadRequest().json(json!({"error": e})),
    }
}

#[get("/users")]
pub async fn get_users() -> impl Responder {
    match authentication::management::get_users_list() {
        Ok(users) => HttpResponse::Ok().json(users),
        Err(e) => HttpResponse::BadRequest().json(json!({"error": e})),
    }
}

#[get("/exists")]
pub async fn check_if_access_token_exists(req: HttpRequest) -> impl Responder {
    let query = req.query_string();
    let parts: HashMap<&str, &str> = query
        .split('&')
        .filter_map(|part| {
            let mut split = part.split('=');
            let key = split.next()?;
            let val = split.next()?;
            Some((key, val))
        })
        .collect();

    let token = parts.get("token").unwrap_or(&"");

    let token: String = match urlencoding::decode(token) {
        Ok(t) => t.to_string(),
        Err(e) => return HttpResponse::BadRequest().json(json!({"error": e.to_string()})),
    };

    let token: &str = token.as_str();

    match authentication::access_tokens::does_token_exist(token) {
        Ok(exists) => HttpResponse::Ok().json(json!({"exists": exists})),
        Err(e) => HttpResponse::BadRequest().json(json!({"error": e})),
    }
}
