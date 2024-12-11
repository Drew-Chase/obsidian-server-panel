use actix_multipart::form::{json::Json as MPJson, tempfile::TempFile, MultipartForm};
use actix_web::{delete, get, post, web, HttpMessage, HttpRequest, HttpResponse, Responder};
use authentication::data::User;
use crypto::hashids::decode;
use log::{debug, error};
use serde::Deserialize;
use serde_json::json;
use servers::file_system_entry::FileSystemEntries;
use servers::server::Server;
use servers::server_database::ServerDatabase;
use servers::server_filesystem::ServerFilesystem;
use std::error::Error;
use std::fs::File;
use std::io::{Read, Write};
use std::path::PathBuf;

#[post("")]
pub async fn get_server_files(id: web::Path<String>, body: Option<String>, req: HttpRequest) -> Result<impl Responder, Box<dyn Error>> {
    if let Some(user) = req.extensions().get::<User>() {
        let id_number = match decode(&id) {
            Ok(id) => id,
            Err(_) => {
                error!("Invalid id: {}", id);
                return Ok(HttpResponse::BadRequest().json(json!({"error": format!("Invalid id: {}", id)})));
            }
        };
        if id_number.is_empty() {
            return Ok(HttpResponse::BadRequest().json(json!({"error": format!("Invalid id: {}", id)})));
        }
        let server = Server::get_owned_server(id_number[0], user.id as u64)?;
        return Ok(HttpResponse::Ok().json(server.get_files(body.unwrap_or("/".to_string()))));
    }

    Ok(HttpResponse::Unauthorized().json(json!({"error": "Unauthorized"})))
}

#[derive(Debug, Deserialize)]
struct UploadFileMetadata {
    directory: String,
    filename: String,
}

#[derive(Debug, MultipartForm)]
struct UploadForm {
    file: TempFile,
    json: MPJson<UploadFileMetadata>,
}
#[post("/upload")]
pub async fn upload_file_to_server(id: web::Path<String>, MultipartForm(mut payload): MultipartForm<UploadForm>, req: HttpRequest) -> Result<impl Responder, Box<dyn Error>> {
    let directory = payload.json.directory.clone();
    let filename = payload.json.filename.clone();
    let ext = req.extensions();
    let user = ext.get::<User>().ok_or("Unauthorized: User not found")?.to_owned();

    // Decode the ID
    let id_number = decode(&id).map_err(|_| format!("Invalid id: {}", id))?.get(0).cloned().ok_or(format!("Invalid id: {}", id))?;

    // Fetch the server owned by the user using ServerDatabase
    let server = Server::get_owned_server(id_number, user.id as u64).map_err(|_| "Server not found")?;

    if let Some(server_dir) = server.directory.to_str() {
        let path = format!("{}{}{}", server_dir, directory, filename);
        debug!("Uploading file to: {:?}", path);
        debug!("Server Directory: {:?}, Directory: {:?}, Filename: {:?}", server_dir, directory, filename);

        // Create the file
        let mut file = File::create(&path).map_err(|e| {
            error!("Error creating file: {:?}", e);
            "Error creating file"
        })?;

        // Read the temporary file into memory
        let temp_file = payload.file.file.as_file_mut();
        let mut temp_bytes: Vec<u8> = Vec::new();
        temp_file.read_to_end(&mut temp_bytes).map_err(|e| {
            error!("Error reading file: {:?}", e);
            "Error reading file"
        })?;

        // Write the data to the new file
        file.write_all(&temp_bytes).map_err(|e| {
            error!("Error writing file: {:?}", e);
            "Error writing file"
        })?;

        Ok(HttpResponse::Ok().json(json!({"success": "File uploaded"})))
    } else {
        Err("Server directory not specified".into())
    }
}

#[post("")]
pub async fn get_files(body: Option<String>, req: HttpRequest) -> Result<impl Responder, Box<dyn Error>> {
    // Check if the user is authenticated
    req.extensions().get::<User>().ok_or("Unauthorized: User not found")?;

    let path = body.unwrap_or_default(); // Get the requested path or default to empty
    let path = std::env::current_dir()?.join(&path); // Resolve the full path

    let file_system_entries: FileSystemEntries = path.into(); // Convert the path directly into FileSystemEntries

    Ok(HttpResponse::Ok().json(file_system_entries)) // Return the entries in a JSON response
}
#[get("/download/{file}")]
pub async fn download_file(path: web::Path<(String, String)>, req: HttpRequest) -> Result<impl Responder, Box<dyn Error>> {
    let (id, file) = path.into_inner();

    // Decode the URI-encoded file path
    let file = percent_encoding::percent_decode(file.as_bytes())
        .decode_utf8()
        .map_err(|e| {
            error!("Error decoding file path: {:?}", e);
            "Error decoding file path"
        })?
        .to_string();

    let ext = req.extensions();
    // Authenticate the user
    let user = ext.get::<User>().ok_or("Unauthorized: User not found")?;

    // Decode the server ID
    let id_number = decode(&id).map_err(|_| format!("Invalid id: {}", id))?.get(0).cloned().ok_or(format!("Invalid id: {}", id))?;

    // Fetch the server owned by the user
    let server = Server::get_owned_server(id_number, user.id as u64).map_err(|_| "Server not found")?;

    // Verify the server directory and construct the file path
    let path = server.directory.join(PathBuf::from(file));
    debug!("Downloading file: {:?}", path);

    // Open and read the file
    let mut file = File::open(&path).map_err(|e| {
        error!("Error opening file: {:?}", e);
        "Error opening file"
    })?;

    let mut bytes: Vec<u8> = Vec::new();
    file.read_to_end(&mut bytes).map_err(|e| {
        error!("Error reading file: {:?}", e);
        "Error reading file"
    })?;

    Ok(HttpResponse::Ok().content_type("application/octet-stream").body(bytes))
}

