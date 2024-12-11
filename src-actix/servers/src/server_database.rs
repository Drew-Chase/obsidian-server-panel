use crate::server::Server;
use crate::server_status::ServerStatus;
use database::{create_appdb_connection, last_inserted_id};
use log::info;
use sqlite::State;
use std::error::Error;
use std::path::{Path, PathBuf};

/// Initializes the server database by creating necessary tables.
///
/// This function ensures that the `server` table is created in the database
/// if it doesn't already exist. It also creates a directory for storing server data.
///
/// # Returns
///
/// * `Result<(), Box<dyn Error>>` - Returns an empty result if successful or an error if any step fails.
///
/// # Errors
///
/// This function will return an error if the database connection fails,
/// if executing the SQL query fails, or if creating the directory fails.
pub fn initialize_server_database() -> Result<(), Box<dyn Error>> {
    // SQL query to create the `server` table with defined columns and constraints
    let query = r#"
        CREATE TABLE IF NOT EXISTS `server` (
            id INTEGER PRIMARY KEY AUTOINCREMENT,                       -- Unique identifier for each server, acting as the primary key
            name TEXT NOT NULL,                                         -- Name of the server, cannot be NULL
            owner INTEGER NOT NULL,                                     -- ID of the server's owner, referencing a user
            members TEXT,                                               -- List of member IDs as a serialized CSV string
            min_ram INTEGER NOT NULL,                                   -- Minimum RAM required (in MB), cannot be NULL
            max_ram INTEGER NOT NULL,                                   -- Maximum RAM allowed (in MB), cannot be NULL
            start_script TEXT,                                          -- Startup script path, can be NULL if no script is used
            minecraft_arguments TEXT,                                   -- Arguments for Minecraft, nullable
            java_arguments TEXT,                                        -- Arguments for Java execution, nullable
            minecraft_version TEXT NOT NULL,                            -- Version of Minecraft to use, nullable
            loader_type INTEGER NOT NULL,                               -- Type of server loader, corresponds to an integer value
            loader_version TEXT,                                        -- Version of the server loader, nullable
            directory TEXT NOT NULL,                                    -- Directory where the server is stored, path is not nullable
            java_runtime TEXT NULL DEFAULT NULL,                        -- Java runtime to use for the server, nullable,
            size INTEGER NOT NULL,                                      -- Size of the server in bytes, cannot be NULL
            auto_start BOOLEAN NOT NULL DEFAULT 0,                      -- Whether the server should automatically start on server startup, cannot be NULL
            created_at TIMESTAMP NOT NULL DEFAULT CURRENT_TIMESTAMP,    -- Timestamp of creation, stored in ISO 8601 format, cannot be NULL
            updated_at TIMESTAMP NOT NULL DEFAULT CURRENT_TIMESTAMP,    -- Timestamp of last update, stored in ISO 8601 format, cannot be NULL
            status TEXT                                                 -- Current status of the server, stored as a string (e.g., "active", "inactive"), nullable
        );
"#;
    let conn = create_appdb_connection()?; // Establish a connection to the application database
    conn.execute(query)?; // Execute the SQL query to create the table

    // Check if the 'servers' directory exists, if not, create it
    if !Path::exists("servers".as_ref()) {
        std::fs::create_dir("servers")?; // Create the 'servers' directory
        info!("Created server directory at: servers"); // Log the directory creation
    }
    Ok(()) // Return success
}

/// A trait for managing server databases, including adding, updating,
/// deleting, and retrieving server information.
pub trait ServerDatabase {
    /// Adds a new server to the system and returns the ID of the newly created server.
    ///
    /// # Returns
    ///
    /// * `Result<u64, Box<dyn Error>>` - On success, the server ID of the newly added server.
    ///
    /// # Errors
    ///
    /// This function will return an error if the server could not be added to the database.
    fn add(&mut self) -> Result<u64, Box<dyn Error>>;

    /// Updates the current server's details.
    ///
    /// # Returns
    ///
    /// * `Result<(), Box<dyn Error>>` - An empty result indicating success or an error if the update fails.
    ///
    /// # Errors
    ///
    /// Returns an error if the server update operation fails.
    fn update(&self) -> Result<(), Box<dyn Error>>;

