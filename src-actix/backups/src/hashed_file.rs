use crate::{create_connection, file_hash_db};
use chrono::{DateTime, NaiveDateTime, Utc};
use log::error;
use serde_derive::{Deserialize, Serialize};
use sha2::{Digest, Sha256};
use sqlite::Statement;
use std::error::Error;
use std::fmt::Write;
use std::fs::File;
use std::io::{BufReader, Read};
use std::path::{Path, PathBuf};
use std::time::SystemTime;

#[derive(Serialize, Deserialize, Clone, Debug)]
pub struct HashedFile {
    pub path: PathBuf,
    pub hash: Vec<u8>,
    pub timestamp: SystemTime,
}
impl HashedFile {
    pub fn from_path(path: impl AsRef<Path>) -> Result<Self, Box<dyn Error>> {
        let path = path.as_ref().to_path_buf();
        if !path.exists() {
            return Err("File does not exist".into());
        }
        if !path.is_file() {
            return Err("Path is not a file".into());
        }
        if let Some(file) = file_hash_db::get(&path) {
            return Ok(file);
        }

        let hash = hash_file(&path)?;
        let timestamp = SystemTime::now();
        Ok(Self {
            path,
            hash,
            timestamp,
        })
    }
    pub fn changed(&self) -> Result<bool, Box<dyn Error>> {
        if let Some(file) = file_hash_db::get(&self.path) {
            let updated_hash = hash_file(&self.path)?;
            Ok(file.hash != updated_hash)
        } else {
            Ok(true)
        }
    }
    
    pub fn cache(&self) -> Result<(), Box<dyn Error>> {
        if file_hash_db::exists(&self.path)? {
            file_hash_db::update(&self.path, &hash_to_string(&self.hash))?;
        } else {
            file_hash_db::insert(&self.path, &hash_to_string(&self.hash))?;
        }
        Ok(())
    }
    
}

fn hash_file(path: impl AsRef<Path>) -> Result<Vec<u8>, Box<dyn Error>> {
    let file = File::open(path.as_ref())?;
    let mut reader = BufReader::new(file);
    let mut hasher = Sha256::new();
    let mut buffer = [0; 1024];
    loop {
        let bytes_read = reader.read(&mut buffer)?;
        if bytes_read == 0 {
            break;
        }
        hasher.update(&buffer[..bytes_read]);
    }
    Ok(hasher.finalize().to_vec())
}
fn hash_to_string(hash: &[u8]) -> String {
    hash.iter().fold(String::new(), |mut acc, byte| {
        write!(acc, "{:02x}", byte).unwrap();
        acc
    })
}
