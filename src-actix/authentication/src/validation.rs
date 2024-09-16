use crate::data::{Token, User};
use crate::get_user_by_username;
use base64::prelude::BASE64_STANDARD;
use base64::Engine;
use serde_json::json;

pub fn generate_token(user: &User, client_secret: &str) -> String {
    let json = json!(
        {
            "user": user,
            "client_secret": client_secret,
        }
    )
    .to_string();
    let hash = bcrypt::hash(json.as_str(), 4).unwrap();
    let json = json!({
        "username": user.username,
        "hash": hash,
    })
    .to_string();

    BASE64_STANDARD.encode(json.as_bytes())
}

pub fn validate_token(token: &str, client_secret: &str) -> Result<User, String> {
    let decoded = match BASE64_STANDARD.decode(token.as_bytes()) {
        Ok(decoded) => decoded,
        Err(e) => return Err(format!("Failed to decode token: {}", e)),
    };

    let tok: Token = match String::from_utf8(decoded) {
        Ok(json) => match serde_json::from_str(json.as_str()) {
            Ok(tok) => tok,
            Err(e) => return Err(format!("Failed to parse token JSON: {}", e)),
        },
        Err(e) => return Err(format!("Failed to convert token to string: {}", e)),
    };

    let user = match get_user_by_username(tok.username.as_str(), false) {
        Ok(user) => user,
        Err(e) => return Err(e),
    };

    let json = json!(
        {
            "user": user,
            "client_secret": client_secret,
        }
    )
    .to_string();

    match bcrypt::verify(json.as_str(), tok.hash.as_str()) {
        Ok(true) => Ok(user),
        Ok(false) => Err("Invalid token".to_string()),
        Err(e) => Err(format!("Failed to verify token: {}", e)),
    }
}
