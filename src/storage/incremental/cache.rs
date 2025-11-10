use std::collections::HashMap;
use std::path::PathBuf;
use crate::core::NTreeError;
use crate::storage::{SymbolId, ContentHash, FileRecord};
use super::func_summary::FuncSummary;

/// Incremental cache for fast recomputation after edits.
#[derive(Debug)]
pub struct IncrementalCache {
    /// File content hashes for change detection
    file_hashes: HashMap<PathBuf, ContentHash>,
    /// Function summaries by symbol ID
    function_summaries: HashMap<SymbolId, FuncSummary>,
    /// Files to symbols mapping
    file_symbols: HashMap<PathBuf, Vec<SymbolId>>,
    /// Cache version counter
    cache_version: u64,
    /// Files marked as dirty/changed
    dirty_files: HashMap<PathBuf, ContentHash>,
}

impl IncrementalCache {
    /// Create new incremental cache.
    pub fn new() -> Self {
        IncrementalCache {
            file_hashes: HashMap::new(),
            function_summaries: HashMap::new(),
            file_symbols: HashMap::new(),
            cache_version: 0,
            dirty_files: HashMap::new(),
        }
    }

    /// Check if file has changed based on hash.
    pub fn has_file_changed(&self, file_path: &PathBuf, current_hash: &ContentHash) -> bool {
        match self.file_hashes.get(file_path) {
            Some(cached_hash) => cached_hash != current_hash,
            None => true, // New file
        }
    }

    /// Update file hash and mark as processed.
    pub fn update_file_hash(&mut self, file_path: PathBuf, new_hash: ContentHash) {
        self.dirty_files.remove(&file_path);
        self.file_hashes.insert(file_path, new_hash);
    }

    /// Mark file as dirty/changed.
    pub fn mark_file_dirty(&mut self, file_path: PathBuf, new_hash: ContentHash) {
        self.dirty_files.insert(file_path, new_hash);
        self.cache_version += 1;
    }

    /// Get all dirty files that need reprocessing.
    pub fn get_dirty_files(&self) -> Vec<PathBuf> {
        self.dirty_files.keys().cloned().collect()
    }

    /// Add function summary to cache.
    pub fn add_function_summary(&mut self, summary: FuncSummary) {
        let file_path = self.get_file_for_symbol(&summary.sym_id);
        if let Some(path) = file_path {
            self.file_symbols
                .entry(path)
                .or_insert_with(Vec::new)
                .push(summary.sym_id.clone());
        }
        self.function_summaries.insert(summary.sym_id.clone(), summary);
    }

    /// Get function summary by symbol ID.
    pub fn get_function_summary(&self, sym_id: &SymbolId) -> Option<&FuncSummary> {
        self.function_summaries.get(sym_id)
    }

    /// Remove function summaries for a file.
    pub fn invalidate_file_summaries(&mut self, file_path: &PathBuf) {
        if let Some(symbols) = self.file_symbols.remove(file_path) {
            for sym_id in symbols {
                self.function_summaries.remove(&sym_id);
            }
        }
    }

    /// Get functions affected by file changes.
    pub fn get_affected_functions(&self, file_path: &PathBuf) -> Vec<SymbolId> {
        self.file_symbols.get(file_path).cloned().unwrap_or_default()
    }

    /// Get all function summaries.
    pub fn get_all_summaries(&self) -> impl Iterator<Item = &FuncSummary> {
        self.function_summaries.values()
    }

    /// Check if cache is up to date for file.
    pub fn is_file_cached(&self, file_record: &FileRecord) -> bool {
        match self.file_hashes.get(&file_record.path) {
            Some(cached_hash) => cached_hash == &file_record.content_hash,
            None => false,
        }
    }

    /// Get cache statistics.
    pub fn get_stats(&self) -> CacheStats {
        CacheStats {
            total_files: self.file_hashes.len(),
            total_functions: self.function_summaries.len(),
            dirty_files: self.dirty_files.len(),
            cache_version: self.cache_version,
        }
    }

    /// Simple file-to-symbol mapping (placeholder implementation).
    fn get_file_for_symbol(&self, _sym_id: &SymbolId) -> Option<PathBuf> {
        // In a real implementation, this would extract file path from symbol ID
        None
    }

    /// Clear all cache data.
    pub fn clear(&mut self) {
        self.file_hashes.clear();
        self.function_summaries.clear();
        self.file_symbols.clear();
        self.dirty_files.clear();
        self.cache_version = 0;
    }

    /// Get current cache version.
    pub fn version(&self) -> u64 {
        self.cache_version
    }
}

/// Cache statistics.
#[derive(Debug, Clone)]
pub struct CacheStats {
    pub total_files: usize,
    pub total_functions: usize,
    pub dirty_files: usize,
    pub cache_version: u64,
}