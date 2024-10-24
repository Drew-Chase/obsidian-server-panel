use crate::server_process::ServerStatus;
use crypto::hashids::encode;
use database::create_appdb_connection;
use log::{error, info};
use serde_derive::{Deserialize, Serialize};
use sqlite::{State, Statement};
use std::path::Path;
use std::str::FromStr;

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct Server {
    pub id: u32,
    pub name: String,
    pub instance: Option<u32>,
    pub owner: u32,
    pub members: Vec<u32>,
    pub size: u64,
    pub auto_start: bool,
    pub min_ram: u64,
    pub max_ram: u64,
    pub executable: Option<String>,
    pub minecraft_arguments: Option<String>,
    pub java_arguments: Option<String>,
    pub minecraft_version: Option<String>,
    pub loader: u8,
    pub loader_version: Option<String>,
    pub directory: Option<String>,
    pub created_at: String,
    pub updated_at: String,
    pub status: Option<ServerStatus>,
    #[serde(skip)]
    pub pid: Option<u32>,
}

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct HashedServer {
    pub id: String,
    pub name: String,
    pub instance: Option<u32>,
    pub owner: String,
    pub members: Vec<String>,
    pub size: u64,
    pub auto_start: bool,
    pub min_ram: u64,
    pub max_ram: u64,
    pub executable: Option<String>,
    pub minecraft_arguments: Option<String>,
    pub java_arguments: Option<String>,
    pub minecraft_version: Option<String>,
    pub loader: u8,
    pub loader_version: Option<String>,
    pub directory: Option<String>,
    pub status: Option<ServerStatus>,
    pub pid: Option<u32>,
    pub created_at: String,
    pub updated_at: String,
}

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct BasicHashedServer {
    pub id: String,
    pub name: String,
    pub owner: String,
    pub members: Vec<String>,
    pub status: Option<ServerStatus>,
    pub created_at: String,
    pub updated_at: String,
}

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct ServerSettings {
    pub name: String,
    pub min_ram: u64,
    pub max_ram: u64,
    pub minecraft_arguments: Option<String>,
    pub java_arguments: Option<String>,
    pub minecraft_version: Option<String>,
    pub loader: u8,
    pub loader_version: Option<String>,
    pub executable: Option<String>,
}

impl HashedServer {
    pub fn from_server(server: Server) -> Self {
        HashedServer {
            id: encode(&[server.id as u64]),
            name: server.name,
            instance: server.instance,
            owner: encode(&[server.owner as u64]),
            members: server
                .members
                .iter()
                .map(|m| encode(&[*m as u64]))
                .collect(),
            size: server.size,
            auto_start: server.auto_start,
            min_ram: server.min_ram,
            max_ram: server.max_ram,
            executable: server.executable,
            minecraft_arguments: server.minecraft_arguments,
            java_arguments: server.java_arguments,
            minecraft_version: server.minecraft_version,
            loader: server.loader,
            loader_version: server.loader_version,
            directory: server.directory,
            status: server.status,
            pid: server.pid,
            created_at: server.created_at,
            updated_at: server.updated_at,
        }
    }
}

impl BasicHashedServer {
    pub fn from_server(server: Server) -> Self {
        BasicHashedServer {
            id: encode(&[server.id as u64]),
            name: server.name,
            owner: encode(&[server.owner as u64]),
            members: server
                .members
                .iter()
                .map(|m| encode(&[*m as u64]))
                .collect(),
            status: server.status,
            created_at: server.created_at,
            updated_at: server.updated_at,
        }
    }
    pub fn from_hashed_server(server: HashedServer) -> Self {
        BasicHashedServer {
            id: server.id,
            name: server.name,
            owner: server.owner,
            members: server.members,
            status: server.status,
            created_at: server.created_at,
            updated_at: server.updated_at,
        }
    }
}

