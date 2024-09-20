use serde::{Deserialize, Serialize};

#[derive(Debug, Serialize, Deserialize)]
pub struct User {
    pub id: u32,
    pub username: String,
    pub password: String,
    pub admin: bool,
    pub created_at: String,
    pub updated_at: String,
    pub last_login: String,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct UserLogin {
    pub username: String,
    pub password: String,
}
#[derive(Debug, Serialize, Deserialize)]
pub struct UserRegistration {
    pub username: String,
    pub password: String,
    pub access_token: String,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct UserResponse {
    pub id: u32,
    pub username: String,
    pub admin: bool,
    pub created_at: String,
    pub updated_at: String,
    pub last_login: String,
    pub token: String,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct Token {
    pub username: String,
    pub hash: String,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct PublicUser {
    pub id: u32,
    pub username: String,
    pub admin: bool,
    pub created_at: String,
    pub updated_at: String,
    pub last_login: String,
}
#[derive(Debug, Serialize, Deserialize)]
pub struct PublicUsersList {
    pub users: Vec<PublicUser>,
    pub count: u32,
}
