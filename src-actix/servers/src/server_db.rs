use log::{debug, error, info};
use serde_derive::{Deserialize, Serialize};
use sqlite::State;

#[derive(Debug, Serialize, Deserialize)]
pub struct Server {
	pub id: i32,
	pub name: String,
	pub instance: Option<i32>,
	pub owner: i32,
	pub size: i64,
	pub auto_start: bool,
	pub min_ram: i64,
	pub max_ram: i64,
	pub executable: Option<String>,
	pub minecraft_arguments: Option<String>,
	pub java_arguments: Option<String>,
	pub minecraft_version: Option<String>,
	pub loader: i8,
	pub loader_version: Option<String>,
	pub created_at: String,
	pub updated_at: String,
}

/// Initializes the servers database by creating the `servers` table if it does not exist.
///
/// # Panics
///
/// This function will panic if it fails to connect to the database.
pub fn initialize() {
	info!("Initializing servers database");
	let conn = create_connection().expect("Failed to connect to database");
	match conn.execute(
		"
			CREATE TABLE IF NOT EXISTS servers
			(
				id	   INTEGER PRIMARY KEY AUTOINCREMENT,
				name	 TEXT	NOT NULL,
				instance INTEGER DEFAULT NULL,
				owner	INTEGER NOT NULL,
				size	 INTEGER DEFAULT 0,
				auto_start INTEGER DEFAULT 0,
				min_ram INTEGER DEFAULT 0,
				max_ram INTEGER DEFAULT 0,
				executable TEXT DEFAULT NULL,
				minecraft_arguments TEXT DEFAULT NULL,
				java_arguments TEXT DEFAULT NULL,
				minecraft_version TEXT DEFAULT NULL,
				loader INTEGER DEFAULT 0,
				loader_version TEXT DEFAULT NULL,
				created_at TIMESTAMP DEFAULT CURRENT_TIMESTAMP,
				updated_at TIMESTAMP DEFAULT CURRENT_TIMESTAMP
			);
		",
	) {
		Ok(_) => info!("Successfully created or verified the servers table."),
		Err(e) => error!("Failed to create servers table: {}", e),
	}
}

/// Creates a new server entry in the database.
///
/// # Arguments
///
/// * `name` - The name of the server.
/// * `owner` - The ID of the server owner.
///
/// # Returns
///
/// * `Ok(Server)` - If the server was successfully created and retrieved.
/// * `Err(String)` - If there was an error creating the server. The error message is returned as a `String`.
///
/// # Example
///
/// ```
/// match create_server("Test Server", 1) {
///     Ok(server) => println!("Server created successfully: {:?}", server),
///     Err(e) => println!("Error creating server: {}", e),
/// }
/// ```
pub fn create_server(name: &str, owner: i64) -> Result<Server, String>
{
	info!("Creating server with name: {} and owner: {}", name, owner);
	let conn = match create_connection() {
		Ok(conn) => {
			debug!("Successfully connected to the database.");
			conn
		},
		Err(e) => {
			error!("Failed to connect to database: {}", e);
			return Err(format!("Failed to connect to database: {}", e));
		}
	};

	let mut statement = match conn.prepare("INSERT INTO servers (name, owner) VALUES (?, ?, ?)") {
		Ok(stmt) => {
			debug!("Successfully prepared the INSERT statement.");
			stmt
		},
		Err(e) => {
			error!("Failed to prepare statement: {}", e);
			return Err(format!("Failed to prepare statement: {}", e));
		}
	};

	statement.bind((1, name)).map_err(|e| {
		error!("Failed to bind name: {}", e);
		format!("Failed to bind name: {}", e)
	})?;
	statement.bind((2, owner)).map_err(|e| {
		error!("Failed to bind owner: {}", e);
		format!("Failed to bind owner: {}", e)
	})?;
	statement.next().map_err(|e| {
		error!("Failed to execute statement: {}", e);
		format!("Failed to execute statement: {}", e)
	})?;

	debug!("Getting the inserted id for the new server.");
	let mut statement = conn.prepare("select seq from sqlite_sequence WHERE name = 'servers'").map_err(|e| {
		error!("Failed to get inserted id: {}", e);
		format!("Failed to get inserted id: {}", e)
	})?;
	statement.next().map_err(|e| {
		error!("Failed to get inserted id: {}", e);
		format!("Failed to get inserted id: {}", e)
	})?;
	let id = statement.read::<i64, _>("seq").map_err(|e| {
		error!("Failed to get inserted id: {}", e);
		format!("Failed to get inserted id: {}", e)
	})?;

	match get_server_by_id(id) {
		Some(server) => {
			info!("Successfully created server with id: {}", id);
			Ok(server)
		},
		None => {
			error!("Server id not present in server list. This usually means that the server failed to be inserted into the database.");
			Err("Something went wrong, server id not present in server list. This usually means that the server failed to be inserted into the database.".to_string())
		}
	}
}

