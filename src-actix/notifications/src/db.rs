use crate::data::{AsNumber, Notification, SenderType};
use crypto::hashids::{decode, encode};
use sqlite::State;
use std::error::Error;

pub fn initialize_db() -> Result<(), Box<dyn Error>> {
    let connection = database::create_appdb_connection()?;
    connection.execute(
        r#"
CREATE TABLE IF NOT EXISTS notifications
(
    id          INTEGER PRIMARY KEY AUTOINCREMENT,
    title       TEXT     NOT NULL DEFAULT '',
    message     TEXT     NOT NULL DEFAULT '',
    read        boolean  NOT NULL DEFAULT FALSE,
    archived    boolean  NOT NULL DEFAULT FALSE,
    sender_id   INTEGER  NOT NULL,
    sender_type INTEGER  NOT NULL,
    receiver_id INTEGER  NOT NULL,
    action      TEXT     NOT NULL DEFAULT '[]',
    created_at  datetime NOT NULL DEFAULT CURRENT_TIMESTAMP
);
	"#,
    )?;

    Ok(())
}

pub fn insert(notification: Notification) -> Result<(), Box<dyn Error>> {
    let connection = database::create_appdb_connection()?;
    let mut stmt=    connection.prepare(
        r#"
INSERT INTO notifications (title, message, read, archived, sender_id, sender_type, receiver_id, action)
VALUES (?, ?, ?, ?, ?, ?, ?, ?, ?);
"#)?;
    stmt.bind((1, notification.title.as_str()))?;
    stmt.bind((2, notification.message.as_str()))?;
    stmt.bind((3, notification.read as i64))?;
    stmt.bind((4, notification.archived as i64))?;
    stmt.bind((5, *decode(&notification.sender)?.first().unwrap() as i64))?;
    stmt.bind((6, notification.sender_type.parse() as i64))?;
    stmt.bind((7, *decode(&notification.receiver)?.first().unwrap() as i64))?;
    stmt.bind((8, serde_json::to_string(&notification.action)?.as_str()))?;

    Ok(())
}

pub fn update(id: impl AsRef<str>, notification: Notification) -> Result<(), Box<dyn Error>> {
    let connection = database::create_appdb_connection()?;
    let mut stmt = connection.prepare(
        r#"
UPDATE notifications
SET title       = ?,
    message     = ?,
    read        = ?,
    archived    = ?,
    sender_id   = ?,
    sender_type = ?,
    receiver_id = ?,
    action      = ?
WHERE id = ?;
"#,
    )?;
    stmt.bind((1, notification.title.as_str()))?;
    stmt.bind((2, notification.message.as_str()))?;
    stmt.bind((3, notification.read as i64))?;
    stmt.bind((4, notification.archived as i64))?;
    stmt.bind((5, *decode(&notification.sender)?.first().unwrap() as i64))?;
    stmt.bind((6, notification.sender_type.parse() as i64))?;
    stmt.bind((7, *decode(&notification.receiver)?.first().unwrap() as i64))?;
    stmt.bind((8, serde_json::to_string(&notification.action)?.as_str()))?;
    stmt.bind((9, *decode(id.as_ref())?.first().unwrap() as i64))?;

    Ok(())
}

pub fn delete(id: impl AsRef<str>) -> Result<(), Box<dyn Error>> {
    let connection = database::create_appdb_connection()?;
    let mut stmt = connection.prepare(
        r#"
DELETE FROM notifications
WHERE id = ?;
"#,
    )?;
    stmt.bind((1, *decode(id.as_ref())?.first().unwrap() as i64))?;

    Ok(())
}

pub fn get() -> Result<Vec<Notification>, Box<dyn Error>> {
    let connection = database::create_appdb_connection()?;
    let mut stmt = connection.prepare(r#"SELECT * FROM notifications;"#)?;
    let mut notifications = vec![];
    while let State::Row = stmt.next()? {
        notifications.push(Notification {
            id: encode(&[stmt.read::<i64, _>("id")? as u64]),
            title: stmt.read::<String, _>("title")?,
            message: stmt.read::<String, _>("message")?,
            read: stmt.read::<i64, _>("read")? == 1,
            archived: stmt.read::<i64, _>("archived")? == 1,
            sender: decode(&stmt.read::<String, _>("sender_id")?)?
                .first()
                .unwrap()
                .to_string(),
            sender_type: SenderType::from_number(stmt.read::<i64, _>("sender_type")? as u8)?,
            receiver: decode(&stmt.read::<String, _>("receiver_id")?)?
                .first()
                .unwrap()
                .to_string(),
            action: serde_json::from_str(&stmt.read::<String, _>("action")?)?,
            date: stmt.read::<String, _>("created_at")?,
        });
    }

    Ok(notifications)
}
