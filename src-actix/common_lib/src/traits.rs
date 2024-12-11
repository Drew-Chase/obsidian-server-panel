use std::path::PathBuf;

pub trait TransformPath {
    fn normalize(&self) -> PathBuf;
    fn to_absolute(&self) -> Result<PathBuf, Box<dyn std::error::Error>>;
    fn to_relative(&self) -> Result<PathBuf, Box<dyn std::error::Error>>;
}
impl TransformPath for PathBuf {
    fn normalize(&self) -> PathBuf {
        let path = self
            .canonicalize()
            .expect(format!("Failed to canonicalize path {:?}", self).as_str());
        let path_str = path.to_str().expect("Failed to convert path to string");

        // If the path starts with the extended-length path prefix, remove it.
        let normalized_path = if path_str.starts_with(r"\\?\") {
            &path_str[4..]
        } else {
            path_str
        };

        PathBuf::from(normalized_path)
    }
    fn to_absolute(&self) -> Result<PathBuf, Box<dyn std::error::Error>> {
        let current_exe = std::env::current_exe()?;
        let current_dir = current_exe.parent();
        if let Some(dir) = current_dir {
            let abs_path = dir.join(self);
            Ok(abs_path)
        } else {
            Err("Failed to get current directory".into())
        }
    }

    fn to_relative(&self) -> Result<PathBuf, Box<dyn std::error::Error>> {
        let current_exe = std::env::current_exe()?;
        let current_dir = current_exe.parent();
        if let Some(dir) = current_dir {
            let rel_path = self.strip_prefix(dir)?;
            Ok(rel_path.to_path_buf())
        } else {
            Err("Failed to get current directory".into())
        }
    }
}
