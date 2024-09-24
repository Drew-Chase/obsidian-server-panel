pub mod backup_item;
pub mod hashed_file;

use log::{error, info};
use serde::{Deserialize, Serialize};
use sha2::{Digest, Sha256};
use sqlite::{State, Statement};
use std::env;
use std::error::Error;
use std::fs::File;
use std::io::{BufReader, Read};
use std::path::{Path, PathBuf};
use std::time::SystemTime;

pub fn initialize() {
	info!("Initializing backups database");
	let conn = create_connection().expect("Failed to connect to database");
	if let Err(e) = conn.execute(
		"
		CREATE TABLE IF NOT EXISTS backups
		(
		    path      TEXT             NOT NULL PRIMARY KEY UNIQUE,
		    type      TINYINT          NOT NULL,
		    method    TINYINT          NOT NULL,
		    timestamp UNSIGNED BIG INT NOT NULL,
		    size      UNSIGNED BIG INT NOT NULL,
		    server    INTEGER          NOT NULL
		);

		CREATE TABLE IF NOT EXISTS file_hash_table
		(
		    path      TEXT    NOT NULL UNIQUE,
		    hash      TEXT    NOT NULL,
		    timestamp TEXT    NOT NULL,
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
	Path::join(
		env::current_exe()
			.expect("Failed to get the current executable path")
			.parent()
			.expect("Failed to get the parent directory of the executable"),
		Path::new("backups"),
	)
}
