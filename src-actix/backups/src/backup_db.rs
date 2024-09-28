use crate::backup_item::{BackupCreationMethod, BackupItem, BackupType};
use crate::{create_connection, get_backups_directory, system_time_from_string};
use chrono::{DateTime, NaiveDateTime, Utc};
use log::{debug, error, info};
use sqlite::{State, Statement};
use std::path::Path;
use std::time::SystemTime;

/// Initializes the backups table in the database and ensures the backup directory exists.
///
/// This function performs the following tasks:
/// 1. Connects to the database and creates the backups table if it does not already exist.
/// 2. Ensures the backup directory exists by creating it if necessary.
///
/// # Panics
///
/// This function will panic if it fails to:
/// - Connect to the database.
/// - Create the backups table.
/// - Create the backup directory.
pub(crate) fn initialize() {
    debug!("Initializing backups table");
    let conn = create_connection().expect("Failed to connect to database");
    if let Err(e) = conn.execute(
        "
				CREATE TABLE IF NOT EXISTS backups
				(
				    id        INTEGER          NOT NULL PRIMARY KEY AUTOINCREMENT,
				    path      TEXT             NOT NULL UNIQUE,
				    type      TINYINT          NOT NULL,
				    method    TINYINT          NOT NULL,
				    timestamp DATETIME DEFAULT CURRENT_TIMESTAMP,
				    size      UNSIGNED BIG INT NOT NULL,
				    server    INTEGER          NOT NULL
				);
	",
    ) {
        error!("Failed to create backups table: {}", e);
    } else {
        info!("Successfully created or verified the backups table.");
    }
    let backups_dir = get_backups_directory();
    if !backups_dir.exists() {
        std::fs::create_dir_all(backups_dir).expect("Unable to create backup directory.");
    }
}

