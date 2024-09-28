use crate::hashed_file::HashedFile;
use crate::{create_connection, system_time_from_string};
use log::{debug, error, info};
use sqlite::State;
use std::path::{Path, PathBuf};

pub(crate) fn initialize() {
    debug!("Initializing file hash table");
    let conn = create_connection().expect("Failed to connect to database");
    if let Err(e) = conn.execute(
        "
				CREATE TABLE IF NOT EXISTS file_hash_table
				(
				    path      TEXT NOT NULL UNIQUE PRIMARY KEY,
				    hash      TEXT NOT NULL,
				    timestamp DATETIME DEFAULT CURRENT_TIMESTAMP
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
) -> Result<(), sqlite::Error> {
    info!(
        "Inserting a new hashed file: {:?} ({})",
        path.as_ref(),
        hash.as_ref()
    );
    let conn = create_connection()?;
    let mut stmt = conn.prepare("INSERT INTO file_hash_table (path, hash) VALUES (?, ?)")?;
    stmt.bind((1, path.as_ref().to_str()))?;
    stmt.bind((2, hash.as_ref()))?;
    stmt.next()?;

    Ok(())
}

pub(crate) fn update(path: impl AsRef<Path>, hash: impl AsRef<str>) -> Result<(), sqlite::Error> {
    info!(
        "Updating the hash for file: {:?} ({})",
        path.as_ref(),
        hash.as_ref()
    );
    let conn = create_connection()?;
    let mut stmt = conn.prepare("UPDATE file_hash_table SET hash = ? WHERE path = ?")?;
    stmt.bind((1, hash.as_ref()))?;
    stmt.bind((2, path.as_ref().to_str()))?;
    stmt.next()?;
    Ok(())
}

pub(crate) fn delete(path: impl AsRef<Path>) -> Result<(), sqlite::Error> {
    let conn = create_connection()?;
    let mut stmt = conn.prepare("DELETE FROM file_hash_table WHERE path = ?")?;
    stmt.bind((1, path.as_ref().to_str()))?;
    stmt.next()?;
    Ok(())
}
pub(crate) fn exists(path: impl AsRef<Path>) -> Result<bool, sqlite::Error> {
    let conn = create_connection()?;
    let mut stmt = conn.prepare("SELECT COUNT(*) FROM file_hash_table WHERE path = ? LIMIT 1")?;
    stmt.bind((1, path.as_ref().to_str()))?;
    Ok(State::Row == stmt.next()?) // If there is a row, the file exists
}
pub(crate) fn get(path: impl AsRef<Path>) -> Option<HashedFile> {
    let conn = create_connection().ok()?;
    let mut stmt = conn
        .prepare("SELECT * FROM file_hash_table WHERE path = ? LIMIT 1")
        .ok()?;
    stmt.bind((1, path.as_ref().to_str())).ok()?;
    if let State::Row = stmt.next().ok()? {
        Some(HashedFile {
            path: PathBuf::from(&stmt.read::<String, _>("path").ok()?),
            hash: stmt.read::<String, _>("hash").ok()?.into_bytes(),
            timestamp: system_time_from_string(stmt.read::<String, _>("timestamp").ok()?)?,
        })
    } else {
        None
    }
}
