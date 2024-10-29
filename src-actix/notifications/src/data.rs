use num_traits::FromPrimitive;
use serde::{Deserialize, Serialize};
use std::fmt::Display;

/// Represents a notification.
#[derive(Serialize, Deserialize)]
pub struct Notification {
    /// Unique identifier for the notification.
    pub id: String,

    /// Title of the notification.
    pub title: String,

    /// Message body of the notification.
    pub message: String,

    /// Indicates if the notification has been read.
    pub read: bool,

    /// Indicates if the notification has been archived.
    pub archived: bool,

    /// Actions associated with the notification.
    pub action: Vec<NotificationAction>,

    /// The id of the sender, it could be a user id or a server id.
    /// If blank, it means the notification is from the system.
    pub sender: String,

    /// The id of the receiver, this should be the user's id.
    /// If blank, it means the notification is for all users.
    pub receiver: String,

    /// The type of the sender, it could be a user, server or system.
    pub sender_type: SenderType,

    /// The date the notification was sent.
    pub date: String,
}

/// Represents an action that can be taken on a notification.

#[derive(Serialize, Deserialize)]
pub struct NotificationAction {
    /// Label of the action.
    pub label: String,

    /// Command to be executed when the action is taken.
    pub command: String,

    /// Color associated with the action.
    pub color: String,
}

/// Represents the type of the sender: user, server, or system.
#[derive(Serialize, Deserialize)]
pub enum SenderType {
    User,
    Server,
    System,
}

impl Display for SenderType {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            SenderType::User => write!(f, "User"),
            SenderType::Server => write!(f, "Server"),
            SenderType::System => write!(f, "System"),
        }
    }
}

pub trait AsNumber {
    fn parse(&self) -> u8;
    fn from_number<N>(n: N) -> Result<Self, Box<dyn std::error::Error>>
    where
        N: FromPrimitive + Into<u8>,
        Self: Sized;
}

impl AsNumber for SenderType {
    fn parse(&self) -> u8 {
        match self {
            SenderType::User => 1,
            SenderType::Server => 2,
            SenderType::System => 3,
        }
    }
    fn from_number<N>(n: N) -> Result<Self, Box<dyn std::error::Error>>
    where
        N: FromPrimitive + Into<u8>,
    {
        match n.into() {
            1 => Ok(SenderType::User),
            2 => Ok(SenderType::Server),
            3 => Ok(SenderType::System),
            _ => Err("Invalid number".into()),
        }
    }
}
