use serde::de::Error;
use serde::{Deserialize, Deserializer, Serialize};
use std::path::Path;

#[derive(PartialEq, Eq)]
/// Enum representing the types of executables that can be started.
pub enum StartExecutableType {
    Jar,
    Executable,
    Script,
}

impl Default for StartExecutableType {
    /// Provides the default executable type, which is `Jar`.
    fn default() -> Self {
        Self::Jar
    }
}

impl Serialize for StartExecutableType {
    /// Serializes the executable type into a string for JSON representation.
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        match self {
            StartExecutableType::Jar => serializer.serialize_str("jar"),
            StartExecutableType::Executable => serializer.serialize_str("executable"),
            StartExecutableType::Script => serializer.serialize_str("script"),
        }
    }
}

impl<'de> Deserialize<'de> for StartExecutableType {
    /// Deserializes a string into a `StartExecutableType`.
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: Deserializer<'de>,
    {
        let s = String::deserialize(deserializer)?;
        match s.as_str() {
            "jar" => Ok(StartExecutableType::Jar),
            "executable" => Ok(StartExecutableType::Executable),
            "script" => Ok(StartExecutableType::Script),
            // Returns an error if the string doesn't match any known type.
            _ => Err(Error::custom("invalid variant")),
        }
    }
}

/// Trait extension to determine `StartExecutableType` from a file path.
pub trait StartExecutableTypeExt {
    /// Determines the `StartExecutableType` from the file's extension.
    ///
    /// # Arguments
    ///
    /// * `path` - A reference to a path.
    ///
    /// # Returns
    ///
    /// * `Result<Self, Box<dyn std::error::Error>>` - Returns the executable type or an error if the extension is not recognized.
    fn from_path(path: impl AsRef<Path>) -> Result<Self, Box<dyn std::error::Error>>
    where
        Self: Sized;
}

impl StartExecutableTypeExt for StartExecutableType {
    fn from_path(path: impl AsRef<Path>) -> Result<Self, Box<dyn std::error::Error>> {
        // Extract the file extension from the given path.
        let extension = path.as_ref().extension().unwrap_or_default().to_str().unwrap_or("");

        // Determine the executable type based on the file extension.
        match extension {
            "jar" => Ok(StartExecutableType::Jar),
            "exe" => Ok(StartExecutableType::Executable),
            "sh" | "bat" | "cmd" | "ps1" => Ok(StartExecutableType::Script),
            "" => Ok(StartExecutableType::Executable), // Default to Executable if no extension is provided.
            // Returns an error for unknown extensions.
            _ => Err(format!("Invalid start executable extension: {:?}", extension).into()),
        }
    }
}
