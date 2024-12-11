use crate::server::Server;
use std::collections::HashMap;
use std::error::Error;
use std::io::Write;
use std::path::Path;

pub trait ServerProperties {
    /// Creates a properties file for the server.
    ///
    /// This function is used to initialize or create a properties file where
    /// various settings and configurations for the server can be stored.
    ///
    /// # Errors
    /// Returns an error if the file creation process fails (e.g., due to permission issues,
    /// I/O errors, or other underlying system errors).
    fn create_properties_file(&self) -> Result<(), Box<dyn Error>>;

    /// Retrieves all properties from the properties file.
    ///
    /// The properties are returned as a key-value pair stored in a `HashMap`.
    /// Each key and its associated value represent a property stored in the file.
    ///
    /// # Errors
    /// Returns an error if the properties file cannot be read (e.g., due to being missing,
    /// corrupted, or file access errors).
    ///
    /// # Returns
    /// A `HashMap` containing all properties as key-value pairs.
    fn get_properties(&self) -> Result<HashMap<String, String>, Box<dyn Error>>;

    /// Retrieves the value of a specific property by its key.
    ///
    /// This function allows access to a single specific property based on
    /// the provided key. If the property is found, its value is returned.
    ///
    /// # Parameters
    /// - `key`: The key of the property to retrieve.
    ///
    /// # Errors
    /// Returns an error if there is an issue reading the properties or if the key
    /// does not exist.
    ///
    /// # Returns
    /// A `String` containing the value of the requested property.
    fn get_property(&self, key: &str) -> Result<String, Box<dyn Error>>;

    /// Sets a property in the properties file.
    ///
    /// This function updates or adds a new property with the given key and value.
    /// If the key already exists, it overwrites the existing value with the provided one.
    /// If the key does not exist, it creates a new entry for the property.
    ///
    /// # Parameters
    /// - `key`: The key of the property to set.
    /// - `value`: The value to associate with the key.
    ///
    /// # Errors
    /// Returns an error if the operation fails (e.g., due to file access issues or
    /// invalid input).
    fn set_property(&self, key: &str, value: &str) -> Result<(), Box<dyn Error>>;
    /// Sets a range of properties by taking a `HashMap` of key-value pairs and applying them.
    ///
    /// # Arguments
    /// * `values` - A `HashMap` containing property names as keys and their corresponding
    ///              values as strings. These properties will be set within the object.
    ///
    /// # Returns
    /// * `Ok(())` - If all the properties were successfully set.
    /// * `Err(Box<dyn Error>)` - If any errors occurred during the execution.
    ///
    /// # Errors
    /// This function may return an error if interacting with underlying components
    /// (e.g., database, configuration files, etc.) fails or invalid values are provided.
    ///
    /// # Example
    /// ```no-code
    /// let mut properties = HashMap::new();
    /// properties.insert("max_connections".to_string(), "100".to_string());
    /// properties.insert("log_level".to_string(), "debug".to_string());
    ///
    /// object.set_property_range(properties)?;
    /// ```
    fn set_property_range(&self, values: HashMap<String, String>) -> Result<(), Box<dyn Error>>;
}

impl ServerProperties for Server<u64> {
    fn create_properties_file(&self) -> Result<(), Box<dyn Error>> {
        // Construct the file path for the server.properties file within the server's directory.
        let file_path = self.directory.join("server.properties");

        // Check if the server.properties file already exists.
        if file_path.exists() {
            // If the file exists, return early with an Ok result.
            return Ok(());
        }

        // If the file does not exist, create a new empty properties file.
        // This saves an empty HashMap into the specified server.properties filepath.
        save_properties(&HashMap::new(), file_path)?;

        // Return Ok if the process completes successfully.
        Ok(())
    }