impl ServerSettings {
    pub fn from_server(server: Server) -> Self {
        ServerSettings {
            name: server.name,
            min_ram: server.min_ram,
            max_ram: server.max_ram,
            minecraft_arguments: server.minecraft_arguments,
            java_arguments: server.java_arguments,
            minecraft_version: server.minecraft_version,
            loader: server.loader,
            loader_version: server.loader_version,
            executable: server.executable,
        }
    }
}

pub fn initialize() {
    info!("Initializing servers database");
    let conn = create_appdb_connection().expect("Failed to connect to database");
    if let Err(e) = conn.execute(
        "
		CREATE TABLE IF NOT EXISTS servers (
			id INTEGER PRIMARY KEY AUTOINCREMENT,
			name TEXT NOT NULL,
			instance INTEGER DEFAULT NULL,
			owner INTEGER NOT NULL,
            members TEXT DEFAULT NULL,
			size INTEGER DEFAULT 0,
			auto_start INTEGER DEFAULT 0,
			min_ram INTEGER DEFAULT 0,
			max_ram INTEGER DEFAULT 0,
			executable TEXT DEFAULT NULL,
			minecraft_arguments TEXT DEFAULT NULL,
			java_arguments TEXT DEFAULT NULL,
			minecraft_version TEXT DEFAULT NULL,
			loader INTEGER DEFAULT 0,
			loader_version TEXT DEFAULT NULL,
			directory TEXT DEFAULT NULL,
            status TEXT DEFAULT NULL,
            pid INTEGER DEFAULT NULL,
			created_at TIMESTAMP DEFAULT CURRENT_TIMESTAMP,
			updated_at TIMESTAMP DEFAULT CURRENT_TIMESTAMP
		)
	",
    ) {
        error!("Failed to create servers table: {}", e);
    } else {
        info!("Successfully created or verified the servers table.");
    }

    if !Path::new("servers").exists() {
        std::fs::create_dir("servers").expect("Failed to create servers directory");
        info!("Created servers directory");
    }
}

pub fn create_server(name: &str, owner: u32) -> Result<Server, String> {
    info!("Creating server with name: {} and owner: {}", name, owner);
    let conn =
        create_appdb_connection().map_err(|e| format!("Failed to connect to database: {}", e))?;

    let mut statement = conn
        .prepare("INSERT INTO servers (name, owner) VALUES (?, ?)")
        .map_err(|e| format!("Failed to prepare statement: {}", e))?;

    statement
        .bind((1, name))
        .map_err(|e| format!("Failed to bind name: {}", e))?;
    statement
        .bind((2, owner as i64))
        .map_err(|e| format!("Failed to bind owner: {}", e))?;
    statement
        .next()
        .map_err(|e| format!("Failed to execute statement: {}", e))?;

    let id: i64 = conn
        .prepare("SELECT seq FROM sqlite_sequence WHERE name = 'servers'")
        .and_then(|mut s| s.next().map(|_| s.read("seq")))
        .map_err(|e| format!("Failed to get inserted id: {}", e))?
        .map_err(|e| format!("Failed to read inserted id: {}", e))?; // Get the last inserted id

    get_server_by_id(id as u32).ok_or_else(|| "Failed to retrieve the new server".to_string())
}

pub fn get_server_by_id(id: u32) -> Option<Server> {
    let conn = create_appdb_connection().ok()?;
    let mut statement = conn
        .prepare("SELECT * FROM servers WHERE id = ? LIMIT 1")
        .ok()?;
    statement.bind((1, id as i64)).ok()?;
    statement.next().ok()?;
    get_server_from_statement(&statement).ok()
}

