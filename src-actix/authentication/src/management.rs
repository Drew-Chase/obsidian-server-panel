use crate::access_tokens::use_access_token;
use crate::data::{PublicUser, PublicUsersList, UserLogin, UserRegistration, UserResponse};
use crate::validation::{generate_token, validate_token};
use crate::{create_appdb_connection, get_user_by_username};

pub fn create_user(user: UserRegistration, is_admin: bool) -> Result<(), String> {
    if !is_admin {
        match use_access_token(user.access_token.as_str()) {
            Ok(_) => {}
            Err(e) => return Err(e),
        }
    }

    let conn = match create_appdb_connection() {
        Ok(conn) => conn,
        Err(e) => return Err(format!("Failed to create DB connection: {}", e)),
    };

    let mut stmt = match conn.prepare(
        r"
            INSERT INTO users
                (username, password, admin, created_at, updated_at, last_login)
            VALUES
                (?, ?, ?, CURRENT_TIMESTAMP, CURRENT_TIMESTAMP, CURRENT_TIMESTAMP);
        ",
    ) {
        Ok(stmt) => stmt,
        Err(e) => return Err(format!("Failed to prepare statement: {}", e)),
    };

    // encrypt password c
    let password = match bcrypt::hash(user.password.as_str(), 4) {
        Ok(password) => password,
        Err(e) => return Err(format!("Failed to hash password: {}", e)),
    };

    match stmt.bind((1, user.username.as_str())) {
        Ok(_) => {}
        Err(e) => return Err(format!("Failed to bind username: {}", e)),
    };

    match stmt.bind((2, password.as_str())) {
        Ok(_) => {}
        Err(e) => return Err(format!("Failed to bind password: {}", e)),
    };
    match stmt.bind((3, if is_admin { 1 } else { 0 })) {
        Ok(_) => {}
        Err(e) => return Err(format!("Failed to bind admin boolean: {}", e)),
    };

    if let Err(e) = stmt.next() {
        return Err(format!("Failed to create user: {}", e));
    }

    Ok(())
}

pub fn login(user: UserLogin, client_secret: &str) -> Result<UserResponse, String> {
    let found_user = match get_user_by_username(user.username.as_str(), false) {
        Ok(user) => user,
        Err(e) => return Err(e),
    };

    match bcrypt::verify(user.password.as_str(), found_user.password.as_str()) {
        Ok(true) => {}
        Ok(false) => return Err("Invalid password".to_string()),
        Err(e) => return Err(format!("Failed to verify password: {}", e)),
    }

    let token = generate_token(&found_user, client_secret);

    let user = UserResponse {
        id: found_user.id,
        username: found_user.username,
        admin: found_user.admin,
        created_at: found_user.created_at,
        updated_at: found_user.updated_at,
        last_login: found_user.last_login,
        token,
    };
    Ok(user)
}

pub fn login_with_token(token: &str, client_secret: &str) -> Result<UserResponse, String> {
    let user = match validate_token(token, client_secret) {
        Ok(user) => user,
        Err(e) => return Err(e),
    };
    Ok(UserResponse {
        id: user.id,
        username: user.username,
        admin: user.admin,
        created_at: user.created_at,
        updated_at: user.updated_at,
        last_login: user.last_login,
        token: token.to_string(),
    })
}
pub fn get_users_list() -> Result<PublicUsersList, String> {
    let conn = match create_appdb_connection() {
        Ok(conn) => conn,
        Err(e) => return Err(format!("Failed to create DB connection: {}", e)),
    };

    let mut stmt = match conn.prepare("SELECT *, (SELECT COUNT(*) FROM users) AS 'count' FROM users") {
        Ok(stmt) => stmt,
        Err(e) => return Err(format!("Failed to prepare statement: {}", e)),
    };

    let mut users = Vec::new();
    let mut count = -1;
    while let Ok(state) = stmt.next() {
        if state == sqlite::State::Done {
            break;
        }

        let user = user_from_statement(&mut stmt)?;
        if count == -1 {
            count = match stmt.read(7) {
                Ok(count) => count,
                Err(e) => return Err(format!("Failed to read count: {}", e)),
            };
        }
        users.push(user);
    }
    Ok(PublicUsersList {
        users,
        count: count as u32,
    })
}

pub fn get_user_by_id(id: u32) -> Result<Option<PublicUser>, String> {
    let connection = match create_appdb_connection() {
        Ok(connection) => connection,
        Err(e) => {
            return Err(format!("Failed to create DB connection: {}", e));
        }
    };

    let mut stmt = match connection.prepare("SELECT * FROM users WHERE id = ? LIMIT 1") {
        Ok(stmt) => stmt,
        Err(e) => {
            return Err(format!("Failed to prepare statement: {}", e));
        }
    };

    match stmt.bind((1, id as i64)) {
        Ok(_) => {}
        Err(e) => {
            return Err(format!("Failed to bind id: {}", e));
        }
    }
    let result = match stmt.next() {
        Ok(result) => result,
        Err(e) => {
            return Err(format!("Failed to get user by id: {}", e));
        }
    };

    if result == sqlite::State::Done {
        return Ok(None); // No user found
    }

    let user = user_from_statement(&mut stmt)?;
    Ok(Some(user))
}

fn user_from_statement(stmt: &mut sqlite::Statement) -> Result<PublicUser, String> {
    Ok(PublicUser {
        id: match stmt.read::<i64, _>(0) {
            Ok(id) => id as u32,
            Err(e) => return Err(format!("Failed to read user id: {}", e)),
        },
        username: match stmt.read(1) {
            Ok(username) => username,
            Err(e) => return Err(format!("Failed to read username: {}", e)),
        },
        admin: match stmt.read::<i64, usize>(3) {
            Ok(admin) => admin == 1,
            Err(e) => return Err(format!("Failed to read admin status: {}", e)),
        },
        created_at: match stmt.read(4) {
            Ok(created_at) => created_at,
            Err(e) => return Err(format!("Failed to read creation date: {}", e)),
        },
        updated_at: match stmt.read(5) {
            Ok(updated_at) => updated_at,
            Err(e) => return Err(format!("Failed to read updated date: {}", e)),
        },
        last_login: match stmt.read(6) {
            Ok(last_login) => last_login,
            Err(e) => return Err(format!("Failed to read last login date: {}", e)),
        },
    })
}