/// Fetches the server with the specified ID.
///
/// # Arguments
///
/// * `id` - The ID of the server to fetch.
///
/// # Returns
///
/// * `Some(Server)` - If the server was successfully fetched.
/// * `None` - If there was an error or the server does not exist.
///
/// # Example
///
/// ```
/// if let Some(server) = get_server_by_id(1) {
///     println!("Server fetched successfully.");
/// } else {
///     println!("Error fetching server or server does not exist.");
/// }
/// ```
pub fn get_server_by_id(id: i64) -> Option<Server>
{
	debug!("Fetching server with id: {}", id);
	let conn = match create_connection() {
		Ok(conn) => {
			debug!("Successfully connected to the database.");
			conn
		},
		Err(_) => {
			error!("Failed to connect to database.");
			return None;
		}
	};

	let mut statement = match conn.prepare("SELECT * FROM servers WHERE id = ?") {
		Ok(stmt) => {
			debug!("Successfully prepared the SELECT statement.");
			stmt
		},
		Err(_) => {
			error!("Failed to prepare the SELECT statement.");
			return None;
		}
	};

	statement.bind((1, id)).ok()?;
	statement.next().ok()?;

	let server = Server {
		id: statement.read::<i64, _>("id").ok()? as i32,
		name: statement.read::<String, _>("name").ok()?,
		instance: Some(statement.read::<i64, _>("instance").ok()? as i32),
		owner: statement.read::<i64, _>("owner").ok()? as i32,
		size: statement.read::<i64, _>("size").ok()?,
		auto_start: statement.read::<i64, _>("auto_start").ok()? == 1,
		min_ram: statement.read::<i64, _>("min_ram").ok()?,
		max_ram: statement.read::<i64, _>("max_ram").ok()?,
		executable: statement.read::<Option<String>, _>("executable").ok()?,
		minecraft_arguments: statement.read::<Option<String>, _>("minecraft_arguments").ok()?,
		java_arguments: statement.read::<Option<String>, _>("java_arguments").ok()?,
		minecraft_version: statement.read::<Option<String>, _>("minecraft_version").ok()?,
		loader: statement.read::<i64, _>("loader").ok()? as i8,
		loader_version: statement.read::<Option<String>, _>("loader_version").ok()?,
		created_at: statement.read::<String, _>("created_at").ok()?,
		updated_at: statement.read::<String, _>("updated_at").ok()?,
	};

	info!("Successfully fetched server with id: {}", id);
	Some(server)
}