pub fn get_servers_by_owner(owner: u32) -> Result<Vec<Server>, String> {
    let conn =
        create_appdb_connection().map_err(|e| format!("Failed to connect to database: {}", e))?;
    let mut statement = conn
        .prepare("SELECT * FROM servers WHERE owner = ?")
        .map_err(|e| format!("Failed to prepare statement: {}", e))?;
    statement
        .bind((1, owner as i64))
        .map_err(|e| format!("Failed to bind owner: {}", e))?;
    let mut servers = Vec::new();
    while let State::Row = statement
        .next()
        .map_err(|e| format!("Failed to execute statement: {}", e))?
    {
        servers.push(
            get_server_from_statement(&statement)
                .map_err(|e| format!("Failed to get server from statement: {}", e))?,
        );
    }
    Ok(servers)
}

pub fn get_servers() -> Vec<Server> {
    let conn = create_appdb_connection().unwrap();
    let mut statement = conn.prepare("SELECT * FROM servers").unwrap();
    let mut servers = Vec::new();
    while let State::Row = statement.next().unwrap() {
        servers.push(get_server_from_statement(&statement).unwrap());
    }
    servers
}

pub fn get_owned_server_by_id(id: u32, owner: u32) -> Option<Server> {
    let conn = create_appdb_connection().ok()?;
    let mut statement = conn
        .prepare("SELECT * FROM servers WHERE id = ? AND owner = ? LIMIT 1")
        .ok()?;
    statement.bind((1, id as i64)).ok()?;
    statement.bind((2, owner as i64)).ok()?;
    statement.next().ok()?;
    get_server_from_statement(&statement).ok()
}

pub fn set_java_arguments(id: u32, java_arguments: &str) -> Result<(), String> {
    update_server_attribute("java_arguments", java_arguments, id)
}
pub fn set_minecraft_arguments(id: u32, minecraft_arguments: &str) -> Result<(), String> {
    update_server_attribute("minecraft_arguments", minecraft_arguments, id)
}

pub fn set_memory_min(id: u32, min_ram: u64) -> Result<(), String> {
    update_server_attribute("min_ram", &min_ram.to_string(), id)
}

pub fn set_memory_max(id: u32, max_ram: u64) -> Result<(), String> {
    update_server_attribute("max_ram", &max_ram.to_string(), id)
}
pub fn set_server_executable(id: u32, executable: &str) -> Result<(), String> {
    update_server_attribute("executable", executable, id)
}

pub fn set_minecraft_version(id: u32, minecraft_version: &str) -> Result<(), String> {
    update_server_attribute("minecraft_version", minecraft_version, id)
}

pub fn set_server_directory(id: u32, dir: &str) -> Result<(), String> {
    update_server_attribute("directory", dir, id)
}

pub fn set_server_name(id: u32, name: &str) -> Result<(), String> {
    update_server_attribute("name", name, id)
}

pub fn set_loader(id: u32, loader: u8, loader_version: &str) -> Result<(), String> {
    let conn =
        create_appdb_connection().map_err(|e| format!("Failed to connect to database: {}", e))?;
    let mut statement = conn
        .prepare("UPDATE servers SET loader = ?, loader_version = ? WHERE id = ?")
        .map_err(|e| format!("Failed to prepare statement: {}", e))?;
    statement
        .bind((1, loader as i64))
        .and_then(|_| statement.bind((2, loader_version)))
        .and_then(|_| statement.bind((3, id as i64)))
        .and_then(|_| statement.next())
        .map_err(|e| format!("Failed to execute statement: {}", e))?;
    Ok(())
}

pub fn delete_server(id: u32) -> Result<(), String> {
    let conn =
        create_appdb_connection().map_err(|e| format!("Failed to connect to database: {}", e))?;
    let mut statement = conn
        .prepare("DELETE FROM servers WHERE id = ?")
        .map_err(|e| format!("Failed to prepare statement: {}", e))?;
    statement
        .bind((1, id as i64))
        .and_then(|_| statement.next())
        .map_err(|e| format!("Failed to execute statement: {}", e))?;
    Ok(())
}

