use crate::server_status::ServerStatus;
use crypto::hashids::{decode, encode};
use serde::de::{self, Deserializer, MapAccess, Visitor};
use serde::{Deserialize, Serialize, Serializer};
use std::any::Any;
use std::fmt;
use std::io::Stdin;
use std::path::PathBuf;
use std::str::FromStr;
use tokio::io::Stdout;

/// The `SimpleHashedIdentifier` structure is used to represent
/// an identifier in a hashed form, using the `serde_derive::Serialize` trait to enable serialization.
#[derive(serde_derive::Serialize)]
struct SimpleHashedIdentifier {
    /// Holds the hashed value of the identifier
    hashed: String,
}

/// The `SimpleIdentifier` structure is intended as a straightforward identifier
/// represented as a `u64`. Currently marked as dead code since it's not used.
#[derive(serde_derive::Serialize)]
struct SimpleIdentifier {
    /// Holds the identifier in a simple numeric form
    id: u64,
}

/// `Identifier` trait defines the behavior for converting objects
/// to a numeric form and into a hashed representation.
pub trait Identifier {
    /// Converts the identifier into a `u64` if possible.
    fn as_u64(&self) -> Option<u64>;

    /// Converts this identifier into a hashed form.
    fn to_hashed(&self) -> Option<Box<dyn HashedIdentifier>>;

    /// Returns a reference to the object as a trait object of `Any`.
    fn as_any(&self) -> &dyn Any;
}

/// `HashedIdentifier` trait is intended for identifiers that can return back
/// to their original `u64` form from a hashed state.
pub trait HashedIdentifier {
    /// Attempts to recover the original `u64` from the hashed identifier.
    fn original(&self) -> Option<u64>;

    /// Returns a reference to the object as a trait object of `Any`.
    fn as_any(&self) -> &dyn Any;
}

/// Implementation of the `Identifier` trait for `u64` values.
impl Identifier for u64 {
    fn as_u64(&self) -> Option<u64> {
        Some(*self) // Returns the `u64` directly
    }

    fn to_hashed(&self) -> Option<Box<dyn HashedIdentifier>> {
        // Encodes the `u64` into a hashed representation
        Some(Box::new(SimpleHashedIdentifier { hashed: encode(&[*self]) }))
    }

    fn as_any(&self) -> &dyn Any {
        self
    }
}

/// Implementation of the `Identifier` trait for `String`.
impl Identifier for String {
    fn as_u64(&self) -> Option<u64> {
        // Attempts to parse the string as a `u64`
        u64::from_str(self).ok()
    }

    fn to_hashed(&self) -> Option<Box<dyn HashedIdentifier>> {
        // Creates a `SimpleHashedIdentifier` from the string
        Some(Box::new(SimpleHashedIdentifier { hashed: self.to_string() }))
    }

    fn as_any(&self) -> &dyn Any {
        self
    }
}

/// Provides a custom display implementation for any `HashedIdentifier`.
impl fmt::Display for dyn HashedIdentifier {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        if let Some(simple_hashed_identifier) = self.as_any().downcast_ref::<SimpleHashedIdentifier>() {
            // Displays the hashed value
            write!(f, "{}", simple_hashed_identifier.hashed)
        } else {
            // Fallback message if downcasting fails
            write!(f, "Invalid HashedIdentifier")
        }
    }
}

