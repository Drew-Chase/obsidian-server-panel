mod backup_db;
pub mod backup_item;
mod backup_schedule_db;
mod backup_schedules;
mod file_hash_db;
pub mod hashed_backup_item;
pub mod hashed_file;
mod lazy_hashed_file;

use chrono::{DateTime, NaiveDateTime, Utc};
use log::info;
use std::path::{Path, PathBuf};
use std::time::SystemTime;

/// Initializes the backups database and the file hash database.
pub fn initialize() {
    info!("Initializing backups database");
    backup_db::initialize();
    file_hash_db::initialize();
}

/// Returns the path to the backups directory.
///
/// # Returns
///
/// A `PathBuf` representing the backups directory path.
pub fn get_backups_directory() -> PathBuf {
    Path::new("backups").to_path_buf()
}

/// Converts a string representation of a date and time to a `SystemTime`.
///
/// The string should be in the format "%Y-%m-%d %H:%M:%S".
///
/// # Arguments
///
/// * `time` - A string that holds the date and time.
///
/// # Returns
///
/// * `Some(SystemTime)` if the conversion is successful.
/// * `None` if the conversion fails.
///
/// # Example
///
/// ```
/// use std::time::SystemTime;
/// use backups::system_time_from_string;
///
/// let time = "2021-01-01 12:00:00";
/// let expected = SystemTime::UNIX_EPOCH + std::time::Duration::new(1609488000, 0);
/// assert_eq!(system_time_from_string(time), Some(expected));
/// ```
pub fn system_time_from_string(time: impl AsRef<str>) -> Option<SystemTime> {
    NaiveDateTime::parse_from_str(time.as_ref(), "%Y-%m-%d %H:%M:%S")
        .ok()
        .map(|naive_dt| {
            let dt = DateTime::<Utc>::from_naive_utc_and_offset(naive_dt, Utc);
            SystemTime::from(dt)
        })
}

/// Converts a `SystemTime` to a string representation.
///
/// The string will be in the format "%Y-%m-%d %H:%M:%S".
///
/// # Arguments
///
/// * `time` - A `SystemTime` that holds the date and time.
///
/// # Returns
///
/// A `String` that represents the date and time.
///
/// # Example
///
/// ```
/// use std::time::SystemTime;
/// use backups::system_time_to_string;
///
/// let time = SystemTime::UNIX_EPOCH + std::time::Duration::new(1609488000, 0);
/// let formatted = system_time_to_string(time);
/// assert_eq!(formatted, "2021-01-01 12:00:00");
/// ```
pub fn system_time_to_string(time: SystemTime) -> String {
    let dt = DateTime::<Utc>::from(time);
    dt.format("%Y-%m-%d %H:%M:%S").to_string()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_system_time_from_string() {
        let time = "2021-01-01 12:00:00";
        let expected = SystemTime::UNIX_EPOCH + std::time::Duration::new(1609488000, 0);
        assert_eq!(system_time_from_string(time), Some(expected));
    }
}
