use crate::server_db::{get_server_by_id, Server};
use log::{debug, error, info};
use std::fmt::format;
use std::fs::create_dir;
use std::path::{Path, PathBuf};

pub fn clean_file_path_string(name: &str) -> String {
    let clean = name
        .trim()
        .replace(" ", "_")
        .replace(|c: char| !c.is_alphanumeric(), "")
        .to_lowercase();
    debug!("'{}' -> '{}'", name, clean);
    clean
}

pub fn create_server_directory(id: i32) -> Result<PathBuf, String> {
    let server: Server = match get_server_by_id(id) {
        Some(s) => s,
        None => {
            let msg = format!("Server with id: {} not found", id);
            error!("{}", msg);
            return Err(msg);
        }
    };
    let name = clean_file_path_string(server.name.as_str());
    let path = Path::join(Path::new(&get_servers_directory()), Path::new(&name));
    let path = find_unique_directory_name(&path);

    match create_dir(&path) {
        Ok(_) => {
            info!("Created server directory: {:?}", path);
        }
        Err(e) => {
            error!("Failed to create server directory: {}", e);
            return Err(format!("Failed to create server directory: {}", e));
        }
    }
    Ok(path)
}

fn find_unique_directory_name(path: &PathBuf) -> PathBuf {
    if !path.exists() {
        return path.clone();
    } // path does not exist, return it as is
    let mut i = 1;
    let mut new_path = path.clone();
    while new_path.exists() {
        new_path = path.with_file_name(format!(
            "{}-{}",
            path.file_name().unwrap().to_str().unwrap(),
            i
        ));
        i += 1;
    }
    new_path
}

pub fn get_servers_directory() -> PathBuf {
    let path = "./servers";
    let path = match Path::new(path).canonicalize() {
        Ok(p) => p,
        Err(e) => {
            let msg = format!("Could not canonicalize servers directory: {} {}", path, e);
            error!("{}", msg);
            panic!("{}", msg);
        }
    };
    if !path.exists() {
        match create_dir(&path) {
            Ok(_) => {
                info!("Created servers directory: {:?}", path);
            }
            Err(e) => {
                error!("Failed to create servers directory: {}", e);
                panic!("Failed to create servers directory: {}", e);
            }
        };
    }
    info!("Using servers directory: {:?}", path);
    path
}