    fn get_properties(&self) -> Result<HashMap<String, String>, Box<dyn Error>> {
        // Create a mutable HashMap to store key-value pairs read from the file
        let mut values: HashMap<String, String> = HashMap::new();

        // Define the path to the file "server.properties" within the specified directory
        let file_path = self.directory.join("server.properties");

        // If the file does not exist, return the empty HashMap
        if !file_path.exists() {
            return Ok(values);
        }

        // Read the contents of the file as a string
        let file_contents = std::fs::read_to_string(file_path)?;

        // Iterate over each line in the file, splitting it into key-value pairs
        for line in file_contents.lines() {
            // Remove any leading or trailing whitespace from the current line
            let line = line.trim();

            // Skip lines that start with '#' as they are comments
            if line.starts_with('#') {
                continue;
            }

            // Attempt to split the line into a key-value pair at the first '=' character
            if let Some((key, value)) = line.split_once('=') {
                // Trim whitespace from both the key and value, and insert them into the HashMap
                values.insert(key.trim().to_string(), value.trim().to_string());
            }
        }

        // Return the populated HashMap of key-value pairs
        Ok(values)
    }

    fn get_property(&self, key: &str) -> Result<String, Box<dyn Error>> {
        // Retrieve the properties from the `ServerProperties` implementation as a HashMap
        self.get_properties()?
            // Attempt to get the value associated with the specified key
            .get(key)
            // Clone the value to return an owned copy instead of a reference
            .cloned()
            // If the key is not found, return a descriptive error detailing the missing property key
            .ok_or_else(|| format!("Property {} not found", key).into())
    }

    fn set_property(&self, key: &str, value: &str) -> Result<(), Box<dyn Error>> {
        // Retrieve the current server properties as a mutable HashMap
        let mut properties = self.get_properties()?;

        // Insert the new property key-value pair into the HashMap
        properties.insert(key.to_string(), value.to_string());

        // Save the updated properties back to the "server.properties" file in the specified directory
        save_properties(&properties, self.directory.join("server.properties"))?;

        // Indicate that the operation completed successfully
        Ok(())
    }

    fn set_property_range(&self, values: HashMap<String, String>) -> Result<(), Box<dyn Error>> {
        // Retrieve the current server properties as a mutable HashMap
        let mut properties = self.get_properties()?;

        // Extend the existing properties HashMap with the new key-value pairs
        properties.extend(values);

        // Save the updated properties back to the "server.properties" file in the specified directory
        save_properties(&properties, self.directory.join("server.properties"))?;

        // Indicate that the operation completed successfully
        Ok(())
    }
}

/// Saves a HashMap of properties to a file in the Minecraft server properties format.
///
/// # Arguments
/// * `properties` - A reference to a HashMap where each key-value pair represents a property.
/// * `file_path` - The path where the properties file should be created or overwritten.
///
/// # Returns
/// * `Result<(), Box<dyn Error>>` - Returns `Ok(())` if the file was successfully created and written to,
///   or an error wrapped in a `Box<dyn Error>` if any failure occurs.
///
/// # Errors
/// This function will return an error if:
/// * The file cannot be created or opened.
/// * Writing to the file fails.
/// * Flushing data to the file fails.
///
/// # Notes
/// * The file created will include a header to indicate it is for Minecraft server properties.
/// * Each key-value pair is written in the format `key=value`, followed by a newline.
fn save_properties(properties: &HashMap<String, String>, file_path: impl AsRef<Path>) -> Result<(), Box<dyn Error>> {
    // Open or create a file at the specified file path. An error will be returned if the path is invalid.
    let mut file = std::fs::File::create(file_path)?;

    // Create a mutable vector to hold the buffer for the file contents (in bytes).
    let mut buffer: Vec<u8> = Vec::new();

    // Add a standard header for the Minecraft server properties file.
    // This header contains descriptive information about the file's purpose.
    buffer.extend_from_slice(b"# Minecraft Server Properties\n# Generated By Obsidian Server Portal\n");

    // Iterate over the provided key-value pairs in the properties HashMap.
    for (key, value) in properties {
        // Format each property into the "key=value" format and add a newline at the end.
        // Append the formatted data to the byte buffer.
        buffer.extend_from_slice(format!("{}={}\n", key, value).as_bytes());
    }

    // Write the accumulated data in the buffer to the file.
    file.write_all(&buffer)?;

    // Ensure all buffered data has been written to the file by explicitly flushing it.
    file.flush()?;

    // Return success if all operations are completed without errors.
    Ok(())
}
