use super::unified_analysis::AnalysisResult;
use crate::api::analysis::AnalysisOptions;
use crate::core::NTreeError;
use std::path::{Path, PathBuf};

/// Builder for source code analysis with fluent configuration API.
#[derive(Debug, Clone)]
pub struct SourceCode {
    path: PathBuf,
    is_workspace: bool,
    options: AnalysisOptions,
    // Advanced analysis options (hidden from public API)
    enable_incremental: bool,
    enable_cha: bool,
    enable_rta: bool,
    enable_external_libs: bool,
    enable_deep_external_calls: bool,
}

impl SourceCode {
    /// Create a new source code analyzer for file or directory.
    pub fn new<P: AsRef<Path>>(path: P) -> Result<Self, NTreeError> {
        let path_buf = path.as_ref().to_path_buf();

        if !path_buf.exists() {
            return Err(NTreeError::ParseError(format!(
                "Path does not exist: {}",
                path_buf.display()
            )));
        }

        // Auto-detect if this is a workspace (directory) or single file
        let is_workspace = path_buf.is_dir();

        Ok(SourceCode {
            path: path_buf,
            is_workspace,
            options: AnalysisOptions::default(),
            enable_incremental: false,
            enable_cha: false,
            enable_rta: false,
            enable_external_libs: false,
            enable_deep_external_calls: false,
        })
    }

    /// Configure complexity analysis (enabled by default).
    pub fn with_complexity_analysis(mut self, enabled: bool) -> Self {
        self.options.complexity_analysis = enabled;
        self
    }

    /// Configure CFG generation (enabled by default).
    pub fn with_cfg_generation(mut self, enabled: bool) -> Self {
        self.options.cfg_generation = enabled;
        self
    }

    /// Configure early exit analysis (enabled by default).
    pub fn with_early_exit_analysis(mut self, enabled: bool) -> Self {
        self.options.early_exit_analysis = enabled;
        self
    }

    /// Configure loop analysis (enabled by default).
    pub fn with_loop_analysis(mut self, enabled: bool) -> Self {
        self.options.loop_analysis = enabled;
        self
    }

    /// Configure basic block generation (enabled by default).
    pub fn with_basic_blocks(mut self, enabled: bool) -> Self {
        self.options.basic_blocks = enabled;
        self
    }

    /// Configure data flow analysis (enabled by default).
    pub fn with_data_flow_analysis(mut self, enabled: bool) -> Self {
        self.options.data_flow_analysis = enabled;
        self
    }

    /// Configure variable lifecycle tracking (enabled by default).
    pub fn with_variable_lifecycle_tracking(mut self, enabled: bool) -> Self {
        self.options.variable_lifecycle_tracking = enabled;
        self
    }

    /// Configure decision tree mapping (enabled by default).
    pub fn with_decision_tree_mapping(mut self, enabled: bool) -> Self {
        self.options.decision_tree_mapping = enabled;
        self
    }

    /// Configure def-use chain analysis (enabled by default).
    pub fn with_def_use_chains(mut self, enabled: bool) -> Self {
        self.options.def_use_chains = enabled;
        self
    }

    /// Use minimal analysis configuration (only complexity and CFG).
    pub fn minimal(mut self) -> Self {
        self.options = AnalysisOptions::minimal();
        self
    }

    /// Disable all analyses.
    pub fn none(mut self) -> Self {
        self.options = AnalysisOptions::none();
        self
    }

    /// Enable or disable workspace search mode.
    pub fn search_workspace(mut self, enabled: bool) -> Self {
        self.options.workspace_search = enabled;
        self.is_workspace = enabled;
        self
    }

    /// Execute the configured analyses and return results.
    pub fn analyze(self) -> Result<AnalysisResult, NTreeError> {
        if !self.options.has_any_enabled() {
            return Err(NTreeError::ParseError(
                "No analyses enabled. Configure at least one analysis type.".to_string(),
            ));
        }

        let workspace_path = if self.is_workspace {
            Some(self.path.clone())
        } else {
            None
        };
        
        let mut result = AnalysisResult::from_source_code(self.path, self.options, self.is_workspace)?;

        // Perform deep external call tracking if enabled
        if self.enable_deep_external_calls {
            use crate::api::analysis::deep_call_tracker::DeepCallTracker;
            let mut tracker = if let Some(path) = workspace_path {
                DeepCallTracker::with_workspace_path(path)
            } else {
                DeepCallTracker::new()
            };
            if let Err(e) = tracker.analyze_external_calls(&result.call_graph, &result.symbol_store) {
                // Log error but don't fail the entire analysis
                eprintln!("Warning: Deep call tracking failed: {}", e);
            } else {
                result.deep_call_tracker = Some(tracker);
            }
        }

        Ok(result)
    }

    /// Get the current path (file or directory).
    pub fn path(&self) -> &Path {
        &self.path
    }

    /// Check if this is workspace mode.
    pub fn is_workspace(&self) -> bool {
        self.is_workspace
    }

    /// Get the current analysis options.
    pub fn options(&self) -> &AnalysisOptions {
        &self.options
    }

    /// Enable incremental analysis for faster recomputation after edits.
    pub fn with_incremental_analysis(mut self, enabled: bool) -> Self {
        self.enable_incremental = enabled;
        self
    }

    /// Enable advanced call resolution for OO/trait calls.
    pub fn with_advanced_call_resolution(mut self, enabled: bool) -> Self {
        self.enable_cha = enabled;
        self.enable_rta = enabled;
        self
    }

    /// Enable external library analysis and security scanning.
    pub fn with_external_library_analysis(mut self, enabled: bool) -> Self {
        self.enable_external_libs = enabled;
        self
    }

    /// Enable deep call tracking into external library source code.
    /// When enabled, ntree will attempt to analyze external library source
    /// code (if available) to track internal function calls.
    /// For example, tracking what functions `requests.get()` calls internally.
    pub fn with_deep_external_call_tracking(mut self, enabled: bool) -> Self {
        self.enable_deep_external_calls = enabled;
        // Deep tracking requires external library analysis
        if enabled {
            self.enable_external_libs = true;
        }
        self
    }
}
