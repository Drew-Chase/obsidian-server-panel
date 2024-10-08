use crate::server_db;
use crate::server_db::{get_server_by_id, Server};
use log::{debug, error, info};
use serde_derive::{Deserialize, Serialize};
use std::collections::HashMap;
use std::fs::create_dir;
use std::path::{Path, PathBuf};
use std::time::SystemTime;

#[derive(Debug, Serialize, Deserialize)]
pub struct FileSystemEntry {
    pub name: String,
    pub path: PathBuf,
    pub is_dir: bool,
    pub size: u64,
    pub r#type: String,
    pub created: SystemTime,
    pub last_modified: SystemTime,
}

fn clean_file_path_string(name: &str) -> String {
    let clean = name
        .trim()
        .replace(
            |c: char| c.is_whitespace() || (!c.is_alphanumeric() && c != '_'),
            "_",
        )
        .to_lowercase();
    debug!("'{}' -> '{}'", name, clean);
    clean
}

pub fn create_server_directory(id: u32) -> Result<PathBuf, String> {
    let server: Server = match get_server_by_id(id) {
        Some(s) => s,
        None => {
            let msg = format!("Server with id: {} not found", id);
            error!("{}", msg);
            return Err(msg);
        }
    };
    let name = clean_file_path_string(server.name.as_str());
    let path = Path::join(Path::new(&get_servers_directory()), Path::new(&name));
    let path = find_unique_directory_name(&path);

    match create_dir(&path) {
        Ok(_) => {
            info!("Created server directory: {:?}", path);
        }
        Err(e) => {
            error!("Failed to create server directory: {}", e);
            return Err(format!("Failed to create server directory: {}", e));
        }
    }
    Ok(path)
}

fn find_unique_directory_name(path: &PathBuf) -> PathBuf {
    if !path.exists() {
        return path.clone();
    } // path does not exist, return it as is
    let mut i = 1;
    let mut new_path = path.clone();
    while new_path.exists() {
        new_path = path.with_file_name(format!(
            "{}-{}",
            path.file_name().unwrap().to_str().unwrap(),
            i
        ));
        i += 1;
    }
    new_path
}

pub fn get_servers_directory() -> PathBuf {
    let path = "./servers";
    let path = match Path::new(path).canonicalize() {
        Ok(p) => p,
        Err(e) => {
            let msg = format!("Could not canonicalize servers directory: {} {}", path, e);
            error!("{}", msg);
            panic!("{}", msg);
        }
    };
    if !path.exists() {
        match create_dir(&path) {
            Ok(_) => {
                info!("Created servers directory: {:?}", path);
            }
            Err(e) => {
                error!("Failed to create servers directory: {}", e);
                panic!("Failed to create servers directory: {}", e);
            }
        };
    }
    info!("Using servers directory: {:?}", path);
    path
}

pub fn get_server_filesystem_entries(
    id: u32,
    owner: u32,
    sub_path: Option<String>,
) -> Vec<FileSystemEntry> {
    let mut entries: Vec<FileSystemEntry> = vec![];

    if let Some(dir_iter) = get_server_directory_iterator(id, owner, sub_path) {
        for file in dir_iter {
            let file = match file {
                Ok(f) => f,
                Err(e) => {
                    error!(
                        "Unable to read path from directory iterator: {}",
                        e.to_string()
                    );
                    continue;
                }
            };
            let metadata = match file.metadata() {
                Ok(m) => m,
                Err(e) => {
                    error!("Unable to get file metadata: {}", e.to_string());
                    continue;
                }
            };

            entries.push(FileSystemEntry {
                name: file.file_name().to_str().unwrap().to_string(),
                path: file.path(),
                is_dir: metadata.is_dir(),
                size: metadata.len(),
                created: metadata
                    .created()
                    .unwrap_or_else(|_| SystemTime::UNIX_EPOCH),
                last_modified: metadata
                    .modified()
                    .unwrap_or_else(|_| SystemTime::UNIX_EPOCH),
                r#type: get_file_type(
                    file.path()
                        .extension()
                        .unwrap_or_default()
                        .to_str()
                        .unwrap_or_default()
                        .to_string(),
                ),
            });
        }
    }

    entries
}
fn get_server_directory_iterator(
    id: u32,
    owner: u32,
    sub_path: Option<String>,
) -> Option<std::fs::ReadDir> {
    if let Some(server) = server_db::get_owned_server_by_id(id, owner) {
        if let Some(mut directory) = server.directory {
            directory += sub_path?.as_str();
            debug!("Searching path: {:?}", directory);
            return match std::fs::read_dir(directory) {
                Ok(iter) => Some(iter),
                Err(e) => {
                    error!("Failed to read servers directory: {}", e);
                    None
                }
            };
        }
    }
    None
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

    types
        .get(&extension[..])
        .unwrap_or(&extension.as_str())
        .to_string()
}