    /// Deletes the current server from the system.
    ///
    /// # Returns
    ///
    /// * `Result<(), Box<dyn Error>>` - An empty result indicating success or an error if the deletion fails.
    ///
    /// # Errors
    ///
    /// Indicates an error if the server could not be deleted from the database.
    fn remove_from_database(&self) -> Result<(), Box<dyn Error>>;

    /// Retrieves a server by its unique ID.
    ///
    /// # Arguments
    ///
    /// * `id` - The ID of the server to retrieve.
    ///
    /// # Returns
    ///
    /// * `Result<Server<u64>, Box<dyn Error>>` - The server with the specified ID, or an error if not found.
    ///
    /// # Errors
    ///
    /// If the server cannot be retrieved, this function will return an error.
    fn get_server(id: u64) -> Result<Server<u64>, Box<dyn Error>>;

    /// Retrieves a server by its ID, ensuring the requester is either the owner or a member.
    ///
    /// # Arguments
    ///
    /// * `id` - The ID of the server to retrieve.
    /// * `owner_or_member` - The user ID of the owner or a member.
    ///
    /// # Returns
    ///
    /// * `Result<Server<u64>, Box<dyn Error>>` - The server with the specified ID for the given owner/member, or an error if unauthorized or not found.
    ///
    /// # Errors
    ///
    /// If the server cannot be retrieved or the authorization fails, an error is returned.
    fn get_owned_server(id: u64, owner_or_member: u64) -> Result<Server<u64>, Box<dyn Error>>;

    /// Retrieves a list of all available servers.
    ///
    /// # Returns
    ///
    /// * `Result<Vec<Server<u64>>, Box<dyn Error>>` - A vector of servers or an error if the list cannot be fetched.
    ///
    /// # Errors
    ///
    /// An error is returned if the server list cannot be retrieved.
    fn get_list_of_servers() -> Result<Vec<Server<u64>>, Box<dyn Error>>;

    /// Retrieves a list of servers owned or for which the user is a member.
    ///
    /// # Arguments
    ///
    /// * `owner_or_member` - The user ID of the owner or a member.
    ///
    /// # Returns
    ///
    /// * `Result<Vec<Server<u64>>, Box<dyn Error>>` - A vector of servers owned or accessible by the given user, or an error if the list cannot be fetched.
    ///
    /// # Errors
    ///
    /// Returns an error if the server list cannot be retrieved.
    fn get_list_of_owned_servers(owner_or_member: u64) -> Result<Vec<Server<u64>>, Box<dyn Error>>;
}

// Implementation of the ServerDatabase trait for Server<u64>
impl ServerDatabase for Server<u64> {
    /// Adds a new server entry to the database and assigns the last inserted ID to `self.id`.
    ///
    /// # Returns
    ///
    /// * `Result<u64, Box<dyn Error>>` - On success, returns the server ID of the newly added server.
    ///
    /// # Errors
    ///
    /// If any database operation fails, an error is returned.
    fn add(&mut self) -> Result<u64, Box<dyn Error>> {
        // Establish a connection to the application database
        let conn = create_appdb_connection()?;

        // Define an SQL query for inserting a new server record
        let query = r#"
  INSERT INTO server
  (name, owner, members, min_ram, max_ram, start_script, minecraft_arguments, 
  java_arguments, loader_type, loader_version, directory, status, java_runtime, size, minecraft_version)
  VALUES (?, ?, ?, ?, ?, ?, ?, ?, ?, ?, ?, ?, ?, ?, ?)
"#;

        // Prepare the SQL insert statement
        let mut statement = conn.prepare(query)?;

        // Bind the values of the server fields to the SQL statement parameters
        statement.bind((1, self.name.as_str()))?; // Bind server name
        statement.bind((2, self.owner as i64))?; // Bind owner ID
        statement.bind((
            3,
            self.members
                .iter()
                .map(u64::to_string)
                .collect::<Vec<String>>()
                .join(",")
                .as_str(),
        ))?; // Bind member IDs as a comma-separated string
        statement.bind((4, self.min_ram as i64))?; // Bind minimum RAM
        statement.bind((5, self.max_ram as i64))?; // Bind maximum RAM
        statement.bind((
            6,
            self.start_script
                .as_ref()
                .map(|i| i.to_str().unwrap_or(""))
                .unwrap_or(""),
        ))?; // Bind start script path
        statement.bind((7, self.minecraft_arguments.as_ref().unwrap_or(&"".to_string()).as_str()))?; // Bind Minecraft arguments
        statement.bind((8, self.java_arguments.as_ref().unwrap_or(&"".to_string()).as_str()))?; // Bind Java arguments
        statement.bind((9, self.loader_type as i64))?; // Bind loader type
        statement.bind((10, self.loader_version.as_ref().unwrap_or(&"".to_string()).as_str()))?; // Bind loader version
        statement.bind((11, self.directory.to_str().unwrap_or("")))?; // Bind directory path
        statement.bind((
            12,
            self.status
                .as_ref()
                .unwrap_or(&ServerStatus::Offline)
                .to_string()
                .as_str(),
        ))?; // Bind server status
        statement.bind((13, self.java_runtime.as_ref().unwrap_or(&PathBuf::from("")).to_str()))?; // Bind the java runtime path.
        statement.bind((14, self.size as i64))?; // Bind the server size
        statement.bind((15, self.minecraft_version.as_str()))?; // Bind Minecraft version

        // Execute the SQL statement
        statement.next()?;

        // Retrieve and set the newly inserted server ID into self.id
        self.id = last_inserted_id("server")?;

        // Return the newly assigned server ID
        Ok(self.id)
    }

