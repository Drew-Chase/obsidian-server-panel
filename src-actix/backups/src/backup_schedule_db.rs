use crate::backup_item::BackupType;
use crate::backup_schedules::BackupSchedule;
use database::create_appdb_connection;
use log::{debug, error, info};
use sqlite::State;
use std::error::Error;

pub fn initialize() {
    debug!("Initializing backup schedule table");
    let conn = create_appdb_connection().expect("Failed to connect to database");
    if let Err(e) = conn.execute(
        "
					CREATE TABLE IF NOT EXISTS scheduled_backups
					(
						id	 INTEGER NOT NULL PRIMARY KEY AUTOINCREMENT,
						server INTEGER NOT NULL,
						type   INTEGER NOT NULL,
						interval   INTEGER	NOT NULL,
						exec_if_empty BOOLEAN NOT NULL,
						exec_if_offline BOOLEAN NOT NULL
					);
	",
    ) {
        error!("Failed to create backups table: {}", e);
    } else {
        info!("Successfully created or verified the backup schedule table.");
    }
}

pub fn insert(
    server: u32,
    backup_type: BackupType,
    interval: i64,
    exec_if_empty: bool,
    exec_if_offline: bool,
) -> Result<(), Box<dyn Error>> {
    let conn = create_appdb_connection()?;
    let mut stmt = conn.prepare("INSERT INTO scheduled_backups (server, type, interval, exec_if_empty, exec_if_offline) VALUES (?, ?, ?, ?, ?)")?;
    let backup_type = match backup_type {
        BackupType::Full => 0,
        BackupType::Incremental => 1,
    };
    stmt.bind((1, server as i64))?;
    stmt.bind((2, backup_type as i64))?;
    stmt.bind((3, interval))?;
    stmt.bind((4, exec_if_empty as i64))?;
    stmt.bind((5, exec_if_offline as i64))?;
    stmt.next()?;

    Ok(())
}

pub fn get(id: u32) -> Result<Option<BackupSchedule>, Box<dyn Error>> {
    let conn = create_appdb_connection()?;
    let mut stmt = conn.prepare("SELECT * FROM scheduled_backups WHERE id = ?")?;
    stmt.bind((1, id as i64))?;
    if State::Row == stmt.next()? {
        let server: u32 = stmt.read::<i64, _>("server")? as u32;
        let backup_type: u8 = stmt.read::<i64, _>("type")? as u8;
        let interval: i64 = stmt.read("interval")?;
        let exec_if_empty: bool = stmt.read::<i64, _>("exec_if_empty")? != 0;
        let exec_if_offline: bool = stmt.read::<i64, _>("exec_if_offline")? != 0;
        let backup_type = match backup_type {
            0 => BackupType::Full,
            1 => BackupType::Incremental,
            _ => return Err("Invalid backup type".into()),
        };
        Ok(Some(BackupSchedule::new(
            id,
            server,
            backup_type,
            interval as u32,
            exec_if_empty,
            exec_if_offline,
        )))
    } else {
        Ok(None)
    }
}

pub fn list() -> Result<Vec<BackupSchedule>, Box<dyn Error>> {
    let mut schedules = Vec::new();

    let conn = create_appdb_connection()?;
    let mut stmt = conn.prepare("SELECT * FROM scheduled_backups")?;
    while State::Row == stmt.next()? {
        let id: u32 = stmt.read::<i64, _>("id")? as u32;
        let server: u32 = stmt.read::<i64, _>("server")? as u32;
        let backup_type: u8 = stmt.read::<i64, _>("type")? as u8;
        let interval: i64 = stmt.read("interval")?;
        let exec_if_empty: bool = stmt.read::<i64, _>("exec_if_empty")? != 0;
        let exec_if_offline: bool = stmt.read::<i64, _>("exec_if_offline")? != 0;
        let backup_type = match backup_type {
            0 => BackupType::Full,
            1 => BackupType::Incremental,
            _ => return Err("Invalid backup type".into()),
        };
        schedules.push(BackupSchedule::new(
            id,
            server,
            backup_type,
            interval as u32,
            exec_if_empty,
            exec_if_offline,
        ));
    }

    Ok(schedules)
}

pub fn update(
    id: u32,
    server: u32,
    backup_type: u8,
    interval: i64,
    exec_if_empty: bool,
    exec_if_offline: bool,
) -> Result<(), Box<dyn Error>> {
    let conn = create_appdb_connection()?;
    let mut stmt = conn.prepare("UPDATE scheduled_backups SET server = ?, type = ?, interval = ?, exec_if_empty = ?, exec_if_offline = ? WHERE id = ?")?;
    stmt.bind((1, server as i64))?;
    stmt.bind((2, backup_type as i64))?;
    stmt.bind((3, interval))?;
    stmt.bind((4, exec_if_empty as i64))?;
    stmt.bind((5, exec_if_offline as i64))?;
    stmt.bind((6, id as i64))?;
    stmt.next()?;

    Ok(())
}

pub fn delete(id: u32) -> Result<(), Box<dyn Error>> {
    let conn = create_appdb_connection()?;
    let mut stmt = conn.prepare("DELETE FROM scheduled_backups WHERE id = ?")?;
    stmt.bind((1, id as i64))?;
    stmt.next()?;

    Ok(())
}
