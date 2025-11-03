use std::fs;
use std::io;
use std::path::Path;

/// Reads a file and returns its contents as a string.
///
/// # Arguments
/// * `path` - Path to the file to read
///
/// # Returns
/// * `Ok(String)` - File contents
/// * `Err(io::Error)` - If file cannot be read
pub fn read_file<P: AsRef<Path>>(path: P) -> io::Result<String> {
    fs::read_to_string(path)
}