/// Inserts a new backup item into the database.
///
/// # Arguments
///
/// * `item` - A `BackupItem` struct containing the details of the backup to be inserted.
///
/// # Returns
///
/// An `Option<BackupItem>` containing the inserted `BackupItem` with the new ID if successful, otherwise `None`.
pub fn insert(item: BackupItem) -> Option<BackupItem> {
    info!("Inserting a new backup item: {:?}", item);
    let conn = create_connection().ok()?;

    let mut stmt = conn
        .prepare("INSERT INTO backups (path, method, type, size, server) VALUES (?, ?, ?, ?, ?)")
        .ok()?;

    let binds = [
        (1, item.path.to_str()?),
        (2, &(item.clone().method as i64).to_string()),
        (3, &(item.clone().r#type as i64).to_string()),
        (4, &(item.size as i64).to_string()),
        (5, &(item.server as i64).to_string()),
    ];

    for (pos, val) in binds.iter() {
        if stmt.bind(((*pos, *val))).is_err() {
            error!("Unable to bind value to the statement at position {}", pos);
            return None;
        }
    }

    stmt.next().ok()?;
    let id = get_last_inserted_id(&conn)?;
    info!("Backup item inserted successfully with ID: {}", id);
    Some(BackupItem { id, ..item })
}

/// Deletes a backup item from the database by ID.
///
/// # Arguments
///
/// * `id` - The ID of the backup item to be deleted.
pub fn delete(id: u32) {
    info!("Deleting backup item with ID: {}", id);
    let conn = match create_connection() {
        Ok(conn) => conn,
        Err(e) => {
            error!("Failed to create a connection to the database: {}", e);
            return;
        }
    };

    let mut stmt = match conn.prepare("DELETE FROM backups WHERE id = ?") {
        Ok(stmt) => stmt,
        Err(e) => {
            error!("Failed to prepare the statement: {}", e);
            return;
        }
    };

    if let Err(e) = stmt.bind((1, id as i64)) {
        error!("Unable to bind the id to the statement: {}", e);
        return;
    }

    match stmt.next() {
        Ok(State::Done) => info!("Backup item with ID {} deleted successfully.", id),
        Ok(_) => error!(
            "Unexpected state while deleting backup item with ID {}.",
            id
        ),
        Err(e) => error!("Failed to delete backup item with ID {}: {}", id, e),
    }
}

/// Retrieves a backup item from the database by ID.
///
/// # Arguments
///
/// * `id` - The ID of the backup item to be retrieved.
///
/// # Returns
///
/// An `Option<BackupItem>` containing the `BackupItem` if found, otherwise `None`.
pub fn get(id: u32) -> Option<BackupItem> {
    info!("Retrieving backup item with ID: {}", id);
    let conn = create_connection().ok()?;

    let mut stmt = conn
        .prepare("SELECT * FROM backups WHERE id = ? LIMIT 1")
        .ok()?;

    if stmt.bind((1, id as i64)).is_err() {
        error!("Unable to bind the id to the statement.");
        return None;
    }

    stmt.next().ok()?;
    from_statement(&stmt)
}

/// Retrieves a list of all backup items from the database.
///
/// # Returns
///
/// A `Vec<BackupItem>` containing all the `BackupItem`s.
pub fn list() -> Vec<BackupItem> {
    info!("Retrieving list of all backup items.");
    let conn = match create_connection() {
        Ok(conn) => conn,
        Err(_) => {
            error!("Failed to create a connection to the database.");
            return Vec::new();
        }
    };

    let mut stmt = match conn.prepare("SELECT * FROM backups") {
        Ok(stmt) => stmt,
        Err(e) => {
            error!(
                "Unable to prepare the statement to get the list of backups: {}",
                e
            );
            return Vec::new();
        }
    };

    let mut items = Vec::new();

    while State::Row == stmt.next().unwrap() {
        if let Some(item) = from_statement(&stmt) {
            items.push(item);
        } else {
            error!("Failed to convert row to BackupItem");
        }
    }

    info!("Retrieved list of all backup items successfully.");
    items
}

/// Retrieves a list of backup items from the database by server ID.
///
/// # Arguments
///
/// * `server_id` - The ID of the server to retrieve the backup items for.
///
/// # Returns
///
/// A `Vec<BackupItem>` containing all the `BackupItem`s for the specified server.
/// If no backup items are found or an error occurs, an empty `Vec` is returned.
pub fn list_by_server(server_id: u32) -> Vec<BackupItem> {
    info!(
        "Retrieving list of backup items for server with ID: {}",
        server_id
    );
    let conn = match create_connection() {
        Ok(conn) => conn,
        Err(e) => {
            error!("Failed to create a connection: {}", e);
            return Vec::new();
        }
    };

    let mut stmt = match conn.prepare("SELECT * FROM backups WHERE server = ?") {
        Ok(stmt) => stmt,
        Err(e) => {
            error!("Failed to prepare statement: {}", e);
            return Vec::new();
        }
    };

    if let Err(e) = stmt.bind((1, server_id as i64)) {
        error!("Unable to bind the server id: {}", e);
        return Vec::new();
    }

    let mut items = Vec::new();

    while let Ok(State::Row) = stmt.next() {
        match from_statement(&stmt) {
            Some(item) => items.push(item),
            None => error!("Failed to convert row to BackupItem"),
        }
    }

    info!(
        "Retrieved list of backup items for server with ID {} successfully.",
        server_id
    );
    items
}

/// Retrieves the last inserted ID from the `sqlite_sequence` table.
///
/// # Arguments
///
/// * `conn` - A reference to the SQLite connection.
///
/// # Returns
///
/// An `Option<u32>` containing the last inserted ID if successful, otherwise `None`.
fn get_last_inserted_id(conn: &sqlite::Connection) -> Option<u32> {
    let mut stmt = conn
        .prepare("SELECT seq FROM sqlite_sequence WHERE name = 'backups' LIMIT 1")
        .ok()?;

    stmt.next().ok()?;
    stmt.read::<i64, _>("seq").ok().map(|seq| seq as u32)
}

/// Converts a SQLite statement row into a `BackupItem`.
///
/// # Arguments
///
/// * `stmt` - A reference to a `Statement` representing a row in the SQLite result set.
///
/// # Returns
///
/// An `Option<BackupItem>` containing the `BackupItem` if the conversion is successful, otherwise `None`.
fn from_statement(stmt: &Statement) -> Option<BackupItem> {
    Some(BackupItem {
        id: stmt.read::<i64, _>("id").ok()? as u32,
        path: Path::new(&(stmt.read::<String, _>("path").ok()?)).to_path_buf(),
        method: match stmt.read::<i64, _>("method").ok()? {
            0 => BackupCreationMethod::AUTO,
            1 => BackupCreationMethod::MANUAL,
            _ => {
                error!("Unknown method value in the `from_statement` function");
                return None;
            }
        },
        r#type: match stmt.read::<i64, _>("type").ok()? {
            0 => BackupType::Full,
            1 => BackupType::Incremental,
            _ => {
                error!("Unknown type value in the `from_statement` function");
                return None;
            }
        },
        timestamp: system_time_from_string(&stmt.read::<String, _>("timestamp").ok()?)?,
        size: stmt.read::<i64, _>("size").ok()? as u64,
        server: stmt.read::<i64, _>("server").ok()? as u32,
    })
}