/// Represents a server entity with attributes commonly used for server management and configuration.
pub struct Server<T: Identifier> {
    /// The server's unique identifier, constrained by the `Identifier` trait.
    pub id: T,
    /// The name of the server, used for display and identification purposes.
    pub name: String,
    /// The owner's unique identifier, typically representing a user or an account.
    pub owner: u64,
    /// A list of identifiers for each member associated with this server.
    pub members: Vec<u64>,
    /// Specifies the minimum required RAM (in MB) for the server to function.
    pub min_ram: u64,
    /// Specifies the maximum allowable RAM (in MB) for the server.
    pub max_ram: u64,
    /// Indicates whether the server should automatically start when the system boots.
    pub auto_start: bool,
    /// An optional file path to a script used to initiate the server.
    pub start_script: Option<PathBuf>,
    /// Optional arguments specific to Minecraft, affecting how the game runs.
    pub minecraft_arguments: Option<String>,
    /// Optional Java arguments, influencing the Java Virtual Machine's behavior.
    pub java_arguments: Option<String>,
    /// Represents the type of server loader being utilized, often an internal configuration value.
    pub loader_type: u8,
    /// An optional string indicating the version of the server loader in use.
    pub loader_version: Option<String>,
    /// Specifies the path to the server's operational working directory.
    pub directory: PathBuf,
    /// The creation timestamp of the server, formatted in ISO 8601.
    pub created_at: String,
    /// The timestamp of the last update made to the server, also formatted in ISO 8601.
    pub updated_at: String,
    /// Represents the current operational status of the server, using the `ServerStatus` enum.
    pub status: Option<ServerStatus>,
    /// Configuration related to the server's standard input stream.
    pub stdin: Option<Stdin>,
    /// Configuration related to the server's standard output stream.
    pub stdout: Option<Stdout>,
}

// Default implementation for `Server<u64>`.
impl Default for Server<u64> {
    /// Provides a default implementation for the `Server<u64>` struct, returning a server
    /// with pre-set initial values for all fields.
    ///
    /// A `Server<String>` instance with default values:
    ///
    /// - `id`: `0`.
    /// - `name`: Empty string.
    /// - `owner`: `0`.
    /// - `members`: An empty vector.
    /// - `auto_start`: `false`.
    /// - `min_ram`: `0`.
    /// - `max_ram`: `0`.
    /// - `start_script`: `None`.
    /// - `minecraft_arguments`: `None`.
    /// - `java_arguments`: `None`.
    /// - `loader_type`: `0`.
    /// - `loader_version`: `None`.
    /// - `directory`: New `PathBuf`.
    /// - `created_at`: `"1970-01-01T00:00:00Z"`.
    /// - `updated_at`: `"1970-01-01T00:00:00Z"`.
    /// - `status`: `None`.
    /// - `stdin`: `None`.
    /// - `stdout`: `None`.
    /// 
    /// # Returns
    /// * `Self` - An instance of `Server<u64>` with default values.
    fn default() -> Self {
        Self {
            id: 0,                                            // Default ID, set to the lowest possible value for a new server.
            name: String::from(""),                           // Empty name string for the server.
            owner: 0,                                         // Default owner ID, initially unattached or new.
            members: Vec::new(),                              // Initialize with an empty membership list.
            auto_start: false,                                // By default, the server does not start automatically.
            min_ram: 0,                                       // Minimum RAM requirement set to zero initially.
            max_ram: 0,                                       // Maximum RAM allowance is also zero initially.
            start_script: None,                               // No start script is associated by default.
            minecraft_arguments: None,                        // No Minecraft-specific arguments provided.
            java_arguments: None,                             // JVM arguments are absent by default.
            loader_type: 0,                                   // Default loader type, often a placeholder value.
            loader_version: None,                             // Loader version not specified initially.
            directory: PathBuf::new(),                        // New path buffer for the server's directory.
            created_at: String::from("1970-01-01T00:00:00Z"), // Default epoch start time in ISO 8601.
            updated_at: String::from("1970-01-01T00:00:00Z"), // Default epoch update time in ISO 8601.
            status: None,                                     // Server status is undefined by default.
            stdin: None,                                      // Standard input stream configuration is absent.
            stdout: None,                                     // Standard output stream configuration is absent.
        }
    }
}

