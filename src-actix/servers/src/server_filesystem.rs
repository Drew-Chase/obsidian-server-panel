use crate::file_system_entry::FileSystemEntries;
use crate::server::Server;
// Import the Server struct from the server module
use std::error::Error;
// Import the Error trait for handling error types
use std::fs;
// Import filesystem module for directory operations
use std::path::{Path, PathBuf};
// Import Path and PathBuf for handling filesystem paths

// Define the trait ServerFilesystem with methods for server directory operations
pub trait ServerFilesystem {
    /// Creates a directory for the server, ensuring unique naming by appending an index if necessary.
    ///
    /// # Errors
    ///
    /// Returns an error if the directory cannot be created.
    fn create_server_directory(&mut self) -> Result<PathBuf, Box<dyn Error>>;

    /// Retrieves the server's icon path if it exists.
    fn get_server_icon(&self) -> Option<PathBuf>;

    /// Calculates and returns the total size of the server's files.
    fn calculate_server_size(&mut self) -> u64;

    fn remove_server_directory(&self) -> Result<(), Box<dyn Error>>;
    fn get_files(&self, subpath: impl AsRef<Path>) -> FileSystemEntries;

    fn relativize_paths(&mut self);
}

// Implementation of the ServerFilesystem trait for the Server struct
impl ServerFilesystem for Server<u64> {
    /// Creates a unique directory for the server, replacing invalid characters in the server name.
    ///
    /// The directory name is made from the server's name, which is cleaned to remove non-alphanumeric
    /// and non-whitespace characters. If a directory with the desired name exists, it appends an
    /// index to create a unique name.
    ///
    /// # Errors
    ///
    /// Returns an error if the directory cannot be created due to filesystem restrictions.
    fn create_server_directory(&mut self) -> Result<PathBuf, Box<dyn Error>> {
        // Clean the server's name to create a directory name
        let directory_name = &self
            .name
            .trim()
            .replace(
                |c: char| c.is_whitespace() || (!c.is_alphabetic() && !c.is_numeric() && c != '_'),
                "_",
            )
            .to_lowercase();
        // Join the cleaned name with the root "servers" directory
        let mut directory = Path::new("servers").join(directory_name);

        // Check and update the directory name if it already exists
        if directory.exists() {
            let mut index = 0;
            while directory.exists() {
                index += 1;
                directory = Path::new("servers").join(format!("{} ({})", directory_name, index));
            }
        }

        // Create the directory
        fs::create_dir_all(&directory)?;
        // Update the server's directory attribute
        self.directory = directory.clone();
        Ok(directory)
    }

    /// Attempts to retrieve the path to the server's icon.
    ///
    /// Returns a `PathBuf` to the icon if it exists, otherwise returns `None`.
    fn get_server_icon(&self) -> Option<PathBuf> {
        // Construct the path to the server icon
        let icon_path = self.directory.join("server-icon.png");

        // Check if the icon exists and return the path if it does
        if icon_path.exists() {
            Some(icon_path)
        } else {
            None
        }
    }

    /// Calculates and returns the total size in bytes of all files within the server's directory.
    ///
    /// This includes walking through the directory tree and summing the sizes of all the files.
    fn calculate_server_size(&mut self) -> u64 {
        // Create an iterator over files in the server's directory
        let files = walkdir::WalkDir::new(self.directory.clone());
        let mut size = 0;

        // Iterate over each entry, accumulate file sizes, and update the total size
        for entry in files.into_iter().filter_map(Result::ok) {
            if let Ok(metadata) = entry.metadata() {
                if entry.file_type().is_file() {
                    size += metadata.len();
                }
            }
        }
        self.size = size;
        size
    }

    fn remove_server_directory(&self) -> Result<(), Box<dyn Error>> {
        fs::remove_dir_all(&self.directory).map_err(|e| e.into())
    }

    fn get_files(&self, subpath: impl AsRef<Path>) -> FileSystemEntries {
        let mut entries = FileSystemEntries::from(self.directory.join(subpath.as_ref()));
        if let Some(parent) = entries.parent {
            entries.parent = parent.strip_prefix(&self.directory).ok().map(|i| i.to_path_buf())
        }

        for entry in entries.entries.iter_mut() {
            if let Some(path) = entry.path.strip_prefix(&self.directory).ok().map(|i| i.to_path_buf()) {
                entry.path = path;
            }
        }

        entries
    }

    fn relativize_paths(&mut self) {
        if let Some(start_script) = &self.start_script {
            self.start_script = Some(
                start_script
                    .strip_prefix(&self.directory)
                    .ok()
                    .map(|i| i.to_path_buf())
                    .unwrap_or(start_script.clone()),
            );
        }
        self.directory = self
            .directory
            .strip_prefix("servers")
            .ok()
            .map(|i| i.to_path_buf())
            .unwrap_or(self.directory.clone());
    }
}
