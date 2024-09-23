use log::error;
use sha2::{Digest, Sha256};
use sqlite::Statement;
use std::error::Error;
use std::fs::File;
use std::io::{BufReader, Read};
use std::path::{Path, PathBuf};
use std::time::SystemTime;

pub struct HashedFile {
	pub name: String,
	pub path: PathBuf,
	pub hash: Vec<u8>,
	pub timestamp: SystemTime,
	pub size: u64,
}

impl HashedFile {
	pub fn get(path: &Path) -> Option<Self> {
		let conn = match create_connection() {
			Ok(c) => c,
			Err(_) => {
				return None;
			}
		};
		let mut stmt = match conn.prepare("select * from file_hash_table where path = ? LIMIT 1") {
			Ok(s) => s,
			Err(e) => {
				error!("Failed to prepare statement: {}", e);
				return None;
			}
		};

		match stmt.bind((1, path.to_str()?)) {
			Ok(_) => {}
			Err(e) => {
				error!("Unable to bind '{:?}'-> path: {}", path, e);
				return None;
			}
		}

		match stmt.next() {
			Ok(_) => {}
			Err(e) => {
				error!("Failed to execute select on file_hash_table: {}", e);
				return None;
			}
		}

		Self::from_statement(&stmt)
	}

	pub fn has_file_been_changed(self) -> bool {
		self.hash
			!= match Self::hash_file(self.path.as_path()) {
			Ok(h) => h,
			Err(e) => {
				error!("Unable to hash file {:?}: {}", self.path, e);
				return false;
			}
		}
	}

	pub fn cache_file_hash(path: &Path) -> Option<Vec<u8>> {
		let hash = match Self::hash_file(&path) {
			Ok(hash) => hash,
			Err(e) => {
				error!("Failed to hash file '{:?}': {}", path, e);
				return None;
			}
		};

		let conn = match create_connection() {
			Ok(c) => c,
			Err(_) => {
				return None;
			}
		};

		let mut stmt = match conn.prepare("INSERT INTO file_hash_table (name, path, hash, timestamp, size) VALUES (?, ?, ?, ?, ?)") {
			Ok(s) => s,
			Err(e) => {
				error!("Failed to prepare statement: {}", e);
				return None;
			}
		};

		match stmt.bind((1, path.file_name()?.to_str()?)) {
			Ok(_) => {}
			Err(e) => {
				error!("Failed to bind name variable: {}", e);
				return None;
			}
		}

		match stmt.bind((2, path.to_str())) {
			Ok(_) => {}
			Err(e) => {
				error!("Failed to bind path variable: {}", e);
				return None;
			}
		}

		match stmt.bind((3, Self::hash_to_string(&hash).as_str())) {
			Ok(_) => {}
			Err(e) => {
				error!("Failed to bind hash variable: {}", e);
				return None;
			}
		}

		match stmt.bind((
			4,
			SystemTime::now()
				.duration_since(SystemTime::UNIX_EPOCH)
				.unwrap()
				.as_secs() as i64,
		)) {
			Ok(_) => {}
			Err(e) => {
				error!("Failed to bind timestamp variable: {}", e);
				return None;
			}
		}

		match stmt.bind((
			5,
			std::fs::metadata(path).map(|m| m.len()).unwrap_or(0) as i64,
		)) {
			Ok(_) => {}
			Err(e) => {
				error!("Failed to bind size variable: {}", e);
				return None;
			}
		}

		None
	}

	fn from_statement(stmt: &Statement) -> Option<Self> {
		Some(
			HashedFile {
				name: stmt.read::<String, _>("name").map_err(|e| {
					error!("Unable to parse the column `name` from the file_hash_table in the `from_id` function: {}", e);
					return None::<Self>;
				}).ok()?,
				path: Path::new(&(stmt.read::<String, _>("path").map_err(|e| {
					error!("Unable to parse the column `path` from the file_hash_table in the `from_id` function: {}", e);
					return None::<Self>;
				}).ok()?)).to_path_buf(),
				hash: stmt.read::<String, _>("hash").map_err(|e| {
					error!("Unable to parse the column `hash` from the file_hash_table in the `from_id` function: {}", e);
					return None::<Self>;
				}).ok()?.into_bytes(),
				timestamp: SystemTime::UNIX_EPOCH + std::time::Duration::from_secs(
					stmt.read::<String, _>("timestamp").map_err(|e| {
						error!("Unable to parse the column `timestamp` from the file_hash_table in the `from_id` function: {}", e);
						return None::<Self>;
					}).ok()?.parse::<u64>().map_err(|e| {
						error!("Unable to convert timestamp string to u64 in the `from_id` function: {}", e);
						return None::<Self>;
					}).ok()?
				),
				size: stmt.read::<i64, _>("size").map_err(|e| {
					error!("Unable to parse the column `size` from the file_hash_table in the `from_id` function: {}", e);
					return None::<Self>;
				}).ok()? as u64,
			}
		)
	}

	fn hash_file(path: &Path) -> Result<Vec<u8>, Box<dyn Error>> {
		match File::open(path) {
			Ok(file) => {
				let mut reader = BufReader::new(file);
				let mut hasher = Sha256::new();
				let mut buffer = [0; 4096];

				loop {
					match reader.read(&mut buffer) {
						Ok(n) => {
							if n == 0 {
								break;
							}
							hasher.update(&buffer[..n]);
						}
						Err(e) => {
							error!("Error reading file '{:?}': {}", path, e);
							return Err(Box::new(e));
						}
					}
				}
				Ok(hasher.finalize().to_vec())
			}
			Err(e) => {
				error!("Failed to open file '{:?}': {}", path, e);
				Err(Box::new(e))
			}
		}
	}
	fn hash_to_string(hash: &Vec<u8>) -> String {
		hash.iter().map(|byte| format!("{:02x}", byte)).collect()
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
