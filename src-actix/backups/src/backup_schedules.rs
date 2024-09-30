use crate::{create_connection, get_backups_directory};
use log::{debug, error, info};

pub fn initialize() {
	debug!("Initializing backup schedule table");
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
		info!("Successfully created or verified the backup schedule table.");
	}
}