/// Sets the Java arguments for the specified server.
///
/// # Arguments
///
/// * `id` - The ID of the server.
/// * `java_arguments` - The Java arguments to set for the server.
///
/// # Returns
///
/// * `Ok(())` - If the Java arguments were successfully set.
/// * `Err(String)` - If there was an error setting the Java arguments. The error message is returned as a `String`.
///
/// # Example
///
/// ```
/// match set_java_arguments(1, "-Xmx1024M -Xms512M") {
///     Ok(()) => println!("Java arguments set successfully."),
///     Err(e) => println!("Error setting Java arguments: {}", e),
/// }
/// ```
pub fn set_java_arguments(id: i32, java_arguments: &str) -> Result<(), String>
{
	info!("Setting Java arguments for server with id: {} to {}", id, java_arguments);
	let conn = match create_connection() {
		Ok(conn) => {
			debug!("Successfully connected to the database.");
			conn
		},
		Err(e) => {
			error!("Failed to connect to the database: {}", e);
			return Err(format!("Failed to connect to database: {}", e));
		}
	};

	let mut statement = match conn.prepare("UPDATE servers SET java_arguments = ? WHERE id = ?") {
		Ok(stmt) => {
			debug!("Successfully prepared the UPDATE statement.");
			stmt
		},
		Err(e) => {
			error!("Failed to prepare the UPDATE statement: {}", e);
			return Err(format!("Failed to prepare statement: {}", e));
		}
	};
	statement.bind((1, java_arguments)).map_err(|e| {
		error!("Failed to bind java_arguments: {}", e);
		format!("Failed to bind java_arguments: {}", e)
	})?;
	statement.bind((2, id as i64)).map_err(|e| {
		error!("Failed to bind id: {}", e);
		format!("Failed to bind id: {}", e)
	})?;
	statement.next().map_err(|e| {
		error!("Failed to execute statement: {}", e);
		format!("Failed to execute statement: {}", e)
	})?;

	info!("Successfully set Java arguments for server with id: {}", id);
	Ok(())
}

/// Sets the Minecraft version for the specified server.
///
/// # Arguments
///
/// * `id` - The ID of the server.
/// * `minecraft_version` - The Minecraft version to set for the server.
///
/// # Returns
///
/// * `Ok(())` - If the Minecraft version was successfully set.
/// * `Err(String)` - If there was an error setting the Minecraft version. The error message is returned as a `String`.
///
/// # Example
///
/// ```
/// match set_minecraft_version(1, "1.17.1") {
///     Ok(()) => println!("Minecraft version set successfully."),
///     Err(e) => println!("Error setting Minecraft version: {}", e),
/// }
/// ```
pub fn set_minecraft_version(id: i32, minecraft_version: &str) -> Result<(), String>
{
	info!("Setting Minecraft version for server with id: {} to {}", id, minecraft_version);
	let conn = match create_connection() {
		Ok(conn) => {
			debug!("Successfully connected to the database.");
			conn
		},
		Err(e) => {
			error!("Failed to connect to the database: {}", e);
			return Err(format!("Failed to connect to database: {}", e));
		}
	};

	let mut statement = match conn.prepare("UPDATE servers SET minecraft_version = ? WHERE id = ?") {
		Ok(stmt) => {
			debug!("Successfully prepared the UPDATE statement.");
			stmt
		},
		Err(e) => {
			error!("Failed to prepare the UPDATE statement: {}", e);
			return Err(format!("Failed to prepare statement: {}", e));
		}
	};
	statement.bind((1, minecraft_version)).map_err(|e| {
		error!("Failed to bind minecraft_version: {}", e);
		format!("Failed to bind minecraft_version: {}", e)
	})?;
	statement.bind((2, id as i64)).map_err(|e| {
		error!("Failed to bind id: {}", e);
		format!("Failed to bind id: {}", e)
	})?;
	statement.next().map_err(|e| {
		error!("Failed to execute statement: {}", e);
		format!("Failed to execute statement: {}", e)
	})?;

	info!("Successfully set Minecraft version for server with id: {}", id);
	Ok(())
}

