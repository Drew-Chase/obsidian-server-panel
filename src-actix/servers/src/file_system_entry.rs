use serde_derive::{Deserialize, Serialize};
use std::collections::HashMap;
use std::ffi::OsStr;
use std::fs;
use std::fs::File;
use std::io::Read;
use std::path::{Path, PathBuf};
use std::time::SystemTime;

#[derive(Debug, Serialize, Deserialize)]
pub struct FileSystemEntry {
    pub name: String,
    pub path: PathBuf,
    pub is_dir: bool,
    pub size: u64,
    pub r#type: String,
    pub mime: Option<String>,
    pub category: FileMimeCategory,
    pub created: SystemTime,
    pub last_modified: SystemTime,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct FileSystemEntries {
    pub parent: Option<PathBuf>,
    pub entries: Vec<FileSystemEntry>,
}

#[derive(Debug, Serialize, Deserialize)]
pub enum FileMimeCategory {
    TEXT,
    IMAGE,
    AUDIO,
    ARCHIVE,
    VIDEO,
    UNKNOWN,
}
fn get_file_type(extension: String) -> String {
    let types: HashMap<&str, &str> = HashMap::from([
        ("zip", "Zip Archive"),
        ("tar", "Tar Archive"),
        ("tar.gz", "Tar GZip Archive"),
        ("tar.bz2", "Tar BZip2 Archive"),
        ("tar.xz", "Tar XZ Archive"),
        ("7z", "7-Zip Archive"),
        ("rar", "RAR Archive"),
        ("jar", "Java Archive"),
        ("war", "Web Archive"),
        ("ear", "Enterprise Archive"),
        ("exe", "Windows Executable"),
        ("msi", "Windows Installer"),
        ("sh", "Shell Script"),
        ("bat", "Batch Script"),
        ("cmd", "Command Script"),
        ("py", "Python Script"),
        ("rb", "Ruby Script"),
        ("pl", "Perl Script"),
        ("php", "PHP Script"),
        ("html", "HTML Document"),
        ("htm", "HTML Document"),
        ("xhtml", "XHTML Document"),
        ("css", "CSS Stylesheet"),
        ("js", "JavaScript File"),
        ("ts", "TypeScript File"),
        ("jsx", "JavaScript XML"),
        ("tsx", "TypeScript XML"),
        ("json", "JSON File"),
        ("xml", "XML Document"),
        ("yaml", "YAML Document"),
        ("yml", "YAML Document"),
        ("toml", "TOML Config"),
        ("ini", "INI Config"),
        ("cfg", "Configuration File"),
        ("conf", "Configuration File"),
        ("log", "Log File"),
        ("md", "Markdown Document"),
        ("txt", "Text File"),
        ("csv", "CSV File"),
        ("tsv", "TSV File"),
        ("pdf", "PDF Document"),
        ("doc", "Word Document"),
        ("docx", "Word Document"),
        ("xls", "Excel Spreadsheet"),
        ("xlsx", "Excel Spreadsheet"),
        ("ppt", "PowerPoint Presentation"),
        ("pptx", "PowerPoint Presentation"),
        ("odt", "OpenDocument Text"),
        ("ods", "OpenDocument Spreadsheet"),
        ("odp", "OpenDocument Presentation"),
        ("jpg", "JPEG Image"),
        ("jpeg", "JPEG Image"),
        ("png", "PNG Image"),
        ("gif", "GIF Image"),
        ("bmp", "Bitmap Image"),
        ("tiff", "TIFF Image"),
        ("ico", "Icon Image"),
        ("svg", "SVG Image"),
        ("mp3", "MP3 Audio"),
        ("wav", "WAV Audio"),
        ("flac", "FLAC Audio"),
        ("ogg", "OGG Audio"),
        ("aac", "AAC Audio"),
        ("m4a", "M4A Audio"),
        ("wma", "WMA Audio"),
        ("mp4", "MP4 Video"),
        ("m4v", "M4V Video"),
        ("mkv", "MKV Video"),
        ("avi", "AVI Video"),
        ("mov", "MOV Video"),
        ("wmv", "WMV Video"),
        ("flv", "FLV Video"),
        ("webm", "WebM Video"),
        ("vob", "DVD Video"),
        ("mpg", "MPEG Video"),
        ("mpeg", "MPEG Video"),
        ("iso", "ISO Disk Image"),
        ("dmg", "MacOS Disk Image"),
        ("vdi", "VirtualBox Disk Image"),
        ("vmdk", "VMware Disk Image"),
        ("qcow2", "QEMU Copy-On-Write Disk Image"),
        ("qcow", "QEMU Copy-On-Write Disk Image"),
        ("ova", "Virtual Appliance"),
        ("ovf", "Open Virtualization Format"),
        ("img", "Disk Image"),
        ("dd", "Disk Dump Image"),
        ("vhd", "Virtual Hard Disk"),
        ("vhdx", "Virtual Hard Disk"),
        ("xpi", "Mozilla Add-on"),
        ("crx", "Chrome Extension"),
        ("oxt", "OpenOffice Extension"),
        ("apk", "Android Package"),
        ("ipa", "iOS App Package"),
        ("deb", "Debian Package"),
        ("rpm", "Red Hat Package"),
        ("flatpak", "Flatpak Package"),
        ("mcworld", "Minecraft World"),
        ("mcpack", "Minecraft Resource Pack"),
        ("mcaddon", "Minecraft Add-On"),
        ("mctemplate", "Minecraft Template"),
        ("mclevel", "Minecraft Level"),
        ("schematic", "Minecraft Schematic"),
        ("dat", "Minecraft Data File"),
        ("ldb", "Minecraft LevelDB Database File"),
        ("mca", "Minecraft Anvil Data"),
        ("mcr", "Minecraft Region Data"),
        ("nbt", "Minecraft Named Binary Tag"),
        ("mcfunction", "Minecraft Function File"),
        ("mcmeta", "Minecraft Metadata File"),
        ("properties", "Minecraft Properties File"),
    ]);

    types.get(&extension[..]).unwrap_or(&extension.as_str()).to_string()
}

fn get_mime_category(path: impl AsRef<Path>) -> FileMimeCategory {
    if !path.as_ref().exists() || path.as_ref().is_dir() {
        return FileMimeCategory::UNKNOWN;
    }

    let mime = mime_guess::from_path(&path).first();
    if let Some(mime) = mime {
        let mime = mime.type_().as_str();
        match mime {
            "text" => FileMimeCategory::TEXT,
            "image" => FileMimeCategory::IMAGE,
            "audio" => FileMimeCategory::AUDIO,
            "video" => FileMimeCategory::VIDEO,
            "application" => FileMimeCategory::ARCHIVE,
            _ => FileMimeCategory::UNKNOWN,
        }
    } else {
        if is_text_file(path) {
            return FileMimeCategory::TEXT;
        }
        FileMimeCategory::UNKNOWN
    }
}

fn is_text_file(file_path: impl AsRef<Path>) -> bool {
    const BUFFER_SIZE: usize = 1024;
    let path = file_path.as_ref();

    if let Ok(mut file) = File::open(path) {
        let mut buffer = [0; BUFFER_SIZE];

        if let Ok(bytes_read) = file.read(&mut buffer) {
            for &byte in &buffer[..bytes_read] {
                if byte != 0x09 && byte != 0x0A && byte != 0x0D && !(0x20..=0x7E).contains(&byte) {
                    return false;
                }
            }
            return true;
        }
    }

    false
}

fn get_mime(path: impl AsRef<Path>) -> Option<String> {
    mime_guess::from_path(path).first().map(|m| m.to_string())
}

impl Default for FileSystemEntry {
    fn default() -> Self {
        Self {
            name: "".to_string(),
            path: Default::default(),
            is_dir: false,
            size: 0,
            r#type: "".to_string(),
            mime: None,
            category: FileMimeCategory::TEXT,
            created: SystemTime::now(),
            last_modified: SystemTime::now(),
        }
    }
}

impl Default for FileSystemEntries {
    fn default() -> Self {
        Self {
            parent: None,
            entries: Vec::new(),
        }
    }
}

impl From<PathBuf> for FileSystemEntry {
    fn from(value: PathBuf) -> Self {
        if let Ok(metadata) = value.metadata() {
            return Self {
                name: value
                    .file_name()
                    .unwrap_or(OsStr::new(""))
                    .to_string_lossy()
                    .to_string(),
                path: value.clone(),
                is_dir: metadata.is_dir(),
                size: metadata.len(),
                r#type: get_file_type(
                    value
                        .extension()
                        .unwrap_or(OsStr::new(""))
                        .to_string_lossy()
                        .to_string(),
                ),
                mime: get_mime(&value),
                category: get_mime_category(&value),
                created: metadata.created().unwrap_or(SystemTime::UNIX_EPOCH),
                last_modified: metadata.modified().unwrap_or(SystemTime::UNIX_EPOCH),
            };
        }
        Self::default()
    }
}

impl From<PathBuf> for FileSystemEntries {
    fn from(value: PathBuf) -> Self {
        if let Ok(directory_entries) = fs::read_dir(&value) {
            let mut entries: Vec<FileSystemEntry> = Vec::new();
            for entry in directory_entries.flatten() {
                entries.push(entry.path().into());
            }
            return Self {
                parent: value.parent().map(|p| p.to_path_buf()),
                entries,
            };
        }
        Self::default()
    }
}
