use std::path::{Path, PathBuf};
use crate::core::NTreeError;
use super::options::AnalysisOptions;
use super::results::AnalysisResult;

/// Builder for source code analysis with fluent configuration API.
#[derive(Debug, Clone)]
pub struct SourceCode {
    file_path: PathBuf,
    options: AnalysisOptions,
}

impl SourceCode {
    /// Create a new source code analyzer for the given file path.
    pub fn new<P: AsRef<Path>>(path: P) -> Result<Self, NTreeError> {
        let file_path = path.as_ref().to_path_buf();

        if !file_path.exists() {
            return Err(NTreeError::ParseError(format!(
                "File does not exist: {}",
                file_path.display()
            )));
        }

        Ok(SourceCode {
            file_path,
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

    /// Execute the configured analyses and return results.
    pub fn analyze(self) -> Result<AnalysisResult, NTreeError> {
        if !self.options.has_any_enabled() {
            return Err(NTreeError::ParseError(
                "No analyses enabled. Configure at least one analysis type.".to_string()
            ));
        }

        AnalysisResult::from_source_code(self.file_path, self.options)
    }

    /// Get the current file path.
    pub fn file_path(&self) -> &Path {
        &self.file_path
    }

    /// Get the current analysis options.
    pub fn options(&self) -> &AnalysisOptions {
        &self.options
    }
}