#[post("/create/file")]
pub async fn create_file(id: web::Path<String>, body: Option<String>, req: HttpRequest) -> Result<impl Responder, Box<dyn Error>> {
    let ext = req.extensions();
    // Authenticate the user
    let user = ext.get::<User>().ok_or("Unauthorized: User not found")?;

    // Decode the server ID
    let id_number = decode(&id).map_err(|_| format!("Invalid id: {}", id))?.get(0).cloned().ok_or(format!("Invalid id: {}", id))?;

    // Fetch the server owned by the user
    let server = Server::get_owned_server(id_number, user.id as u64).map_err(|_| "Server not found")?;

    // Verify the server directory
    let server_dir = server.directory.to_str().ok_or("Server directory not specified")?;

    // Construct the full file path
    let path = format!("{}{}", server_dir, body.unwrap_or_default());
    debug!("Creating file: {:?}", path);

    // Create the file
    File::create(&path).map_err(|e| {
        error!("Error creating file: {:?}", e);
        "Error creating file"
    })?;

    Ok(HttpResponse::Ok().json(json!({"success": "File created"})))
}

#[post("/create/directory")]
pub async fn create_directory(id: web::Path<String>, body: Option<String>, req: HttpRequest) -> Result<impl Responder, Box<dyn Error>> {
    let ext = req.extensions();
    // Authenticate the user
    let user = ext.get::<User>().ok_or("Unauthorized: User not found")?;

    // Decode the server ID
    let id_number = decode(&id).map_err(|_| format!("Invalid id: {}", id))?.get(0).cloned().ok_or(format!("Invalid id: {}", id))? as u64;

    // Fetch the server owned by the user
    let server = Server::get_owned_server(id_number, user.id as u64).map_err(|_| "Server not found")?;

    // Verify the server directory
    let server_dir = server.directory.to_str().ok_or("Server directory not specified")?;

    // Construct the full directory path
    let path = format!("{}{}", server_dir, body.unwrap_or_default());
    debug!("Creating directory: {:?}", path);

    // Create the directory
    std::fs::create_dir(&path).map_err(|e| {
        error!("Error creating directory: {:?}", e);
        "Error creating directory"
    })?;

    Ok(HttpResponse::Ok().json(json!({"success": "Directory created"})))
}
#[delete("")]
pub async fn delete_path(id: web::Path<String>, body: String, req: HttpRequest) -> Result<impl Responder, Box<dyn Error>> {
    let ext = req.extensions();
    // Authenticate the user
    let user = ext.get::<User>().ok_or("Unauthorized: User not found")?;

    // Decode the server ID
    let id_number = decode(&id).map_err(|_| format!("Invalid id: {}", id))?.get(0).cloned().ok_or(format!("Invalid id: {}", id))? as u64;

    // Fetch the server owned by the user
    let server = Server::get_owned_server(id_number, user.id as u64).map_err(|_| "Server not found")?;

    // Verify the server directory
    let server_dir = server.directory.to_str().ok_or("Server directory not specified")?;

    // Construct the full path
    let path = format!("{}{}", server_dir, body);
    debug!("Deleting path: {:?}", path);

    // Check if the path exists and is a file or directory
    let metadata = std::fs::metadata(&path).map_err(|e| {
        error!("Error accessing path metadata: {:?}", e);
        "Error accessing path metadata"
    })?;

    // Determine whether the path is a file or directory and delete accordingly
    if metadata.is_file() {
        std::fs::remove_file(&path).map_err(|e| {
            error!("Error deleting file: {:?}", e);
            "Error deleting file"
        })?;
        Ok(HttpResponse::Ok().json(json!({"success": "File deleted"})))
    } else {
        std::fs::remove_dir_all(&path).map_err(|e| {
            error!("Error deleting directory: {:?}", e);
            "Error deleting directory"
        })?;
        Ok(HttpResponse::Ok().json(json!({"success": "Path deleted"})))
    }
}
