use crate::server_db::Server;
use log::{error, info};
use notify::Watcher;
use std::error::Error;
use std::path::Path;

pub async fn read_console<F>(server: Server, callback: F) -> Result<(), Box<dyn Error>>
where
    F: Fn(&str) + Send + 'static,
{
    let directory = match server.directory {
        Some(directory) => directory,
        None => Err("Server directory not found!")?,
    };

    // Read the current contents of the latest.log file
    let log_file = Path::new(&directory).join("logs/latest.log");
    let contents = std::fs::read_to_string(&log_file)?;
    callback(&contents);
    let mut last_read = contents.len();

    // Watch the latest.log file for changes
    let (tx, rx) = std::sync::mpsc::channel();
    let mut watcher = notify::recommended_watcher(tx)?;
    watcher.watch(&log_file, notify::RecursiveMode::NonRecursive)?;

    // Update the callback with the new contents of the latest.log file
    loop {
        match rx.recv() {
            Ok(event) => match event {
                Ok(event) => {
                    info!("Event: {:?}", event);
                }
                Err(e) => {
                    error!("Error: {:?}", e);
                    break;
                }
            },
            Err(e) => {
                error!("Error: {:?}", e);
                break;
            }
        }
    }

    Ok(())
}