    /// Updates the server entry in the database with the current server's data.
    ///
    /// # Returns
    ///
    /// * `Result<(), Box<dyn Error>>` - Indicates success or returns an error if the update fails.
    ///
    /// # Errors
    ///
    /// This function returns an error if the update operation fails.
    fn update(&self) -> Result<(), Box<dyn Error>> {
        // Establish a connection to the application database
        let conn = create_appdb_connection()?;

        // Prepare an SQL query to update a server record
        let query = r#"
UPDATE server SET
name = ?,
owner = ?,
members = ?,
min_ram = ?,
max_ram = ?,
start_script = ?,
minecraft_arguments = ?,
java_arguments = ?,
loader_type = ?,
loader_version = ?,
directory = ?,
status = ?,
java_runtime = ?,
size = ?,
minecraft_version = ?,
updated_at = CURRENT_TIMESTAMP,
auto_start = ?
WHERE id = ?
"#;

        // Prepare the SQL statement with the query
        let mut statement = conn.prepare(query)?;

        // Bind the server's name to the first placeholder (index 1)
        statement.bind((1, self.name.as_str()))?;

        // Bind the owner's ID to the second placeholder (index 2)
        statement.bind((2, self.owner as i64))?;

        // Bind the server's members as a comma-separated string to the third placeholder (index 3)
        statement.bind((
            3,
            self.members
                .iter()
                .map(u64::to_string)
                .collect::<Vec<String>>()
                .join(",")
                .as_str(),
        ))?;

        // Bind the minimum RAM requirement to the fourth placeholder (index 4)
        statement.bind((4, self.min_ram as i64))?;

        // Bind the maximum RAM requirement to the fifth placeholder (index 5)
        statement.bind((5, self.max_ram as i64))?;

        // Bind the start script path to the sixth placeholder (index 6)
        statement.bind((
            6,
            self.start_script
                .as_ref()
                .map(|i| i.to_str().unwrap_or(""))
                .unwrap_or(""),
        ))?;

        // Bind the Minecraft launch arguments to the seventh placeholder (index 7)
        statement.bind((7, self.minecraft_arguments.as_ref().unwrap_or(&"".to_string()).as_str()))?;

        // Bind the Java launch arguments to the eighth placeholder (index 8)
        statement.bind((8, self.java_arguments.as_ref().unwrap_or(&"".to_string()).as_str()))?;

        // Bind the loader type to the ninth placeholder (index 9)
        statement.bind((9, self.loader_type as i64))?;

        // Bind the loader version to the tenth placeholder (index 10)
        statement.bind((10, self.loader_version.as_ref().unwrap_or(&"".to_string()).as_str()))?;

        // Bind the server directory path to the eleventh placeholder (index 11)
        statement.bind((11, self.directory.to_str().unwrap_or("")))?;

        // Bind the server status to the twelfth placeholder (index 12)
        statement.bind((
            12,
            self.status
                .as_ref()
                .unwrap_or(&ServerStatus::Offline)
                .to_string()
                .as_str(),
        ))?;

        // Bind the Java runtime path to the thirteenth placeholder (index 13)
        statement.bind((13, self.java_runtime.as_ref().unwrap_or(&PathBuf::from("")).to_str()))?;

        // Bind the server size to the fourteenth placeholder (index 14)
        statement.bind((14, self.size as i64))?;

        // Bind the Minecraft version to the fifteenth placeholder (index 15)
        statement.bind((15, self.minecraft_version.as_str()))?;

        // Bind the auto_start field to the sixteenth placeholder (index 16)
        statement.bind((16, self.auto_start as i64))?;

        // Bind the server ID to the seventeenth placeholder (index 17) to specify which record to update
        statement.bind((17, self.id as i64))?;

        // Execute the next statement in the prepared sequence
        statement.next()?;

        // Return a successful result
        Ok(())
    }

