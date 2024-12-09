use serde::{Serialize, Serializer};
use serde_derive::Deserialize;
use std::fmt::Display;
use std::str::FromStr;

/// Represents the various possible statuses of a server.

#[derive(Debug, Clone)]
pub enum ServerStatus {
    Offline,      // Indicates that the server is offline
    Online,       // Indicates that the server is online
    Crashed,      // Indicates that the server has crashed
    Starting,     // Indicates that the server is in the process of starting
    Stopping,     // Indicates that the server is in the process of stopping
    Restarting,   // Indicates that the server is restarting
    Updating,     // Indicates that the server is updating its components
    Installing,   // Indicates that the server is installing new components
    Uninstalling, // Indicates that the server is uninstalling components
    Reloading,    // Indicates that the server is reloading its configuration
    Deleting,     // Indicates the server is being deleted
    Creating,     // Indicates a new server instance is being created
}

impl Default for ServerStatus {
    /// Provides a default server status; in this case, `Offline`.
    fn default() -> Self {
        Self::Offline
    }
}

impl Serialize for ServerStatus {
    /// Serializes the `ServerStatus` into a string representation suitable for JSON.
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: Serializer,
    {
        match self {
            // Serializes each enum variant to its string equivalent
            ServerStatus::Offline => serializer.serialize_str("offline"),
            ServerStatus::Online => serializer.serialize_str("online"),
            ServerStatus::Crashed => serializer.serialize_str("crashed"),
            ServerStatus::Starting => serializer.serialize_str("starting"),
            ServerStatus::Stopping => serializer.serialize_str("stopping"),
            ServerStatus::Restarting => serializer.serialize_str("restarting"),
            ServerStatus::Updating => serializer.serialize_str("updating"),
            ServerStatus::Installing => serializer.serialize_str("installing"),
            ServerStatus::Uninstalling => serializer.serialize_str("uninstalling"),
            ServerStatus::Reloading => serializer.serialize_str("reloading"),
            ServerStatus::Deleting => serializer.serialize_str("deleting"),
            ServerStatus::Creating => serializer.serialize_str("creating"),
        }
    }
}

impl<'de> serde::de::Deserialize<'de> for ServerStatus {
    /// Deserializes a string into a `ServerStatus`.
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: serde::de::Deserializer<'de>,
    {
        struct Visitor;

        impl<'de> serde::de::Visitor<'de> for Visitor {
            type Value = ServerStatus;

            /// Defines what is expected when deserializing the data.
            fn expecting(&self, formatter: &mut std::fmt::Formatter) -> std::fmt::Result {
                formatter.write_str("a valid server status string")
            }

            /// Converts a string to the corresponding `ServerStatus` variant.
            fn visit_str<E>(self, value: &str) -> Result<Self::Value, E>
            where
                E: serde::de::Error,
            {
                match value {
                    "offline" => Ok(ServerStatus::Offline),
                    "online" => Ok(ServerStatus::Online),
                    "crashed" => Ok(ServerStatus::Crashed),
                    "starting" => Ok(ServerStatus::Starting),
                    "stopping" => Ok(ServerStatus::Stopping),
                    "restarting" => Ok(ServerStatus::Restarting),
                    "updating" => Ok(ServerStatus::Updating),
                    "installing" => Ok(ServerStatus::Installing),
                    "uninstalling" => Ok(ServerStatus::Uninstalling),
                    "reloading" => Ok(ServerStatus::Reloading),
                    "deleting" => Ok(ServerStatus::Deleting),
                    "creating" => Ok(ServerStatus::Creating),
                    // Returns an error if the provided string doesn't match any known server status
                    _ => Err(E::custom(format!("unknown server status: {}", value))),
                }
            }
        }

        deserializer.deserialize_str(Visitor)
    }
}

impl Display for ServerStatus {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let str = match self {
            ServerStatus::Offline => "offline".to_string(),
            ServerStatus::Online => "online".to_string(),
            ServerStatus::Crashed => "crashed".to_string(),
            ServerStatus::Starting => "starting".to_string(),
            ServerStatus::Stopping => "stopping".to_string(),
            ServerStatus::Restarting => "restarting".to_string(),
            ServerStatus::Updating => "updating".to_string(),
            ServerStatus::Installing => "installing".to_string(),
            ServerStatus::Uninstalling => "uninstalling".to_string(),
            ServerStatus::Reloading => "reloading".to_string(),
            ServerStatus::Deleting => "deleting".to_string(),
            ServerStatus::Creating => "creating".to_string(),
        };
        write!(f, "{}", str)
    }
}
impl FromStr for ServerStatus {
    type Err = ();

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s.to_lowercase().as_str() {
            "offline" => Ok(ServerStatus::Offline),
            "online" => Ok(ServerStatus::Online),
            "crashed" => Ok(ServerStatus::Crashed),
            "starting" => Ok(ServerStatus::Starting),
            "stopping" => Ok(ServerStatus::Stopping),
            "restarting" => Ok(ServerStatus::Restarting),
            "updating" => Ok(ServerStatus::Updating),
            "installing" => Ok(ServerStatus::Installing),
            "uninstalling" => Ok(ServerStatus::Uninstalling),
            "reloading" => Ok(ServerStatus::Reloading),
            "deleting" => Ok(ServerStatus::Deleting),
            "creating" => Ok(ServerStatus::Creating),
            _ => Err(()),
        }
    }
}
