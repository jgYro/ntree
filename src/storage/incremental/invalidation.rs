use std::collections::{HashMap, HashSet};
use std::path::PathBuf;
use crate::core::NTreeError;
use crate::storage::{SymbolId, FileRecord, ContentHash};
use super::{IncrementalCache, ReverseDependencyIndex, FuncSummary};

/// Invalidation engine for incremental analysis.
#[derive(Debug)]
pub struct InvalidationEngine {
    /// Incremental cache
    cache: IncrementalCache,
    /// Reverse dependency tracking
    reverse_deps: ReverseDependencyIndex,
}

impl InvalidationEngine {
    /// Create new invalidation engine.
    pub fn new() -> Self {
        InvalidationEngine {
            cache: IncrementalCache::new(),
            reverse_deps: ReverseDependencyIndex::new(),
        }
    }

    /// Process file changes and determine what needs recomputation.
    pub fn process_file_changes(&mut self, file_records: &[FileRecord]) -> Result<InvalidationResult, NTreeError> {
        let mut changed_files = Vec::new();
        let mut affected_functions = HashSet::new();

        // Detect changed files
        for file_record in file_records {
            if self.cache.has_file_changed(&file_record.path, &file_record.content_hash) {
                changed_files.push(file_record.path.clone());
                self.cache.mark_file_dirty(file_record.path.clone(), file_record.content_hash.clone());

                // Get functions in this file
                let file_functions = self.cache.get_affected_functions(&file_record.path);
                affected_functions.extend(file_functions);
            }
        }

        // Compute transitive invalidation
        let mut functions_to_recompute = HashSet::new();
        for function in &affected_functions {
            let invalidation_set = self.reverse_deps.get_invalidation_set(function)?;
            functions_to_recompute.extend(invalidation_set);
        }

        Ok(InvalidationResult {
            changed_files,
            affected_functions,
            functions_to_recompute,
            cache_version: self.cache.version(),
        })
    }

    /// Mark file as reprocessed.
    pub fn mark_file_processed(&mut self, file_path: PathBuf, new_hash: ContentHash) {
        self.cache.update_file_hash(file_path, new_hash);
    }

    /// Add function summary and update dependencies.
    pub fn add_function_summary(&mut self, summary: FuncSummary) {
        // Update reverse dependency index
        for callee in summary.get_callees() {
            self.reverse_deps.add_call(summary.sym_id.clone(), callee.clone());
        }

        // Add to cache
        self.cache.add_function_summary(summary);
    }

    /// Remove function from cache and dependencies.
    pub fn remove_function(&mut self, sym_id: &SymbolId) {
        // Remove from reverse deps
        if let Some(summary) = self.cache.get_function_summary(sym_id) {
            for callee in summary.get_callees() {
                self.reverse_deps.remove_call(sym_id, callee);
            }
        }

        // Remove from cache would need additional cache method
        // For now, this is a placeholder
    }

    /// Get function summary if cached and valid.
    pub fn get_valid_summary(&self, sym_id: &SymbolId, min_version: u64) -> Option<&FuncSummary> {
        match self.cache.get_function_summary(sym_id) {
            Some(summary) if summary.is_newer_than(min_version) => Some(summary),
            _ => None,
        }
    }

    /// Rebuild reverse dependency index from current summaries.
    pub fn rebuild_reverse_deps(&mut self) {
        self.reverse_deps.rebuild_from_summaries(self.cache.get_all_summaries());
    }

    /// Get cache statistics.
    pub fn get_cache_stats(&self) -> super::cache::CacheStats {
        self.cache.get_stats()
    }

    /// Get dependency statistics.
    pub fn get_dependency_stats(&self) -> super::reverse_deps::DependencyStats {
        self.reverse_deps.get_stats()
    }

    /// Check if any files need reprocessing.
    pub fn has_dirty_files(&self) -> bool {
        !self.cache.get_dirty_files().is_empty()
    }

    /// Clear all cache and dependency data.
    pub fn clear(&mut self) {
        self.cache.clear();
        self.reverse_deps.clear();
    }
}

/// Result of invalidation analysis.
#[derive(Debug)]
pub struct InvalidationResult {
    /// Files that have changed
    pub changed_files: Vec<PathBuf>,
    /// Functions directly in changed files
    pub affected_functions: HashSet<SymbolId>,
    /// Functions that need recomputation (including transitive dependencies)
    pub functions_to_recompute: HashSet<SymbolId>,
    /// Current cache version
    pub cache_version: u64,
}

impl InvalidationResult {
    /// Check if any invalidation is needed.
    pub fn needs_recomputation(&self) -> bool {
        !self.functions_to_recompute.is_empty()
    }

    /// Get count of functions that need recomputation.
    pub fn recomputation_count(&self) -> usize {
        self.functions_to_recompute.len()
    }

    /// Check if specific function needs recomputation.
    pub fn function_needs_recompute(&self, sym_id: &SymbolId) -> bool {
        self.functions_to_recompute.contains(sym_id)
    }

    /// Alias for needs_recomputation.
    pub fn needs_work(&self) -> bool {
        self.needs_recomputation()
    }

    /// Get invalidation statistics.
    pub fn get_stats(&self) -> InvalidationStats {
        InvalidationStats {
            changed_files: self.changed_files.len(),
            affected_functions: self.affected_functions.len(),
            functions_to_recompute: self.functions_to_recompute.len(),
            cache_version: self.cache_version,
        }
    }
}

/// Statistics for invalidation analysis.
#[derive(Debug, Clone)]
pub struct InvalidationStats {
    pub changed_files: usize,
    pub affected_functions: usize,
    pub functions_to_recompute: usize,
    pub cache_version: u64,
}