    /// Deletes the server entry from the database for the current instance.
    ///
    /// # Errors
    ///
    /// Returns an error if:
    /// - There was an error creating the database connection.
    /// - An error occurred preparing or executing the delete statement.
    fn remove_from_database(&self) -> Result<(), Box<dyn Error>> {
        // Establish a connection to the database
        let conn = create_appdb_connection()?;

        // Prepare the delete query to remove a server by its ID
        let query = r#"DELETE FROM server WHERE id = ?"#;
        let mut statement = conn.prepare(query)?;

        // Bind the server ID to the query
        statement.bind((1, self.id as i64))?;

        // Execute the statement
        statement.next()?;
        Ok(())
    }

    /// Retrieves a server by its ID from the database.
    ///
    /// # Arguments
    ///
    /// * `id` - The unique identifier of the desired server.
    ///
    /// # Returns
    ///
    /// A result containing the `Server` object if successful, or an error if any database operations fail.
    fn get_server(id: u64) -> Result<Server<u64>, Box<dyn Error>> {
        // Establish a connection to the database
        let conn = create_appdb_connection()?;

        // Prepare the query to retrieve a server by its ID
        let query = r#"SELECT * FROM server WHERE id = ?"#;
        let mut statement = conn.prepare(query)?;

        // Bind the server ID to the query
        statement.bind((1, id as i64))?;

        // Move to the next row of results
        statement.next()?;
        get_server_from_statement(&mut statement)
    }

    /// Retrieves a server that is owned by or accessible to a user by ID.
    ///
    /// # Arguments
    ///
    /// * `id` - The unique identifier of the server.
    /// * `owner_or_member` - The ID of the owner or a member of the server.
    ///
    /// # Returns
    ///
    /// A result containing the `Server` object if successful, or an error if any database operations fail.
    fn get_owned_server(id: u64, owner_or_member: u64) -> Result<Server<u64>, Box<dyn Error>> {
        // Establish a connection to the database
        let conn = create_appdb_connection()?;

        // Prepare the query to retrieve a server based on user ownership or membership
        let query = r#"SELECT * FROM server WHERE id = ? and owner = ? or members like ? LIMIT 1"#;
        let mut statement = conn.prepare(query)?;

        // Bind the server ID, owner ID, and member ID to the query
        statement.bind((1, id as i64))?;
        statement.bind((2, owner_or_member as i64))?;
        statement.bind((3, owner_or_member as i64))?;

        // Move to the next row of results
        statement.next()?;
        get_server_from_statement(&mut statement)
    }

    /// Retrieves a list of all servers from the database.
    ///
    /// # Returns
    ///
    /// A result containing a vector of `Server` objects if successful, or an error if any database operations fail.
    fn get_list_of_servers() -> Result<Vec<Server<u64>>, Box<dyn Error>> {
        // Establish a connection to the database
        let conn = create_appdb_connection()?;

        // Prepare the query to retrieve all servers
        let query = r#"SELECT * FROM server"#;
        let mut statement = conn.prepare(query)?;

        // Container for retrieved server records
        let mut servers: Vec<Server<u64>> = Vec::new();

        // Iterate over all rows of results, pushing each server into the vector
        while let State::Row = statement.next()? {
            servers.push(get_server_from_statement(&mut statement)?);
        }
        Ok(servers)
    }

