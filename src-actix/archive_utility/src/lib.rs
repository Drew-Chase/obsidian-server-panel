use rayon::prelude::*;
use std::fs::File;
use std::io::{self, BufReader, BufWriter, Read, Write};
use std::path::Path;
use walkdir::WalkDir;
use zip::write::SimpleFileOptions;
use zip::{CompressionMethod, ZipWriter};

// Custom error type to better handle and propagate errors.
#[derive(Debug)]
pub enum ArchiveError {
    Io(io::Error),
    Zip(zip::result::ZipError),
    WalkDir(walkdir::Error),
    PathPrefix(std::path::StripPrefixError),
}

impl From<io::Error> for ArchiveError {
    fn from(err: io::Error) -> ArchiveError {
        ArchiveError::Io(err)
    }
}

impl From<zip::result::ZipError> for ArchiveError {
    fn from(err: zip::result::ZipError) -> ArchiveError {
        ArchiveError::Zip(err)
    }
}

impl From<walkdir::Error> for ArchiveError {
    fn from(err: walkdir::Error) -> ArchiveError {
        ArchiveError::WalkDir(err)
    }
}

impl From<std::path::StripPrefixError> for ArchiveError {
    fn from(err: std::path::StripPrefixError) -> ArchiveError {
        ArchiveError::PathPrefix(err)
    }
}

// Struct to hold file information.
struct FileEntry {
    relative_path: String,
    buffer: Vec<u8>,
}

/// Archives a directory into a zip file.
///
/// # Arguments
///
/// * `directory` - The directory to archive.
/// * `output_file` - The path to the output zip file.
/// * `filter` - A closure that takes a `Path` and returns a `bool` indicating whether the path should be included in the archive.
///
/// # Errors
///
/// Returns an error if the operation fails.
///
/// # Example
///
/// ```no_run
/// use archive_utility::archive_directory;
/// use std::path::Path;
///
/// let directory = Path::new("/path/to/directory");
/// let output_file = Path::new("/path/to/output.zip");
///
/// archive_directory(directory, output_file, &|path| {
///    // Exclude files with .tmp extension
///   !path.ends_with(".tmp")
/// }).unwrap();
/// ```
pub fn archive_directory(
    directory: impl AsRef<Path>,
    output_file: impl AsRef<Path>,
    filter: &dyn Fn(&Path) -> bool,
) -> Result<(), ArchiveError> {
    // Convert the input arguments to `Path` references.
    let directory = directory.as_ref();
    let output_file = output_file.as_ref();

    // Gather all entries that pass the filter.
    let all_entries: Vec<_> = WalkDir::new(directory)
        .into_iter()
        .filter_map(|e| e.ok())
        .filter(|e| !e.file_type().is_dir() && filter(e.path()))
        .collect();

    // Process files in parallel.
    let file_entries: Result<Vec<_>, ArchiveError> = all_entries
        .par_iter()
        .map(|entry| {
            let path = entry.path();
            let relative_path = path.strip_prefix(directory)?.to_string_lossy().to_string();

            // Read file contents.
            let mut buffer = Vec::new();
            let input_file = File::open(path)?;
            let mut reader = BufReader::new(input_file);
            reader.read_to_end(&mut buffer)?;

            Ok(FileEntry { relative_path, buffer })
        })
        .collect();

    // Collect all file entries from parallel processing.
    let file_entries = file_entries?;

    // Create a new zip file.
    let file = File::create(output_file)?;
    let writer = BufWriter::new(file);
    let mut zip = ZipWriter::new(writer);
    let options = SimpleFileOptions::default().compression_method(CompressionMethod::Deflated);

    for entry in file_entries {
        // Start the new file in the zip archive.
        zip.start_file(&entry.relative_path, options)?;
        zip.write_all(&entry.buffer)?;
    }

    // Finish writing to the zip file.
    zip.finish()?;
    Ok(())
}
