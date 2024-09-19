use log::{debug, info};
use std::collections::HashMap;
use std::io::Write;

pub struct Properties {
	pub file: String,
	pub items: HashMap<String, String>
}

impl Properties {
	pub fn new(file: &str) -> Result<Self, String> {
		let map = match Properties::load(file) {
			Ok(map) => map,
			Err(e) => return Err(format!("Error loading properties file: {}", e))
		};
		Ok(Self { file: file.to_string(), items: map })
	}


	pub fn get(&self, key: &str) -> Option<&String> {
		self.items.get(key)
	}

	pub fn set(&mut self, key: &str, value: &str) {
		let result = self.items.insert(key.to_string(), value.to_string());
		debug!("Setting property {} from {} to {}", key, result.unwrap_or("null".to_string()) , value);
	}

	
	pub fn write(&self) -> Result<(), String> {
		let mut file = match std::fs::File::create(&self.file) {
			Ok(file) => file,
			Err(e) => return Err(format!("Error creating properties file: {}", e))
		};
		for (key, value) in &self.items {
			let line = format!("{}={}\n", key, value);
			match file.write_all(line.as_bytes()) {
				Ok(_) => (),
				Err(e) => return Err(format!("Error writing to properties file: {}", e))
			}
		}
		Ok(())
	}

	pub fn load(file: &str) -> Result<HashMap<String, String>, String>
	{
		info!("Loading properties file: {}", file);
		let mut map = HashMap::new();
		let file = std::fs::read_to_string(file).unwrap();
		for line in file.lines() {
			let parts: Vec<&str> = line.split('=').collect();
			if parts.len() == 2 {
				map.insert(parts[0].to_string(), parts[1].to_string());
			}
		}
		debug!("Loaded {} properties", map.len());
		Ok(map)
	}
}