use std::path::Path;
use std::collections::HashMap;
use crate::core::NTreeError;
use crate::storage::{
    InvalidationEngine, ExternalLibraryHandler, ClassHierarchyAnalyzer,
    RapidTypeAnalyzer, FileRecord, Resolution
};
use super::{InterproceduralOptions, InterproceduralResult};

/// Advanced incremental analysis with CHA/RTA and external library support.
#[derive(Debug)]
pub struct IncrementalAnalyzer {
    /// Invalidation engine for cache management
    invalidation_engine: InvalidationEngine,
    /// External library handler
    library_handler: ExternalLibraryHandler,
    /// Class hierarchy analyzer
    cha_analyzer: ClassHierarchyAnalyzer,
    /// Rapid type analyzer
    rta_analyzer: Option<RapidTypeAnalyzer>,
}

impl IncrementalAnalyzer {
    /// Create new incremental analyzer.
    pub fn new() -> Self {
        let cha_analyzer = ClassHierarchyAnalyzer::new();
        let library_handler = ExternalLibraryHandler::new();

        IncrementalAnalyzer {
            invalidation_engine: InvalidationEngine::new(),
            library_handler,
            cha_analyzer,
            rta_analyzer: None,
        }
    }

    /// Enable RTA analysis.
    pub fn enable_rta(&mut self) {
        let cha = ClassHierarchyAnalyzer::new(); // Fresh instance for RTA
        self.rta_analyzer = Some(RapidTypeAnalyzer::new(cha));
    }

    /// Perform incremental interprocedural analysis.
    pub fn analyze_incremental<P: AsRef<Path>>(
        &mut self,
        workspace_path: P,
        options: IncrementalAnalysisOptions,
    ) -> Result<IncrementalResult, NTreeError> {
        // Get file records for change detection
        let file_records = self.get_file_records(&workspace_path)?;

        // Process file changes and determine what needs recomputation
        let invalidation_result = self.invalidation_engine.process_file_changes(&file_records)?;

        // If no changes, return cached results
        if !invalidation_result.needs_work() && !options.force_recompute {
            return Ok(IncrementalResult {
                invalidation_stats: invalidation_result.get_stats(),
                interprocedural_result: None,
                resolutions: HashMap::new(),
                cache_hit: true,
            });
        }

        // Perform full or partial recomputation
        let interprocedural_result = if options.full_recompute || invalidation_result.changed_files.len() > 10 {
            // Full recomputation
            self.perform_full_analysis(&workspace_path, options.interprocedural_options.clone())?
        } else {
            // Incremental recomputation
            self.perform_incremental_analysis(&workspace_path, &invalidation_result, options.interprocedural_options.clone())?
        };

        // Generate call resolutions using CHA/RTA
        let mut resolutions = HashMap::new();
        if options.enable_cha || options.enable_rta {
            resolutions = self.generate_call_resolutions(&interprocedural_result, &options)?;
        }

        // Update cache with new results
        self.update_cache_from_results(&interprocedural_result, &file_records)?;

        Ok(IncrementalResult {
            invalidation_stats: invalidation_result.get_stats(),
            interprocedural_result: Some(interprocedural_result),
            resolutions,
            cache_hit: false,
        })
    }

    /// Perform full analysis (fallback or initial run).
    fn perform_full_analysis<P: AsRef<Path>>(
        &self,
        workspace_path: P,
        options: InterproceduralOptions,
    ) -> Result<InterproceduralResult, NTreeError> {
        super::analyze_interprocedural_cfg(workspace_path, options)
    }

    /// Perform incremental analysis on changed functions only.
    fn perform_incremental_analysis<P: AsRef<Path>>(
        &self,
        _workspace_path: P,
        _invalidation_result: &crate::storage::incremental::invalidation::InvalidationResult,
        _options: InterproceduralOptions,
    ) -> Result<InterproceduralResult, NTreeError> {
        // Placeholder: would recompute only affected functions
        Err(NTreeError::InvalidInput("Incremental analysis not yet implemented".to_string()))
    }

