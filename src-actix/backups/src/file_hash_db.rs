use crate::hashed_file::HashedFile;
use crate::{system_time_from_string, system_time_to_string};
use database::create_appdb_connection;
use log::{debug, error, info};
use sqlite::State;
use std::path::{Path, PathBuf};

pub(crate) fn initialize() {
    debug!("Initializing file hash table");
    let conn = create_appdb_connection().expect("Failed to connect to database");
    if let Err(e) = conn.execute(
        "
				CREATE TABLE IF NOT EXISTS file_hash_table
				(
				    path      TEXT NOT NULL UNIQUE PRIMARY KEY,
				    hash      TEXT NOT NULL,
				    timestamp DATETIME NOT NULL
				);
	",
    ) {
        error!("Failed to create backups table: {}", e);
    } else {
        info!("Successfully created or verified the backups table.");
    }
}
pub(crate) fn insert(
    path: impl AsRef<Path>,
    hash: impl AsRef<str>,
) -> Result<(), Box<dyn std::error::Error>> {
    info!(
        "Inserting a new hashed file: {:?} ({})",
        path.as_ref(),
        hash.as_ref()
    );

    // Get the last modified time of the file
    let metadata = std::fs::metadata(path.as_ref())?;
    let timestamp = metadata.modified()?;
    let timestamp = system_time_to_string(timestamp);

    let conn = create_appdb_connection()?;
    let mut stmt =
        conn.prepare("INSERT INTO file_hash_table (path, hash, timestamp) VALUES (?, ?, ?)")?;
    stmt.bind((1, path.as_ref().to_str()))?;
    stmt.bind((2, hash.as_ref()))?;
    stmt.bind((3, timestamp.as_str()))?;
    stmt.next()?;

    Ok(())
}

pub(crate) fn update(
    path: impl AsRef<Path>,
    hash: impl AsRef<str>,
) -> Result<(), Box<dyn std::error::Error>> {
    info!(
        "Updating the hash for file: {:?} ({})",
        path.as_ref(),
        hash.as_ref()
    );

    // Get the last modified time of the file
    let metadata = std::fs::metadata(path.as_ref())?;
    let timestamp = metadata.modified()?;
    let timestamp = system_time_to_string(timestamp);

    let conn = create_appdb_connection()?;
    let mut stmt =
        conn.prepare("UPDATE file_hash_table SET hash = ?, timestamp = ? WHERE path = ?")?;
    stmt.bind((1, hash.as_ref()))?;
    stmt.bind((2, timestamp.as_str()))?;
    stmt.bind((3, path.as_ref().to_str()))?;
    stmt.next()?;
    Ok(())
}
#[allow(dead_code)]
pub(crate) fn delete(path: impl AsRef<Path>) -> Result<(), sqlite::Error> {
    let conn = create_appdb_connection()?;
    let mut stmt = conn.prepare("DELETE FROM file_hash_table WHERE path = ?")?;
    stmt.bind((1, path.as_ref().to_str()))?;
    stmt.next()?;
    Ok(())
}
pub(crate) fn exists(path: impl AsRef<Path>) -> Result<bool, sqlite::Error> {
    let conn = create_appdb_connection()?;
    let mut stmt = conn.prepare("SELECT * FROM file_hash_table WHERE path = ? LIMIT 1")?;
    stmt.bind((1, path.as_ref().to_str()))?;
    Ok(State::Row == stmt.next()?) // If there is a row, the file exists
}
pub(crate) fn get(path: impl AsRef<Path>) -> Option<HashedFile> {
    let conn = create_appdb_connection().ok()?;
    let mut stmt = conn
        .prepare("SELECT * FROM file_hash_table WHERE path = ? LIMIT 1")
        .ok()?;
    stmt.bind((1, path.as_ref().to_str())).ok()?;
    if let State::Row = stmt.next().ok()? {
        Some(HashedFile {
            path: PathBuf::from(&stmt.read::<String, _>("path").ok()?),
            hash: stmt.read::<String, _>("hash").ok()?,
            timestamp: system_time_from_string(stmt.read::<String, _>("timestamp").ok()?)?,
        })
    } else {
        None
    }
}
