use crate::file_system_entry::FileSystemEntries;
use crate::server::Server;
use log::error;
use notify::{RecursiveMode, Watcher};
use std::error::Error;
use std::fs;
use std::fs::File;
use std::io::{Read, Seek, SeekFrom};
use std::path::{Path, PathBuf};

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

    /// Removes the server directory and its contents from the file system.
    ///
    /// # Returns
    /// - `Ok(())` if the server directory was successfully removed.
    /// - `Err(Box<dyn Error>)` if an error occurred during the removal process.
    fn remove_server_directory(&self) -> Result<(), Box<dyn Error>>;

    /// Retrieves the file system entries (files and directories) within a specified subpath.
    ///
    /// # Parameters
    /// - `subpath`: The path relative to the server's root directory to search for files and directories.
    ///
    /// # Returns
    /// - A `FileSystemEntries` object representing the files and directories found in the specified subpath.
    fn get_files(&self, subpath: impl AsRef<Path>) -> FileSystemEntries;

    /// Retrieves the entries within a specified subpath within an archive.
    ///
    /// # Parameters
    /// - `subpath`: The relative path inside the archive to retrieve entries for.
    ///
    /// # Returns
    /// - A `FileSystemEntries` object containing the files and directories found in the given subpath of the archive.
    fn get_archive_entries(&self, subpath: impl AsRef<Path>) -> FileSystemEntries;

    /// Archives the specified file system paths into a single archive file.
    ///
    /// # Parameters
    /// - `subpaths`: A vector of paths to include in the archive.
    /// - `archive_path`: The destination path where the archive file will be created.
    ///
    /// # Returns
    /// - `Ok(())` if the paths were successfully archived.
    /// - `Err(Box<dyn Error>)` if an error occurred.
    fn archive_paths(&self, subpaths: Vec<PathBuf>, archive_path: impl AsRef<Path>) -> Result<(), Box<dyn Error>>;

    /// Extracts the contents of an archive into the specified destination directory.
    ///
    /// # Parameters
    /// - `archive_path`: The path to the archive to be extracted.
    /// - `destination_path`: The path to the directory where the contents will be extracted.
    ///
    /// # Returns
    /// - `Ok(())` if the archive was successfully extracted.
    /// - `Err(Box<dyn Error>)` if an error occurred during extraction.
    fn extract_archive(
        &self,
        archive_path: impl AsRef<Path>,
        destination_path: impl AsRef<Path>,
    ) -> Result<(), Box<dyn Error>>;

    /// Reads the contents of a log file and provides updates via a callback function whenever the file changes.
    ///
    /// This function monitors the file for changes and reads new content incrementally.
    /// It blocks the current thread and should be executed in a separate thread to avoid
    /// hindering the main application's responsiveness.
    ///
    /// # Parameters
    /// - `log_path`: The path to the log file to be monitored.
    /// - `on_update`: A callback function invoked whenever new data is detected in the log file.
    ///   The callback receives a `&str` containing the new log data.
    ///   - Returning `true` from the callback will continue monitoring the file.
    ///   - Returning `false` will stop the monitoring operation.
    ///
    /// # Notes
    /// - The function utilizes the `notify` crate to watch for changes to the file.
    /// - If the log file is truncated or reset, the function automatically detects the
    ///   truncation and resumes reading from the beginning.
    /// - This function is thread-safe and designed to handle asynchronous file updates.
    ///
    /// # Returns
    /// - `Ok(String)` containing the log file's full contents up to the termination point.
    /// - `Err(Box<dyn Error>)` if an error arises during file reading or monitoring.
    fn read_log_file(
        &self,
        log_path: impl AsRef<Path>,
        on_update: impl Fn(&str) -> bool + Send + Sync + 'static,
    ) -> Result<String, Box<dyn Error>>;

    /// Converts file paths within the current object to relative paths.
    ///
    /// This function is typically used to standardize paths or ensure portability.
    /// It may modify the internal state of the object to store paths in a relative format.
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

    fn get_archive_entries(&self, subpath: impl AsRef<Path>) -> FileSystemEntries {
        todo!()
    }

    fn archive_paths(&self, subpaths: Vec<PathBuf>, archive_path: impl AsRef<Path>) -> Result<(), Box<dyn Error>> {
        todo!()
    }

    fn extract_archive(
        &self,
        archive_path: impl AsRef<Path>,
        destination_path: impl AsRef<Path>,
    ) -> Result<(), Box<dyn Error>> {
        todo!()
    }

    fn read_log_file(
        &self,
        log_path: impl AsRef<Path>,
        on_update: impl Fn(&str) -> bool + Send + Sync + 'static,
    ) -> Result<String, Box<dyn Error>> {
        // Open the file specified by the path (log_path)
        let log_path = self.directory.join("logs").join(&log_path);
        let mut file = File::open(&log_path)?;

        // Prepare a buffer (String) to read the file content into
        let mut contents = String::new();

        // Read the entire contents of the file into the buffer
        file.read_to_string(&mut contents)?;

        // Call the on_update function with the file's current contents as input
        // The function returns a boolean indicating whether to continue monitoring (true)
        // or stop the operation (false).
        if !on_update(&contents) {
            return Ok(contents);
        }

        // Keep track of the byte position of the file from which new content should be read
        let mut last_read = file.metadata()?.len(); // Retrieve file size for tracking

        // Set up a channel for communication between the file watcher and the main event loop
        let (tx, rx) = std::sync::mpsc::channel();

        // Create a file watcher using the notify crate's recommended watcher
        let mut watcher = notify::recommended_watcher(tx)?;

        // Start watching the log file for changes (non-recursively)
        watcher.watch(log_path.as_ref(), RecursiveMode::NonRecursive)?;

        // Begin the main loop to handle file change events
        loop {
            // Wait for an event from the file watcher
            match rx.recv() {
                Ok(event) => match event {
                    Ok(_) => {
                        // Re-open the file in case it was changed/truncated
                        let mut file = File::open(&log_path)?;

                        // Retrieve the current file size
                        let size = file.metadata()?.len();

                        // Check if the file was truncated. If so, reset the last_read position to 0
                        if last_read > size {
                            last_read = 0;
                        }

                        // Seek to the last read position in the file
                        file.seek(SeekFrom::Start(last_read))?;

                        // Prepare a buffer to read new contents from the file
                        let mut new_contents = String::new();

                        // Read new content starting from the last read position
                        file.read_to_string(&mut new_contents)?;

                        // If there is new content, call the on_update function and update last_read
                        // The function returns a boolean indicating whether to continue monitoring (true)
                        // or stop the operation (false).
                        if !new_contents.is_empty() {
                            if !on_update(&new_contents)
                            // Notify about the new content
                            {
                                break;
                            }
                            last_read = file.metadata()?.len(); // Update last_read to the new file size
                        }
                    }
                    Err(e) => {
                        // Log an error if an event error occurs and break the loop
                        error!("Error: {:?}", e);
                        break;
                    }
                },
                Err(e) => {
                    // Log an error if receiving from the channel fails and break the loop
                    error!("Error: {:?}", e);
                    break;
                }
            }
        }

        // Return success when the loop ends (if it exits cleanly)
        Ok(contents)
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
