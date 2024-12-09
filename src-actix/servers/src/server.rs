use crate::server_status::ServerStatus;
use crypto::hashids::encode;
use serde::de::{self, Deserializer, MapAccess, Visitor};
use serde::{Deserialize, Serialize, Serializer};
use std::any::Any;
use std::fmt;
use std::io::Stdin;
use std::path::PathBuf;
use std::str::FromStr;
use tokio::io::Stdout;

#[derive(serde_derive::Serialize)]
struct SimpleHashedIdentifier {
    hashed: String,
}

#[derive(serde_derive::Serialize)]
struct SimpleIdentifier {
    id: u64,
}

pub trait Identifier {
    fn as_u64(&self) -> Option<u64>;
    fn to_hashed(&self) -> Option<Box<dyn HashedIdentifier>>;

    fn as_any(&self) -> &dyn Any;
}

pub trait HashedIdentifier {
    fn original(&self) -> Option<u64>;
    fn as_any(&self) -> &dyn Any;
}

impl Identifier for u64 {
    fn as_u64(&self) -> Option<u64> {
        Some(*self)
    }
    fn to_hashed(&self) -> Option<Box<dyn HashedIdentifier>> {
        Some(Box::new(SimpleHashedIdentifier { hashed: encode(&[*self]) }))
    }
    fn as_any(&self) -> &dyn Any {
        self
    }
}

impl Identifier for String{
    fn as_u64(&self) -> Option<u64> {
        u64::from_str(self).ok()
    }

    fn to_hashed(&self) -> Option<Box<dyn HashedIdentifier>> {
        Some(Box::new(SimpleHashedIdentifier{
            hashed: self.to_string(),
        }))
    }

    fn as_any(&self) -> &dyn Any {
        self
    }
}

impl fmt::Display for dyn HashedIdentifier {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        if let Some(simple_hashed_identifier) = self.as_any().downcast_ref::<SimpleHashedIdentifier>() {
            write!(f, "{}", simple_hashed_identifier.hashed)
        } else {
            write!(f, "Invalid HashedIdentifier")
        }
    }
}

pub struct Server<T: Identifier> {
    pub id: T,
    pub name: String,
    pub owner: u64,
    pub members: Vec<u64>,
    pub min_ram: u64,
    pub max_ram: u64,
    pub auto_start: bool,
    pub start_script: Option<PathBuf>,
    pub minecraft_arguments: Option<String>,
    pub java_arguments: Option<String>,
    pub loader_type: u8,
    pub loader_version: Option<String>,
    pub directory: PathBuf,
    pub created_at: String,
    pub updated_at: String,
    pub status: Option<ServerStatus>,
    pub stdin: Option<Stdin>,
    pub stdout: Option<Stdout>,
}

impl Default for Server<u64> {
    fn default() -> Self {
        Self {
            id: 0,
            name: String::from(""),
            owner: 0,
            members: Vec::new(),
            auto_start: false,
            min_ram: 0,
            max_ram: 0,
            start_script: None,
            minecraft_arguments: None,
            java_arguments: None,
            loader_type: 0,
            loader_version: None,
            directory: PathBuf::new(),
            created_at: String::from("1970-01-01T00:00:00Z"),
            updated_at: String::from("1970-01-01T00:00:00Z"),
            status: None,
            stdin: None,
            stdout: None,
        }
    }
}

impl Default for Server<String>{
    fn default() -> Self {
        Self {
            id: "".to_string(),
            name: String::from(""),
            owner: 0,
            members: Vec::new(),
            auto_start: false,
            min_ram: 0,
            max_ram: 0,
            start_script: None,
            minecraft_arguments: None,
            java_arguments: None,
            loader_type: 0,
            loader_version: None,
            directory: PathBuf::new(),
            created_at: String::from("1970-01-01T00:00:00Z"),
            updated_at: String::from("1970-01-01T00:00:00Z"),
            status: None,
            stdin: None,
            stdout: None,
        }
    }
}

impl HashedIdentifier for SimpleHashedIdentifier {
    fn original(&self) -> Option<u64> {
        // Implement the logic to convert `hashed` back to `u64`, if possible
        None
    }

    fn as_any(&self) -> &dyn Any {
        self
    }
}

impl<T: Identifier> Server<T> {}

impl Serialize for Server<u64> {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: Serializer,
    {
        use serde::ser::SerializeStruct;

        let mut state = serializer.serialize_struct("Server", 17)?;
        state.serialize_field("id", &self.id.to_hashed().unwrap().to_string())?;
        state.serialize_field("name", &self.name)?;
        state.serialize_field("owner", &self.owner)?;
        state.serialize_field("members", &self.members)?;
        state.serialize_field("min_ram", &self.min_ram)?;
        state.serialize_field("max_ram", &self.max_ram)?;
        state.serialize_field("auto_start", &self.auto_start)?;
        state.serialize_field("start_script", &self.start_script)?;
        state.serialize_field("minecraft_arguments", &self.minecraft_arguments)?;
        state.serialize_field("java_arguments", &self.java_arguments)?;
        state.serialize_field("loader_type", &self.loader_type)?;
        state.serialize_field("loader_version", &self.loader_version)?;
        state.serialize_field("directory", &self.directory)?;
        state.serialize_field("created_at", &self.created_at)?;
        state.serialize_field("updated_at", &self.updated_at)?;
        state.serialize_field("status", &self.status)?;
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
        };

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

                while let Some(key) = map.next_key()? {
                    match key {
                        Field::Id => {
                            if id.is_some() {
                                return Err(de::Error::duplicate_field("id"));
                            }
                            id = Some(map.next_value()?);
                        }
                        Field::Name => {
                            if name.is_some() {
                                return Err(de::Error::duplicate_field("name"));
                            }
                            name = Some(map.next_value()?);
                        }
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

