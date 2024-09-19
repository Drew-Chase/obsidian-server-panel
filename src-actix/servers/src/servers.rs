use log::info;
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

pub fn initialize() {
	info!("Initializing servers database");
	let conn = create_connection().expect("Failed to connect to database");
	conn.execute(
		"
					CREATE TABLE IF NOT EXISTS servers
					(
					    id       INTEGER PRIMARY KEY AUTOINCREMENT,
					    name     TEXT    NOT NULL,
					    instance INTEGER DEFAULT NULL,
					    owner    INTEGER NOT NULL,
					    size     INTEGER DEFAULT 0,
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
	)
	    .expect("Failed to create servers table");
}

pub fn create_server(name: &str, owner: i64) -> Result<Server, String>
{
	let conn = match create_connection() {
		Ok(conn) => conn,
		Err(e) => return Err(format!("Failed to connect to database: {}", e))
	};

	let mut statement = match conn.prepare("INSERT INTO servers (name, owner) VALUES (?, ?, ?)") {
		Ok(stmt) => stmt,
		Err(e) => return Err(format!("Failed to prepare statement: {}", e))
	};

	statement.bind((1, name)).map_err(|e| format!("Failed to bind name: {}", e))?;
	statement.bind((2, owner)).map_err(|e| format!("Failed to bind owner: {}", e))?;
	statement.next().map_err(|e| format!("Failed to execute statement: {}", e))?;

	// get the inserted id for sqlite
	let mut statement = conn.prepare("select seq from sqlite_sequence WHERE name = 'servers'").map_err(|e| format!("Failed to get inserted id: {}", e))?;
	statement.next().map_err(|e| format!("Failed to get inserted id: {}", e))?;
	let id = statement.read::<i64, _>("seq").map_err(|e| format!("Failed to get inserted id: {}", e))?;

	match get_server_by_id(id) {
		Some(server) => Ok(server),
		None => Err("Something went wrong, server id not present in server list. This usually means that the server failed to be inserted into the database.".to_string())
	}
}

pub fn get_server_by_id(id: i64) -> Option<Server>
{
	let conn = match create_connection() {
		Ok(conn) => conn,
		Err(_) => return None
	};

	let mut statement = match conn.prepare("SELECT * FROM servers WHERE id = ?") {
		Ok(stmt) => stmt,
		Err(_) => return None
	};

	statement.bind((1, id)).ok()?;
	statement.next().ok()?;

	Some(Server {
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
	})
}

pub fn get_servers() -> Result<Vec<Server>, String>
{
	let conn = match create_connection() {
		Ok(conn) => conn,
		Err(e) => return Err(format!("Failed to connect to database: {}", e))
	};

	let mut statement = match conn.prepare("SELECT * FROM servers") {
		Ok(stmt) => stmt,
		Err(e) => return Err(format!("Failed to prepare statement: {}", e))
	};

	let mut servers: Vec<Server> = vec![];
	let mut state = statement.next().unwrap();
	while state != State::Done {
		servers.push(Server {
			id: statement.read::<i64, _>("id").unwrap() as i32,
			name: statement.read::<String, _>("name").unwrap(),
			instance: Some(statement.read::<i64, _>("instance").unwrap() as i32),
			owner: statement.read::<i64, _>("owner").unwrap() as i32,
			size: statement.read::<i64, _>("size").unwrap(),
			auto_start: statement.read::<i64, _>("auto_start").unwrap() == 1,
			min_ram: statement.read::<i64, _>("min_ram").unwrap(),
			max_ram: statement.read::<i64, _>("max_ram").unwrap(),
			executable: statement.read::<Option<String>, _>("executable").unwrap(),
			minecraft_arguments: statement.read::<Option<String>, _>("minecraft_arguments").unwrap(),
			java_arguments: statement.read::<Option<String>, _>("java_arguments").unwrap(),
			minecraft_version: statement.read::<Option<String>, _>("minecraft_version").unwrap(),
			loader: statement.read::<i64, _>("loader").unwrap() as i8,
			loader_version: statement.read::<Option<String>, _>("loader_version").unwrap(),
			created_at: statement.read::<String, _>("created_at").unwrap(),
			updated_at: statement.read::<String, _>("updated_at").unwrap(),
		});
		state = statement.next().unwrap();
	}

	Ok(servers)
}

pub fn set_ram_allocation(id: i32, min_ram: i64, max_ram: i64) -> Result<(), String>
{
	let conn = match create_connection() {
		Ok(conn) => conn,
		Err(e) => return Err(format!("Failed to connect to database: {}", e))
	};

	let mut statement = match conn.prepare("UPDATE servers SET min_ram = ?, max_ram = ? WHERE id = ?") {
		Ok(stmt) => stmt,
		Err(e) => return Err(format!("Failed to prepare statement: {}", e))
	};

	statement.bind((1, min_ram)).map_err(|e| format!("Failed to bind min_ram: {}", e))?;
	statement.bind((2, max_ram)).map_err(|e| format!("Failed to bind max_ram: {}", e))?;
	statement.bind((3, id as i64)).map_err(|e| format!("Failed to bind id: {}", e))?;
	statement.next().map_err(|e| format!("Failed to execute statement: {}", e))?;

	Ok(())
}

pub fn set_autostart(id: i32, auto_start: bool) -> Result<(), String>
{
	let conn = match create_connection() {
		Ok(conn) => conn,
		Err(e) => return Err(format!("Failed to connect to database: {}", e))
	};

	let mut statement = match conn.prepare("UPDATE servers SET auto_start = ? WHERE id = ?") {
		Ok(stmt) => stmt,
		Err(e) => return Err(format!("Failed to prepare statement: {}", e))
	};
	statement.bind((1, auto_start as i64)).map_err(|e| format!("Failed to bind auto_start: {}", e))?;
	statement.bind((2, id as i64)).map_err(|e| format!("Failed to bind id: {}", e))?;
	statement.next().map_err(|e| format!("Failed to execute statement: {}", e))?;

	Ok(())
}

pub fn set_executable(id: i32, executable: &str) -> Result<(), String>
{
	let conn = match create_connection() {
		Ok(conn) => conn,
		Err(e) => return Err(format!("Failed to connect to database: {}", e))
	};

	let mut statement = match conn.prepare("UPDATE servers SET executable = ? WHERE id = ?") {
		Ok(stmt) => stmt,
		Err(e) => return Err(format!("Failed to prepare statement: {}", e))
	};
	statement.bind((1, executable)).map_err(|e| format!("Failed to bind executable: {}", e))?;
	statement.bind((2, id as i64)).map_err(|e| format!("Failed to bind id: {}", e))?;
	statement.next().map_err(|e| format!("Failed to execute statement: {}", e))?;

	Ok(())
}

pub fn set_minecraft_arguments(id: i32, minecraft_arguments: &str) -> Result<(), String>
{
	let conn = match create_connection() {
		Ok(conn) => conn,
		Err(e) => return Err(format!("Failed to connect to database: {}", e))
	};

	let mut statement = match conn.prepare("UPDATE servers SET minecraft_arguments = ? WHERE id = ?") {
		Ok(stmt) => stmt,
		Err(e) => return Err(format!("Failed to prepare statement: {}", e))
	};
	statement.bind((1, minecraft_arguments)).map_err(|e| format!("Failed to bind minecraft_arguments: {}", e))?;
	statement.bind((2, id as i64)).map_err(|e| format!("Failed to bind id: {}", e))?;
	statement.next().map_err(|e| format!("Failed to execute statement: {}", e))?;

	Ok(())
}

pub fn set_java_arguments(id: i32, java_arguments: &str) -> Result<(), String>
{
	let conn = match create_connection() {
		Ok(conn) => conn,
		Err(e) => return Err(format!("Failed to connect to database: {}", e))
	};

	let mut statement = match conn.prepare("UPDATE servers SET java_arguments = ? WHERE id = ?") {
		Ok(stmt) => stmt,
		Err(e) => return Err(format!("Failed to prepare statement: {}", e))
	};
	statement.bind((1, java_arguments)).map_err(|e| format!("Failed to bind java_arguments: {}", e))?;
	statement.bind((2, id as i64)).map_err(|e| format!("Failed to bind id: {}", e))?;
	statement.next().map_err(|e| format!("Failed to execute statement: {}", e))?;

	Ok(())
}

pub fn set_minecraft_version(id: i32, minecraft_version: &str) -> Result<(), String>
{
	let conn = match create_connection() {
		Ok(conn) => conn,
		Err(e) => return Err(format!("Failed to connect to database: {}", e))
	};

	let mut statement = match conn.prepare("UPDATE servers SET minecraft_version = ? WHERE id = ?") {
		Ok(stmt) => stmt,
		Err(e) => return Err(format!("Failed to prepare statement: {}", e))
	};
	statement.bind((1, minecraft_version)).map_err(|e| format!("Failed to bind minecraft_version: {}", e))?;
	statement.bind((2, id as i64)).map_err(|e| format!("Failed to bind id: {}", e))?;
	statement.next().map_err(|e| format!("Failed to execute statement: {}", e))?;

	Ok(())
}

pub fn set_loader(id: i32, loader: i8, loader_version: &str) -> Result<(), String>
{
	let conn = match create_connection() {
		Ok(conn) => conn,
		Err(e) => return Err(format!("Failed to connect to database: {}", e))
	};

	let mut statement = match conn.prepare("UPDATE servers SET loader = ?, loader_version = ? WHERE id = ?") {
		Ok(stmt) => stmt,
		Err(e) => return Err(format!("Failed to prepare statement: {}", e))
	};
	statement.bind((1, loader as i64)).map_err(|e| format!("Failed to bind loader: {}", e))?;
	statement.bind((2, loader_version)).map_err(|e| format!("Failed to bind loader_version: {}", e))?;
	statement.bind((3, id as i64)).map_err(|e| format!("Failed to bind id: {}", e))?;
	statement.next().map_err(|e| format!("Failed to execute statement: {}", e))?;

	Ok(())
}

pub fn delete_server(id: i32) -> Result<(), String>
{
	let conn = match create_connection() {
		Ok(conn) => conn,
		Err(e) => return Err(format!("Failed to connect to database: {}", e))
	};

	let mut statement = match conn.prepare("DELETE FROM servers WHERE id = ?") {
		Ok(stmt) => stmt,
		Err(e) => return Err(format!("Failed to prepare statement: {}", e))
	};
	statement.bind((1, id as i64)).map_err(|e| format!("Failed to bind id: {}", e))?;
	statement.next().map_err(|e| format!("Failed to execute statement: {}", e))?;

	Ok(())
}



fn create_connection() -> Result<sqlite::Connection, sqlite::Error> {
	sqlite::Connection::open("servers.db")
}
