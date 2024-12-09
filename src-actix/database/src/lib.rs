#![deny(clippy::unwrap_used)]
#![deny(clippy::expect_used)]
#![deny(clippy::panic)]
#![deny(unused_must_use)]

use log::error;
use std::error::Error;

/// Establishes a connection to the app's SQLite database.
///
/// # Returns
///
/// * `Ok(Connection)` - A successful connection to the SQLite database.
/// * `Err(Error)` - An error that occurred while attempting to open the database or set the journal mode.
///
/// # Errors
///
/// This function will return an error if:
///
/// * The database file `app.db` could not be opened.
/// * The `PRAGMA journal_mode = WAL` statement could not be executed.
///
/// The error is logged with the `log` crate's `error!` macro.
pub fn create_appdb_connection() -> Result<sqlite::Connection, sqlite::Error> {
    match sqlite::Connection::open("app.db").map_err(|e| {
        error!("Failed to open apps database connection: {}", e);
        e
    }) {
        Ok(conn) => {
            // allows multiple connections to the database
            match conn.execute("PRAGMA journal_mode = WAL;") {
                Ok(_) => {}
                Err(e) => {
                    error!("Failed to set journal mode to WAL: {}", e);
                }
            }
            Ok(conn)
        }
        Err(e) => Err(e),
    }
}
pub fn create_cachedb_connection() -> Result<sqlite::Connection, sqlite::Error> {
    match sqlite::Connection::open("cache.db").map_err(|e| {
        error!("Failed to open cache database connection: {}", e);
        e
    }) {
        Ok(conn) => {
            // allows multiple connections to the database
            match conn.execute("PRAGMA journal_mode = WAL;") {
                Ok(_) => {}
                Err(e) => {
                    error!("Failed to set journal mode to WAL: {}", e);
                }
            }
            Ok(conn)
        }
        Err(e) => Err(e),
    }
}

pub fn last_inserted_id(table: impl AsRef<str>) -> Result<u64, Box<dyn Error>> {
    let conn = create_appdb_connection()?;
    let id: i64 = conn
        .prepare(format!("SELECT seq FROM sqlite_sequence WHERE name = '{}'", table.as_ref()))
        .and_then(|mut s| s.next().map(|_| s.read("seq")))
        .map_err(|e| format!("Failed to get inserted id: {}", e))?
        .map_err(|e| format!("Failed to read inserted id: {}", e))?;

    Ok(id as u64)
}
