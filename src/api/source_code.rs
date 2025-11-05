use std::path::{Path, PathBuf};
use crate::core::NTreeError;
use super::options::AnalysisOptions;
use super::unified_analysis::AnalysisResult;

/// Builder for source code analysis with fluent configuration API.
#[derive(Debug, Clone)]
pub struct SourceCode {
    path: PathBuf,
    is_workspace: bool,
    options: AnalysisOptions,
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
                "No analyses enabled. Configure at least one analysis type.".to_string()
            ));
        }

        AnalysisResult::from_source_code(self.path, self.options, self.is_workspace)
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
}