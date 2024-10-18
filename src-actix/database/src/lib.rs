use log::error;

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
        error!(
            "Failed to open apps database connection: {}",
            e
        );
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
        error!(
            "Failed to open cache database connection: {}",
            e
        );
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
