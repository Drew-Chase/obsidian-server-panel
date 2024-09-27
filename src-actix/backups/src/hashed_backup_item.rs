use std::path::PathBuf;
use std::time::SystemTime;
use crypto::hashids::encode;
use serde_derive::Serialize;
use crate::backup_item::{BackupCreationMethod, BackupItem, BackupType};

#[derive(Debug, Serialize)]
pub struct HashedBackupItem {
	pub id: String,
	pub path: PathBuf,
	pub r#type: BackupType,
	pub method: BackupCreationMethod,
	pub timestamp: SystemTime,
	pub size: u64,
	pub server: u32,
}

impl HashedBackupItem {
	pub fn from_backup_item(item: BackupItem) -> Self {
		HashedBackupItem {
			id: encode(&[item.id as u64]),
			path: item.path,
			method: item.method,
			r#type: item.r#type,
			timestamp: item.timestamp,
			size: item.size,
			server: item.server,
		}
	}
}
