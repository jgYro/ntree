use std::path::{Path, PathBuf};
use serde::{Deserialize, Serialize};
use sha2::{Sha256, Digest};
use crate::language::SupportedLanguage;
use crate::core::NTreeError;

/// Content hash for change detection and cache invalidation.
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq, Hash)]
pub struct ContentHash(String);

impl ContentHash {
    /// Create hash from file content.
    pub fn from_content(content: &str) -> Self {
        let mut hasher = Sha256::new();
        hasher.update(content.as_bytes());
        let result = hasher.finalize();
        ContentHash(format!("{:x}", result))
    }

    /// Get hash as string.
    pub fn as_str(&self) -> &str {
        &self.0
    }
}

/// Record of a source file with metadata for caching and analysis.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct FileRecord {
    /// Full path to the source file
    pub path: PathBuf,
    /// Detected programming language
    pub language: String,
    /// Hash of file content for change detection
    pub content_hash: ContentHash,
    /// File size in bytes
    pub size: u64,
    /// Last modification time (Unix timestamp)
    pub modified: u64,
}

impl FileRecord {
    /// Create a new file record from path and content.
    pub fn new<P: AsRef<Path>>(path: P, content: &str) -> Result<Self, NTreeError> {
        let path_buf = path.as_ref().to_path_buf();

        // Detect language
        let language = match SupportedLanguage::from_path(&path_buf) {
            Ok(lang) => lang.name().to_string(),
            Err(_) => "unknown".to_string(),
        };

        // Generate content hash
        let content_hash = ContentHash::from_content(content);

        // Get file metadata
        let metadata = match std::fs::metadata(&path_buf) {
            Ok(meta) => meta,
            Err(e) => return Err(NTreeError::IoError(e)),
        };

        let size = metadata.len();
        let modified = match metadata.modified() {
            Ok(time) => match time.duration_since(std::time::UNIX_EPOCH) {
                Ok(duration) => duration.as_secs(),
                Err(_) => 0,
            },
            Err(_) => 0,
        };

        Ok(FileRecord {
            path: path_buf,
            language,
            content_hash,
            size,
            modified,
        })
    }

    /// Check if file has changed since this record was created.
    pub fn has_changed(&self, new_content: &str) -> bool {
        let new_hash = ContentHash::from_content(new_content);
        self.content_hash != new_hash
    }

    /// Get file extension.
    pub fn extension(&self) -> Option<String> {
        self.path
            .extension()
            .and_then(|ext| ext.to_str())
            .map(|s| s.to_string())
    }

    /// Get relative path from a base directory.
    pub fn relative_path<P: AsRef<Path>>(&self, base: P) -> Option<PathBuf> {
        self.path.strip_prefix(base).ok().map(|p| p.to_path_buf())
    }
}