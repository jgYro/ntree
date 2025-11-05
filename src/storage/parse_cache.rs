use std::collections::HashMap;
use std::path::PathBuf;
use serde::{Deserialize, Serialize};
use super::file_record::ContentHash;

/// Version of the extractor for cache invalidation.
pub const EXTRACTOR_VERSION: &str = "1.0.0";

/// Cache key for parsed results.
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq, Hash)]
pub struct CacheKey {
    /// File path
    pub path: PathBuf,
    /// Content hash for change detection
    pub content_hash: ContentHash,
    /// Extractor version for compatibility
    pub extractor_version: String,
}

impl CacheKey {
    /// Create a new cache key.
    pub fn new(path: PathBuf, content_hash: ContentHash) -> Self {
        CacheKey {
            path,
            content_hash,
            extractor_version: EXTRACTOR_VERSION.to_string(),
        }
    }
}

/// Cached parse results for a file.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CachedParseResult {
    /// Function analysis results
    pub functions: Vec<crate::models::FunctionSpan>,
    /// CFG analysis results
    pub cfgs: Vec<crate::api::CfgResult>,
    /// Complexity analysis results
    pub complexity: Vec<crate::analyzers::ComplexityResult>,
    /// Parse timestamp
    pub parsed_at: u64,
}

/// In-memory parse cache for fast repeated analysis.
#[derive(Debug)]
pub struct ParseCache {
    cache: HashMap<CacheKey, CachedParseResult>,
    max_entries: usize,
}

impl ParseCache {
    /// Create a new parse cache.
    pub fn new(max_entries: usize) -> Self {
        ParseCache {
            cache: HashMap::new(),
            max_entries,
        }
    }

    /// Check if results are cached for the given key.
    pub fn get(&self, key: &CacheKey) -> Option<&CachedParseResult> {
        self.cache.get(key)
    }

    /// Store parse results in cache.
    pub fn put(&mut self, key: CacheKey, result: CachedParseResult) {
        if self.cache.len() >= self.max_entries {
            // Simple LRU: remove one entry
            if let Some(oldest_key) = self.cache.keys().next().cloned() {
                self.cache.remove(&oldest_key);
            }
        }
        self.cache.insert(key, result);
    }

    /// Check if cache contains results for key.
    pub fn contains(&self, key: &CacheKey) -> bool {
        self.cache.contains_key(key)
    }

    /// Clear all cached entries.
    pub fn clear(&mut self) {
        self.cache.clear();
    }

    /// Get cache statistics.
    pub fn stats(&self) -> CacheStats {
        CacheStats {
            entries: self.cache.len(),
            max_entries: self.max_entries,
            hit_rate: 0.0, // Could be tracked with counters
        }
    }
}

/// Cache performance statistics.
#[derive(Debug, Clone)]
pub struct CacheStats {
    pub entries: usize,
    pub max_entries: usize,
    pub hit_rate: f64,
}

impl Default for ParseCache {
    fn default() -> Self {
        Self::new(1000) // Default to 1000 entries
    }
}