fn get_server_from_statement(statement: &Statement) -> Result<Server, String> {
    Ok(Server {
        id: statement
            .read::<i64, _>("id")
            .map_err(|e| format!("Failed to read 'id': {}", e))? as u32,
        name: statement
            .read::<String, _>("name")
            .map_err(|e| format!("Failed to read 'name': {}", e))?,
        instance: statement
            .read::<Option<i64>, _>("instance")
            .map_err(|e| format!("Failed to read 'instance': {}", e))?
            .map(|v| v as u32),
        owner: statement
            .read::<i64, _>("owner")
            .map_err(|e| format!("Failed to read 'owner': {}", e))? as u32,
        members: statement
            .read::<Option<String>, _>("members")
            .map_err(|e| format!("Failed to read 'members': {}", e))?
            .unwrap_or_else(|| "".to_string())
            .split(',')
            .filter_map(|s| s.parse::<u32>().ok())
            .collect(),
        size: statement
            .read::<i64, _>("size")
            .map_err(|e| format!("Failed to read 'size': {}", e))? as u64,
        auto_start: statement
            .read::<i64, _>("auto_start")
            .map_err(|e| format!("Failed to read 'auto_start': {}", e))?
            == 1,
        min_ram: statement
            .read::<i64, _>("min_ram")
            .map_err(|e| format!("Failed to read 'min_ram': {}", e))? as u64,
        max_ram: statement
            .read::<i64, _>("max_ram")
            .map_err(|e| format!("Failed to read 'max_ram': {}", e))? as u64,
        executable: statement
            .read::<Option<String>, _>("executable")
            .map_err(|e| format!("Failed to read 'executable': {}", e))?,
        minecraft_arguments: statement
            .read::<Option<String>, _>("minecraft_arguments")
            .map_err(|e| format!("Failed to read 'minecraft_arguments': {}", e))?,
        java_arguments: statement
            .read::<Option<String>, _>("java_arguments")
            .map_err(|e| format!("Failed to read 'java_arguments': {}", e))?,
        minecraft_version: statement
            .read::<Option<String>, _>("minecraft_version")
            .map_err(|e| format!("Failed to read 'minecraft_version': {}", e))?,
        loader: statement
            .read::<i64, _>("loader")
            .map_err(|e| format!("Failed to read 'loader': {}", e))? as u8,
        loader_version: statement
            .read::<Option<String>, _>("loader_version")
            .map_err(|e| format!("Failed to read 'loader_version': {}", e))?,
        directory: statement
            .read::<Option<String>, _>("directory")
            .map_err(|e| format!("Failed to read 'directory': {}", e))?,
        created_at: statement
            .read::<String, _>("created_at")
            .map_err(|e| format!("Failed to read 'created_at': {}", e))?,
        updated_at: statement
            .read::<String, _>("updated_at")
            .map_err(|e| format!("Failed to read 'updated_at': {}", e))?,
        status: statement
            .read::<Option<String>, _>("status")
            .map_err(|e| format!("Failed to read 'status': {}", e))?
            .map(|s| ServerStatus::from_str(&s).unwrap_or(ServerStatus::Stopped)),
        pid: statement
            .read::<Option<i64>, _>("pid")
            .map_err(|e| format!("Failed to read 'pid': {}", e))
            .ok()
            .map(|v| if let Some(v) = v { v as u32 } else { 0 }),
    })
}

fn update_server_attribute(attr: &str, value: &str, id: u32) -> Result<(), String> {
    let conn =
        create_appdb_connection().map_err(|e| format!("Failed to connect to database: {}", e))?;
    let query = format!("UPDATE servers SET {} = ? WHERE id = ?", attr);
    let mut statement = conn
        .prepare(&query)
        .map_err(|e| format!("Failed to prepare statement: {}", e))?;
    statement
        .bind((1, value))
        .and_then(|_| statement.bind((2, id as i64)))
        .and_then(|_| statement.next())
        .map_err(|e| format!("Failed to execute statement: {}", e))?;
    Ok(())
}
