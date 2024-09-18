use serde_derive::{Deserialize, Serialize};
use log::info;
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

fn create_connection() -> Result<sqlite::Connection, sqlite::Error> {
	sqlite::Connection::open("servers.db")
}