// Default implementation for `Server<String>`.
impl Default for Server<String> {
    /// Provides a default implementation for the `Server<String>` struct.
    ///
    /// This function initializes an instance of `Server<String>` with default values,
    /// filling fields with neutral or placeholder values. It is particularly useful for
    /// creating new instances without manually setting each field.
    ///
    /// # Returns
    ///
    /// A `Server<String>` instance with default values:
    ///
    /// - `id`: Empty string.
    /// - `name`: Empty string.
    /// - `owner`: `0`.
    /// - `members`: An empty vector.
    /// - `auto_start`: `false`.
    /// - `min_ram`: `0`.
    /// - `max_ram`: `0`.
    /// - `start_script`: `None`.
    /// - `minecraft_arguments`: `None`.
    /// - `java_arguments`: `None`.
    /// - `loader_type`: `0`.
    /// - `loader_version`: `None`.
    /// - `directory`: New `PathBuf`.
    /// - `created_at`: `"1970-01-01T00:00:00Z"`.
    /// - `updated_at`: `"1970-01-01T00:00:00Z"`.
    /// - `status`: `None`.
    /// - `stdin`: `None`.
    /// - `stdout`: `None`.
    fn default() -> Self {
        Self {
            id: "".to_string(),                               // Default empty string for server ID
            name: String::from(""),                           // Default empty string for server name
            owner: 0,                                         // Default owner ID of 0
            members: Vec::new(),                              // Default to an empty list of members
            auto_start: false,                                // Default auto-start to false
            min_ram: 0,                                       // Default minimum RAM to 0 MB
            max_ram: 0,                                       // Default maximum RAM to 0 MB
            start_script: None,                               // Default start script to None
            minecraft_arguments: None,                        // Default Minecraft arguments to None
            java_arguments: None,                             // Default Java arguments to None
            loader_type: 0,                                   // Default loader type as 0
            loader_version: None,                             // Default loader version to None
            directory: PathBuf::new(),                        // Default directory as an empty PathBuf
            created_at: String::from("1970-01-01T00:00:00Z"), // Default creation timestamp
            updated_at: String::from("1970-01-01T00:00:00Z"), // Default updated timestamp
            status: None,                                     // Default status to None
            stdin: None,                                      // Default standard input to None
            stdout: None,                                     // Default standard output to None
        }
    }
}

// Implementation of `HashedIdentifier` trait for `SimpleHashedIdentifier`.
impl HashedIdentifier for SimpleHashedIdentifier {
    fn original(&self) -> Option<u64> {
        // Attempts to decode the hashed string back to its original `u64`
        decode(self.hashed.as_str()).ok()?.first().copied()
    }

    fn as_any(&self) -> &dyn Any {
        self
    }
}

// Serialization logic for `Server<u64>`.
impl Serialize for Server<u64> {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: Serializer,
    {
        // Uses `serde` to serialize the fields of the `Server` struct
        use serde::ser::SerializeStruct;

        // Starts the serialization process by creating a serialized representation
        // of the `Server` struct with the specified number of fields (17 in this case)
        let mut state = serializer.serialize_struct("Server", 17)?;

        // Serializes the `id` field; uses a custom function to convert it to a hashed string
        // If the `id` hashing fails, it defaults to "invalid id"
        state.serialize_field("id", &self.id.to_hashed().map_or_else(|| "invalid id".to_string(), |id| id.to_string()))?;

        // Serializes the `name` field of the server
        state.serialize_field("name", &self.name)?;

        // Serializes the `owner` field; represents the owner information
        state.serialize_field("owner", &self.owner)?;

        // Serializes the `members` field; represents the list of members associated with the server
        state.serialize_field("members", &self.members)?;

        // Serializes the `min_ram` field; represents the minimum required RAM for the server
        state.serialize_field("min_ram", &self.min_ram)?;

        // Serializes the `max_ram` field; represents the maximum RAM the server can use
        state.serialize_field("max_ram", &self.max_ram)?;

        // Serializes the `auto_start` field; indicates if the server should start automatically
        state.serialize_field("auto_start", &self.auto_start)?;

        // Serializes the `start_script` field; holds the script content needed to start the server
        state.serialize_field("start_script", &self.start_script)?;