/// Sets the loader and loader version for the specified server.
///
/// # Arguments
///
/// * `id` - The ID of the server.
/// * `loader` - The loader to set for the server.
/// * `loader_version` - The loader version to set for the server.
///
/// # Returns
///
/// * `Ok(())` - If the loader and loader version were successfully set.
/// * `Err(String)` - If there was an error setting the loader or loader version. The error message is returned as a `String`.
///
/// # Example
///
/// ```
/// match set_loader(1, 2, "0.11.2") {
///     Ok(()) => println!("Loader set successfully."),
///     Err(e) => println!("Error setting loader: {}", e),
/// }
/// ```
pub fn set_loader(id: i32, loader: i8, loader_version: &str) -> Result<(), String>
{
	info!("Updating loader for server with id: {}", id);
	let conn = match create_connection() {
		Ok(conn) => {
			debug!("Successfully connected to the database.");
			conn
		},
		Err(e) => {
			error!("Failed to connect to the database: {}", e);
			return Err(format!("Failed to connect to database: {}", e));
		}
	};

	let mut statement = match conn.prepare("UPDATE servers SET loader = ?, loader_version = ? WHERE id = ?") {
		Ok(stmt) => {
			debug!("Successfully prepared the UPDATE statement.");
			stmt
		},
		Err(e) => {
			error!("Failed to prepare the UPDATE statement: {}", e);
			return Err(format!("Failed to prepare statement: {}", e));
		}
	};

	if let Err(e) = statement.bind((1, loader as i64)) {
		error!("Failed to bind loader: {}", e);
		return Err(format!("Failed to bind loader: {}", e));
	}

	if let Err(e) = statement.bind((2, loader_version)) {
		error!("Failed to bind loader_version: {}", e);
		return Err(format!("Failed to bind loader_version: {}", e));
	}

	if let Err(e) = statement.bind((3, id as i64)) {
		error!("Failed to bind id: {}", e);
		return Err(format!("Failed to bind id: {}", e));
	}

	if let Err(e) = statement.next() {
		error!("Failed to execute statement: {}", e);
		return Err(format!("Failed to execute statement: {}", e));
	}

	info!("Successfully updated loader for server with id: {}", id);
	Ok(())
}

/// Deletes the server with the given ID from the database.
///
/// # Arguments
///
/// * `id` - The ID of the server to be deleted.
///
/// # Returns
///
/// * `Ok(())` - If the server was successfully deleted.
/// * `Err(String)` - If there was an error deleting the server. The error message is returned as a `String`.
///
/// # Example
///
/// ```
/// match delete_server(1) {
///     Ok(()) => println!("Server deleted successfully."),
///     Err(e) => println!("Error deleting server: {}", e),
/// }
/// ```
pub fn delete_server(id: i32) -> Result<(), String> {
	info!("Deleting server with id: {}", id);
	let conn = match create_connection() {
		Ok(conn) => {
			debug!("Successfully connected to the database.");
			conn
		},
		Err(e) => {
			error!("Failed to connect to the database: {}", e);
			return Err(format!("Failed to connect to database: {}", e));
		}
	};

	let mut statement = match conn.prepare("DELETE FROM servers WHERE id = ?") {
		Ok(stmt) => {
			debug!("Successfully prepared the DELETE statement.");
			stmt
		},
		Err(e) => {
			error!("Failed to prepare the DELETE statement: {}", e);
			return Err(format!("Failed to prepare statement: {}", e));
		}
	};

	if let Err(e) = statement.bind((1, id as i64)) {
		error!("Failed to bind id: {}", e);
		return Err(format!("Failed to bind id: {}", e));
	}

	if let Err(e) = statement.next() {
		error!("Failed to execute statement: {}", e);
		return Err(format!("Failed to execute statement: {}", e));
	}

	info!("Successfully deleted server with id: {}", id);
	Ok(())
}


fn create_connection() -> Result<sqlite::Connection, sqlite::Error> {
	debug!("Attempting to open the database connection for servers.");
	match sqlite::Connection::open("servers.db") {
		Ok(conn) => {
			debug!("Servers database connection opened successfully.");
			Ok(conn)
		},
		Err(e) => {
			error!("Failed to open servers database connection: {}", e);
			Err(e)
		}
	}
}
