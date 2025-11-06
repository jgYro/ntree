use std::path::{Path, PathBuf};
use std::collections::HashMap;
use jwalk::{WalkDir, DirEntry};
use crate::core::NTreeError;
use super::file_record::FileRecord;

/// File walker that discovers source files and respects ignore patterns.
pub struct FileWalker {
    root: PathBuf,
    supported_extensions: Vec<String>,
    ignore_patterns: Vec<String>,
}

impl FileWalker {
    /// Create a new file walker for the given root directory.
    pub fn new<P: AsRef<Path>>(root: P) -> Self {
        FileWalker {
            root: root.as_ref().to_path_buf(),
            supported_extensions: Self::default_extensions(),
            ignore_patterns: Self::default_ignore_patterns(),
        }
    }

    /// Get default supported file extensions.
    fn default_extensions() -> Vec<String> {
        vec![
            "rs".to_string(),
            "py".to_string(),
            "js".to_string(),
            "mjs".to_string(),
            "ts".to_string(),
            "java".to_string(),
            "c".to_string(),
            "h".to_string(),
            "cpp".to_string(),
            "cc".to_string(),
            "cxx".to_string(),
            "hpp".to_string(),
            "hxx".to_string(),
        ]
    }

    /// Get default ignore patterns.
    fn default_ignore_patterns() -> Vec<String> {
        vec![
            "target".to_string(),
            "node_modules".to_string(),
            ".git".to_string(),
            "build".to_string(),
            "dist".to_string(),
            "__pycache__".to_string(),
            ".pytest_cache".to_string(),
            "coverage".to_string(),
        ]
    }

    /// Walk the directory and discover source files.
    pub fn discover_files(&self) -> Result<Vec<FileRecord>, NTreeError> {
        let mut file_records = Vec::new();

        for entry in WalkDir::new(&self.root) {
            match entry {
                Ok(entry) => {
                    if let Some(record) = self.process_entry(entry)? {
                        file_records.push(record);
                    }
                }
                Err(e) => {
                    eprintln!("Warning: Failed to process directory entry: {}", e);
                    continue;
                }
            }
        }

        Ok(file_records)
    }

    /// Discover files grouped by language.
    pub fn discover_by_language(&self) -> Result<HashMap<String, Vec<FileRecord>>, NTreeError> {
        let files = self.discover_files()?;
        let mut by_language = HashMap::new();

        for file in files {
            by_language
                .entry(file.language.clone())
                .or_insert_with(Vec::new)
                .push(file);
        }

        Ok(by_language)
    }

    /// Process a single directory entry.
    fn process_entry(&self, entry: DirEntry<((), ())>) -> Result<Option<FileRecord>, NTreeError> {
        let path = entry.path();

        // Skip directories
        if path.is_dir() {
            return Ok(None);
        }

        // Check if path should be ignored
        if self.should_ignore_path(&path) {
            return Ok(None);
        }

        // Check if extension is supported
        if !self.has_supported_extension(&path) {
            return Ok(None);
        }

        // Read file content
        let content = match std::fs::read_to_string(&path) {
            Ok(content) => content,
            Err(e) => {
                eprintln!("Warning: Failed to read {}: {}", path.display(), e);
                return Ok(None);
            }
        };

        // Create file record
        match FileRecord::new(path, &content) {
            Ok(record) => Ok(Some(record)),
            Err(_) => Ok(None), // Skip files we can't process
        }
    }

    /// Check if path should be ignored based on ignore patterns.
    fn should_ignore_path(&self, path: &Path) -> bool {
        let path_str = path.to_string_lossy();
        for pattern in &self.ignore_patterns {
            if path_str.contains(pattern) {
                return true;
            }
        }
        false
    }

    /// Check if path has a supported file extension.
    fn has_supported_extension(&self, path: &Path) -> bool {
        match path.extension().and_then(|ext| ext.to_str()) {
            Some(ext) => self.supported_extensions.contains(&ext.to_string()),
            None => false,
        }
    }
}