        // Serializes the `minecraft_arguments` field; contains arguments for Minecraft execution
        state.serialize_field("minecraft_arguments", &self.minecraft_arguments)?;

        // Serializes the `java_arguments` field; holds the Java arguments used in server execution
        state.serialize_field("java_arguments", &self.java_arguments)?;

        // Serializes the `loader_type` field; identifies the type of loader being used
        state.serialize_field("loader_type", &self.loader_type)?;

        // Serializes the `loader_version` field; specifies the version of the loader
        state.serialize_field("loader_version", &self.loader_version)?;

        // Serializes the `directory` field; denotes the directory where server files are located
        state.serialize_field("directory", &self.directory)?;

        // Serializes the `created_at` field; records the timestamp when the server was created
        state.serialize_field("created_at", &self.created_at)?;

        // Serializes the `updated_at` field; records the timestamp of the last server update
        state.serialize_field("updated_at", &self.updated_at)?;

        // Serializes the `status` field; reflects the current status of the server
        state.serialize_field("status", &self.status)?;

        // Ends the serialization process for the `Server` struct
        state.end()
    }
}

impl<'de> Deserialize<'de> for Server<u64> {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: Deserializer<'de>,
    {
        #[derive(serde_derive::Deserialize)]
        enum Field {
            Id,
            Name,
            Owner,
            Members,
            MinRam,
            MaxRam,
            AutoStart,
            StartScript,
            MinecraftArguments,
            JavaArguments,
            LoaderType,
            LoaderVersion,
            Directory,
            CreatedAt,
            UpdatedAt,
            Status,
        }

        struct ServerVisitor;

        impl<'de> Visitor<'de> for ServerVisitor {
            type Value = Server<u64>;

            fn expecting(&self, formatter: &mut fmt::Formatter) -> fmt::Result {
                formatter.write_str("struct Server")
            }

