use crate::create_appdb_connection;
use rand::Rng;
use serde::Serialize;

#[derive(Debug, Serialize)]
pub struct AccessToken {
    pub id: i64,
    pub token: String,
    pub created_at: String,
}

pub fn generate_unique_registration_access_token(message: &str) -> Result<String, String> {
    if message.len() > 10 {
        return Err("Message is too long, the message cannot be longer than 10 characters".to_string());
    }

    let mut rng = rand::thread_rng();
    let sec = std::iter::repeat(())
        .map(|()| rng.sample(rand::distr::Alphanumeric))
        .take(32)
        .collect::<Vec<u8>>();

    let sec = match String::from_utf8(sec) {
        Ok(sec) => sec,
        Err(e) => {
            return Err(format!(
                "Failed to convert generated random bytes to UTF-8 string: {}",
                e
            ))
        }
    };

    let sec = format!("{}@{}", message, sec);

    let conn = match create_appdb_connection() {
        Ok(conn) => conn,
        Err(e) => return Err(format!("Failed to create connection to the database: {}", e)),
    };

    let mut stmt = match conn.prepare("INSERT INTO `access-tokens` (token, created_at) VALUES (?, CURRENT_TIMESTAMP);")
    {
        Ok(stmt) => stmt,
        Err(e) => return Err(format!("Failed to prepare statement: {}", e)),
    };
    match stmt.bind((1, sec.as_str())) {
        Ok(_) => {}
        Err(e) => return Err(format!("Failed to bind token: {}", e)),
    };

    match stmt.next() {
        Ok(_) => Ok(sec),
        Err(e) => Err(format!("Failed to insert token into the database: {}", e)),
    }
}

pub fn does_token_exist(token: &str) -> Result<bool, String> {
    let conn = match create_appdb_connection() {
        Ok(conn) => conn,
        Err(e) => return Err(format!("Failed to create connection to the database: {}", e)),
    };
    let mut stmt = match conn.prepare("SELECT 1 FROM `access-tokens` WHERE token = ?;") {
        Ok(exists) => exists,
        Err(e) => return Err(format!("Failed to check if token exists: {}", e)),
    };
    match stmt.bind((1, token)) {
        Ok(_) => {}
        Err(e) => return Err(format!("Failed to bind token: {}", e)),
    };

    match stmt.next() {
        Ok(_) => {}
        Err(e) => return Err(format!("Failed to check if token exists: {}", e)),
    };

    let exists: bool = match stmt.read::<i64, usize>(0) {
        Ok(exists) => exists == 1,
        Err(e) => return Err(format!("Failed to read token existence: {}", e)),
    };

    Ok(exists)
}

pub fn get_all_access_tokens() -> Result<Vec<AccessToken>, String> {
    let conn = match create_appdb_connection() {
        Ok(conn) => conn,
        Err(e) => return Err(format!("Failed to create connection to the database: {}", e)),
    };
    let mut stmt = match conn.prepare("SELECT * FROM `access-tokens`;") {
        Ok(stmt) => stmt,
        Err(e) => return Err(format!("Failed to prepare statement: {}", e)),
    };

    let mut tokens: Vec<AccessToken> = vec![];

    while let Ok(sqlite::State::Row) = stmt.next() {
        let id: i64 = match stmt.read(0) {
            Ok(id) => id,
            Err(e) => return Err(format!("Failed to read token id: {}", e)),
        };
        let token: String = match stmt.read(1) {
            Ok(token) => token,
            Err(e) => return Err(format!("Failed to read token: {}", e)),
        };
        let created_at: String = match stmt.read(2) {
            Ok(date) => date,
            Err(e) => return Err(format!("Failed to read token creation date: {}", e)),
        };
        tokens.push(AccessToken { id, token, created_at });
    }
    Ok(tokens)
}

pub fn use_access_token(token: &str) -> Result<(), String> {
    let conn = match create_appdb_connection() {
        Ok(conn) => conn,
        Err(e) => return Err(format!("Failed to create connection to the database: {}", e)),
    };
    let mut stmt = match conn.prepare("SELECT 1 FROM `access-tokens` WHERE token = ?;") {
        Ok(exists) => exists,
        Err(e) => return Err(format!("Failed to check if token exists: {}", e)),
    };
    match stmt.bind((1, token)) {
        Ok(_) => {}
        Err(e) => return Err(format!("Failed to bind token: {}", e)),
    };

    match stmt.next() {
        Ok(_) => {}
        Err(e) => return Err(format!("Failed to check if token exists: {}", e)),
    };

    let exists: bool = match stmt.read::<i64, usize>(0) {
        Ok(exists) => exists == 1,
        Err(e) => return Err(format!("Failed to read token existence: {}", e)),
    };

    if !exists {
        return Err("Token does not exist".to_string());
    }

    let mut stmt = match conn.prepare("DELETE FROM `access-tokens` WHERE token = ?;") {
        Ok(stmt) => stmt,
        Err(e) => return Err(format!("Failed to prepare statement: {}", e)),
    };

    match stmt.bind((1, token)) {
        Ok(_) => {}
        Err(e) => return Err(format!("Failed to bind token: {}", e)),
    };

    match stmt.next() {
        Ok(_) => {}
        Err(e) => return Err(format!("Failed to delete token: {}", e)),
    };

    Ok(())
}