    /// Generate call resolutions using CHA/RTA.
    fn generate_call_resolutions(
        &mut self,
        _interprocedural_result: &InterproceduralResult,
        _options: &IncrementalAnalysisOptions,
    ) -> Result<HashMap<usize, Resolution>, NTreeError> {
        // Placeholder: would generate resolutions for call sites
        Ok(HashMap::new())
    }

    /// Update cache with analysis results.
    fn update_cache_from_results(
        &mut self,
        _interprocedural_result: &InterproceduralResult,
        file_records: &[FileRecord],
    ) -> Result<(), NTreeError> {
        // Mark files as processed
        for file_record in file_records {
            self.invalidation_engine.mark_file_processed(
                file_record.path.clone(),
                file_record.content_hash.clone()
            );
        }
        Ok(())
    }

    /// Get file records for workspace.
    fn get_file_records<P: AsRef<Path>>(&self, _workspace_path: P) -> Result<Vec<FileRecord>, NTreeError> {
        // Placeholder: would scan workspace for files
        Ok(Vec::new())
    }

    /// Get cache statistics.
    pub fn get_cache_stats(&self) -> crate::storage::incremental::cache::CacheStats {
        self.invalidation_engine.get_cache_stats()
    }

    /// Get dependency statistics.
    pub fn get_dependency_stats(&self) -> crate::storage::incremental::reverse_deps::DependencyStats {
        self.invalidation_engine.get_dependency_stats()
    }

    /// Get external library statistics.
    pub fn get_library_stats(&self) -> crate::storage::external::library_handler::LibraryStats {
        self.library_handler.get_stats()
    }
}

/// Configuration for incremental analysis.
#[derive(Debug, Clone)]
pub struct IncrementalAnalysisOptions {
    /// Interprocedural analysis options
    pub interprocedural_options: InterproceduralOptions,
    /// Force full recomputation
    pub force_recompute: bool,
    /// Prefer full over incremental if many files changed
    pub full_recompute: bool,
    /// Enable Class Hierarchy Analysis
    pub enable_cha: bool,
    /// Enable Rapid Type Analysis
    pub enable_rta: bool,
    /// Enable external library analysis
    pub enable_external_analysis: bool,
}

impl Default for IncrementalAnalysisOptions {
    fn default() -> Self {
        IncrementalAnalysisOptions {
            interprocedural_options: InterproceduralOptions::default(),
            force_recompute: false,
            full_recompute: false,
            enable_cha: false,
            enable_rta: false,
            enable_external_analysis: true,
        }
    }
}

/// Result of incremental analysis.
#[derive(Debug)]
pub struct IncrementalResult {
    /// Invalidation statistics
    pub invalidation_stats: crate::storage::incremental::invalidation::InvalidationStats,
    /// Interprocedural analysis result (None if cache hit)
    pub interprocedural_result: Option<InterproceduralResult>,
    /// Call site resolutions (CHA/RTA results)
    pub resolutions: HashMap<usize, Resolution>,
    /// Whether this was a cache hit
    pub cache_hit: bool,
}

impl IncrementalResult {
    /// Check if analysis was performed (not cached).
    pub fn was_recomputed(&self) -> bool {
        !self.cache_hit
    }

    /// Get performance metrics.
    pub fn get_performance_metrics(&self) -> PerformanceMetrics {
        PerformanceMetrics {
            cache_hit: self.cache_hit,
            functions_recomputed: self.invalidation_stats.functions_to_recompute,
            files_changed: self.invalidation_stats.changed_files,
            resolutions_generated: self.resolutions.len(),
        }
    }
}

/// Performance metrics for incremental analysis.
#[derive(Debug, Clone)]
pub struct PerformanceMetrics {
    pub cache_hit: bool,
    pub functions_recomputed: usize,
    pub files_changed: usize,
    pub resolutions_generated: usize,
}