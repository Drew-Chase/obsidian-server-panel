pub mod backup_item;
pub mod hashed_file;
mod backup_db;
mod hashed_backup_item;

use log::{error, info};
use std::path::{Path, PathBuf};

pub fn initialize() {
	info!("Initializing backups database");
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
	let backups_dir = get_backups_directory();
	if !backups_dir.exists() {
		std::fs::create_dir_all(backups_dir).expect("Unable to create backup directory.");
	}
}

fn create_connection() -> Result<sqlite::Connection, sqlite::Error> {
	sqlite::Connection::open("servers.db").map_err(|e| {
		error!(
            "Failed to open servers database connection for backups: {}",
            e
        );
		e
	})
}

pub fn get_backups_directory() -> PathBuf {
	Path::new("backups").to_path_buf()
}