    /// Retrieves a list of servers owned or accessible by a given user.
    ///
    /// # Arguments
    ///
    /// * `owner_or_member` - The ID of the owner or a member of the server.
    ///
    /// # Returns
    ///
    /// A result containing a vector of `Server` objects if successful, or an error if any database operations fail.
    fn get_list_of_owned_servers(owner_or_member: u64) -> Result<Vec<Server<u64>>, Box<dyn Error>> {
        // Establish a connection to the database
        let conn = create_appdb_connection()?;

        // Prepare the query to retrieve servers owned by or accessible to a specific user
        let query = r#"SELECT * FROM server WHERE owner = ? or members like ?"#;
        let mut statement = conn.prepare(query)?;

        // Bind the owner ID and member ID to the query
        statement.bind((1, owner_or_member as i64))?;
        statement.bind((2, owner_or_member as i64))?;

        // Container for retrieved server records
        let mut servers: Vec<Server<u64>> = Vec::new();

        // Iterate over all rows of results, pushing each server into the vector
        while let State::Row = statement.next()? {
            servers.push(get_server_from_statement(&mut statement)?);
        }
        Ok(servers)
    }
}

/// Converts a SQLite statement result into a `Server` instance.
///
/// # Arguments
///
/// * `statement` - A mutable reference to a SQLite statement from which the server details are extracted.
///
/// # Returns
///
/// * `Result<Server<u64>, Box<dyn Error>>` - Returns a `Server` instance on success, or an error if something goes wrong during the extraction.
///
/// **Note**: Assumes that the provided statement yields rows where each column mapping corresponds to server fields.
fn get_server_from_statement(statement: &mut sqlite::Statement) -> Result<Server<u64>, Box<dyn Error>> {
    Ok(Server {
        // Server ID: Parse the "id" column from the statement and convert to u64.
        id: statement.read::<i64, _>("id")? as u64,

        // Server Name: Directly read the "name" column as a String.
        name: statement.read::<String, _>("name")?,

        // Server Owner ID: Parse the "owner" column and convert to u64.
        owner: statement.read::<i64, _>("owner")? as u64,

        // Server Member IDs: Read the "members" column as a String, split by commas, and parse each into u64.
        members: statement
            .read::<String, _>("members")?
            .split(',')
            .filter_map(|s| s.parse::<u64>().ok())
            .collect(),

        // RAM Specifications: Minimum RAM is converted from i64 to u64.
        min_ram: statement.read::<i64, _>("min_ram")? as u64,

        // Maximum RAM is also converted from i64 to u64.
        max_ram: statement.read::<i64, _>("max_ram")? as u64,

        // Auto Start: Read and check if "auto_start" is non-zero to be true.
        auto_start: statement.read::<i64, _>("auto_start")? != 0,

        // Start Script Path: Optional, map the result to PathBuf if present.
        start_script: statement.read::<String, _>("start_script").ok().map(PathBuf::from),

        // Minecraft and Java Arguments: Read these columns optionally as Strings.
        minecraft_arguments: statement.read::<String, _>("minecraft_arguments").ok(),
        java_arguments: statement.read::<String, _>("java_arguments").ok(),

        // Loader Type: Parse the "loader_type" from i64 to u8.
        loader_type: statement.read::<i64, _>("loader_type")? as u8,

        // Loader Version: Optionally read the "loader_version" as a String.
        loader_version: statement.read::<String, _>("loader_version").ok(),

        // Directory Path: Convert the "directory" column to PathBuf.
        directory: PathBuf::from(statement.read::<String, _>("directory")?),

        // Timestamps: Read the "created_at" and "updated_at" as Strings.
        created_at: statement.read::<String, _>("created_at")?,
        updated_at: statement.read::<String, _>("updated_at")?,

        // Status: Optionally parse the "status" as a ServerStatus if it's present and valid.
        status: statement.read::<String, _>("status").ok().and_then(|s| s.parse().ok()),

        java_runtime: statement
            .read::<String, _>("java_runtime")
            .ok()
            .and_then(|s| s.parse().ok()),

        // Server Size: Read "size" column from the statement and convert it to u64.
        size: statement.read::<i64, _>("size")? as u64,

        // Minecraft Version: Read the "minecraft_version" column as a String.
        minecraft_version: statement.read::<String, _>("minecraft_version")?,

        // I/O streams: Initialize as None, as they are not detailed in the statement.
        pid: None,
    })
}
