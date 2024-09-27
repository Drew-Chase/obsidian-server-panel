use log::error;
use std::error::Error;
use std::fs::File;
use std::io::{Read, Write};
use std::path::Path;
use walkdir::{DirEntry, WalkDir};
use zip::write::SimpleFileOptions;
use zip::{CompressionMethod, ZipWriter};

pub fn test(){
	archive_directory("/var/games", "output.zip", &|entry| {
		entry.ends_with(".txt")
	}).unwrap();
}

pub fn archive_directory<T: AsRef<Path>>(directory: T, output_file: T, filter: &dyn Fn(T)->bool) -> Result<(), Box<dyn Error>> {
	let file = File::create(output_file)?;
	let mut zip = ZipWriter::new(file);
	let options = SimpleFileOptions::default().compression_method(CompressionMethod::Stored);
	let all_files = WalkDir::new(&directory)
		.into_iter()
		.map(|e| e.unwrap())
		.collect::<Vec<_>>();
	for entry in all_files {
		if entry.file_type().is_dir() {
			continue;
		}
		let relative_path = entry.path().strip_prefix(&directory).unwrap();
		zip.start_file_from_path(relative_path, options)
		   .map_err(|e| format!("Failed to start file from path: {}", e))?;
		let mut file = match File::open(entry.path()) {
			Ok(f) => f,
			Err(e) => {
				error!("Failed to open file: {}", e);
				return Err(e.into());
			}
		};
		let mut file_contents = vec![];
		file.read_to_end(&mut file_contents)
		    .map_err(|e| format!("Failed to read file: {}", e))?;
		zip.write(&file_contents)
		   .map_err(|e| format!("Failed to write file: {}", e))?;
	}
	zip.finish()?;
	Ok(())
}
