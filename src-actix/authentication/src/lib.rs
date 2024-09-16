pub mod access_tokens;
pub mod data;
pub mod management;
pub mod validation;

use crate::data::{User, UserRegistration};
use crate::management::create_user;
use sqlite::Connection;

pub fn init_auth() -> Result<(), String> {
    println!("Initializing authentication database!");
    match create_db_connection() {
        Ok(conn) => {
            match conn.execute(
                r"
								CREATE TABLE IF NOT EXISTS users (
									id INTEGER PRIMARY KEY,
									username TEXT NOT NULL UNIQUE,
									password TEXT NOT NULL,
									admin BOOLEAN NOT NULL,
									created_at DATE CURRENT_TIMESTAMP NOT NULL,
									updated_at DATE CURRENT_TIMESTAMP NOT NULL,
									last_login DATE CURRENT_TIMESTAMP NOT NULL
								);
							",
            ) {
                Ok(_) => {}
                Err(_) => return Err("Failed to create users table".to_string()),
            }
            match conn.execute(
                r"
								CREATE TABLE IF NOT EXISTS `access-tokens` (
									id INTEGER PRIMARY KEY,
									token TEXT NOT NULL UNIQUE,
									created_at DATE CURRENT_TIMESTAMP NOT NULL
								);
							",
            ) {
                Ok(_) => {}
                Err(e) => return Err(format!("Failed to create `access-tokens` table: {}", e)),
            }
            match create_user(UserRegistration {
				username: "warehouse".to_string(),
				password: "warehouse".to_string(),
				access_token: "".to_string(),
			}, true)
			{
				Ok(_) => { println!("Default user created, username: admin, password: stacked") },
				Err(e) => println!("Failed to create default user, this is probably because it already exists, but if it does not you may have to delete the user.db file and restart (note: this will delete all existing users). \nError: {}", e),
			}
            Ok(())
        }
        Err(e) => Err(e),
    }
}

pub fn create_db_connection() -> Result<Connection, String> {
    let conn = match sqlite::open("user.db").map_err(|e| e.to_string()) {
        Ok(conn) => conn,
        Err(e) => return Err(e),
    };

    Ok(conn)
}

pub fn get_user_by_username(username: &str, sanitized: bool) -> Result<User, String> {
    let conn = match create_db_connection() {
        Ok(connection) => connection,
        Err(e) => return Err(format!("Failed to create DB connection: {}", e)),
    };

    let mut stmt = match conn.prepare("SELECT * FROM users WHERE username = ? LIMIT 1") {
        Ok(statement) => statement,
        Err(e) => return Err(format!("Failed to prepare statement: {}", e)),
    };

    if let Err(e) = stmt.bind((1, username)) {
        return Err(format!("Failed to bind username: {}", e));
    };

    match stmt.next() {
        Ok(state) => {
            if state == sqlite::State::Done {
                return Err("User not found".to_string());
            }
        }
        Err(e) => return Err(format!("Failed to execute query: {}", e)),
    };

    let mut password = "".to_string();
    if !sanitized {
        password = match stmt.read(2) {
            Ok(password) => password,
            Err(_) => return Err("Failed to read password".to_string()),
        };
    }

    Ok(User {
        id: match stmt.read(0) {
            Ok(id) => id,
            Err(_) => return Err("Failed to read user id".to_string()),
        },
        username: match stmt.read(1) {
            Ok(username) => username,
            Err(_) => return Err("Failed to read username".to_string()),
        },
        password,
        admin: match stmt.read::<i64, usize>(3) {
            Ok(admin) => admin == 1,
            Err(_) => return Err("Failed to read admin status".to_string()),
        },
        created_at: match stmt.read(4) {
            Ok(created_at) => created_at,
            Err(_) => return Err("Failed to read creation date".to_string()),
        },
        updated_at: match stmt.read(5) {
            Ok(updated_at) => updated_at,
            Err(_) => return Err("Failed to read updated date".to_string()),
        },
        last_login: match stmt.read(6) {
            Ok(last_login) => last_login,
            Err(_) => return Err("Failed to read last login date".to_string()),
        },
    })
}