            fn visit_map<V>(self, mut map: V) -> Result<Server<u64>, V::Error>
            where
                V: MapAccess<'de>,
            {
                // Declare mutable variables for each field to store parsed values
                let mut id = None;
                let mut name = None;
                let mut owner = None;
                let mut members = None;
                let mut min_ram = None;
                let mut max_ram = None;
                let mut auto_start = None;
                let mut start_script = None;
                let mut minecraft_arguments = None;
                let mut java_arguments = None;
                let mut loader_type = None;
                let mut loader_version = None;
                let mut directory = None;
                let mut created_at = None;
                let mut updated_at = None;
                let mut status = None;

                // Iterate over each key-value pair in the map
                while let Some(key) = map.next_key()? {
                    // Match each key to its corresponding field
                    match key {
                        // Parse and assign the 'id' field, ensuring no duplicates
                        Field::Id => {
                            if id.is_some() {
                                return Err(de::Error::duplicate_field("id"));
                            }
                            id = Some(map.next_value()?);
                        }
                        // Parse and assign the 'name' field, ensuring no duplicates
                        Field::Name => {
                            if name.is_some() {
                                return Err(de::Error::duplicate_field("name"));
                            }
                            name = Some(map.next_value()?);
                        }
                        // Parse and assign other fields similarly, checking for duplicates
                        Field::Owner => {
                            if owner.is_some() {
                                return Err(de::Error::duplicate_field("owner"));
                            }
                            owner = Some(map.next_value()?);
                        }
                        Field::Members => {
                            if members.is_some() {
                                return Err(de::Error::duplicate_field("members"));
                            }
                            members = Some(map.next_value()?);
                        }
                        Field::MinRam => {
                            if min_ram.is_some() {
                                return Err(de::Error::duplicate_field("min_ram"));
                            }
                            min_ram = Some(map.next_value()?);
                        }
                        Field::MaxRam => {
                            if max_ram.is_some() {
                                return Err(de::Error::duplicate_field("max_ram"));
                            }
                            max_ram = Some(map.next_value()?);
                        }
                        Field::AutoStart => {
                            if auto_start.is_some() {
                                return Err(de::Error::duplicate_field("auto_start"));
                            }
                            auto_start = Some(map.next_value()?);
                        }
                        Field::StartScript => {
                            if start_script.is_some() {
                                return Err(de::Error::duplicate_field("start_script"));
                            }
                            start_script = Some(map.next_value()?);
                        }
                        Field::MinecraftArguments => {
                            if minecraft_arguments.is_some() {
                                return Err(de::Error::duplicate_field("minecraft_arguments"));
                            }
                            minecraft_arguments = Some(map.next_value()?);
                        }
                        Field::JavaArguments => {
                            if java_arguments.is_some() {
                                return Err(de::Error::duplicate_field("java_arguments"));
                            }
                            java_arguments = Some(map.next_value()?);
                        }
                        Field::LoaderType => {
                            if loader_type.is_some() {
                                return Err(de::Error::duplicate_field("loader_type"));
                            }
                            loader_type = Some(map.next_value()?);
                        }
                        Field::LoaderVersion => {
                            if loader_version.is_some() {
                                return Err(de::Error::duplicate_field("loader_version"));
                            }
                            loader_version = Some(map.next_value()?);
                        }
                        Field::Directory => {
                            if directory.is_some() {
                                return Err(de::Error::duplicate_field("directory"));
                            }
                            directory = Some(map.next_value()?);
                        }
                        Field::CreatedAt => {
                            if created_at.is_some() {
                                return Err(de::Error::duplicate_field("created_at"));
                            }
                            created_at = Some(map.next_value()?);
                        }
                        Field::UpdatedAt => {
                            if updated_at.is_some() {
                                return Err(de::Error::duplicate_field("updated_at"));
                            }
                            updated_at = Some(map.next_value()?);
                        }
                        Field::Status => {
                            if status.is_some() {
                                return Err(de::Error::duplicate_field("status"));
                            }
                            status = Some(map.next_value()?);
                        }
                    }
                }

                // Validate that all required fields have been set, and handle missing fields
                let id = id.ok_or_else(|| de::Error::missing_field("id"))?;
                let name = name.ok_or_else(|| de::Error::missing_field("name"))?;
                let owner = owner.ok_or_else(|| de::Error::missing_field("owner"))?;
                let members = members.ok_or_else(|| de::Error::missing_field("members"))?;
                let min_ram = min_ram.ok_or_else(|| de::Error::missing_field("min_ram"))?;
                let max_ram = max_ram.ok_or_else(|| de::Error::missing_field("max_ram"))?;
                let auto_start = auto_start.ok_or_else(|| de::Error::missing_field("auto_start"))?;
                let start_script = start_script;
                let minecraft_arguments = minecraft_arguments;
                let java_arguments = java_arguments;
                let loader_type = loader_type.ok_or_else(|| de::Error::missing_field("loader_type"))?;
                let loader_version = loader_version;
                let directory = directory.ok_or_else(|| de::Error::missing_field("directory"))?;
                let created_at = created_at.ok_or_else(|| de::Error::missing_field("created_at"))?;
                let updated_at = updated_at.ok_or_else(|| de::Error::missing_field("updated_at"))?;
                let status = status;

                // Construct and return the Server object
                Ok(Server {
                    id,
                    name,
                    owner,
                    members,
                    min_ram,
                    max_ram,
                    auto_start,
                    start_script,
                    minecraft_arguments,
                    java_arguments,
                    loader_type,
                    loader_version,
                    directory,
                    created_at,
                    updated_at,
                    status,
                    stdin: None,
                    stdout: None,
                })
            }
        }

        const FIELDS: &[&str] = &[
            "id",
            "name",
            "owner",
            "members",
            "min_ram",
            "max_ram",
            "auto_start",
            "start_script",
            "minecraft_arguments",
            "java_arguments",
            "loader_type",
            "loader_version",
            "directory",
            "created_at",
            "updated_at",
            "status",
        ];
        deserializer.deserialize_struct("Server", FIELDS, ServerVisitor)
    }
}
