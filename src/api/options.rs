/// Configuration options for source code analysis.
#[derive(Debug, Clone, PartialEq)]
pub struct AnalysisOptions {
    /// Enable cyclomatic complexity analysis
    pub complexity_analysis: bool,
    /// Enable Control Flow Graph generation
    pub cfg_generation: bool,
    /// Enable early exit pattern analysis
    pub early_exit_analysis: bool,
    /// Enable loop structure analysis
    pub loop_analysis: bool,
    /// Enable basic block generation
    pub basic_blocks: bool,
    /// Enable workspace-wide search and analysis
    pub workspace_search: bool,
}

impl Default for AnalysisOptions {
    /// Create default configuration with all analyses enabled.
    fn default() -> Self {
        AnalysisOptions {
            complexity_analysis: true,
            cfg_generation: true,
            early_exit_analysis: true,
            loop_analysis: true,
            basic_blocks: true,
            workspace_search: false,
        }
    }
}

impl AnalysisOptions {
    /// Create new options with all analyses disabled.
    pub fn none() -> Self {
        AnalysisOptions {
            complexity_analysis: false,
            cfg_generation: false,
            early_exit_analysis: false,
            loop_analysis: false,
            basic_blocks: false,
            workspace_search: false,
        }
    }

    /// Create new options with only essential analyses enabled.
    pub fn minimal() -> Self {
        AnalysisOptions {
            complexity_analysis: true,
            cfg_generation: true,
            early_exit_analysis: false,
            loop_analysis: false,
            basic_blocks: false,
            workspace_search: false,
        }
    }

    /// Check if any analysis is enabled.
    pub fn has_any_enabled(&self) -> bool {
        self.complexity_analysis
            || self.cfg_generation
            || self.early_exit_analysis
            || self.loop_analysis
            || self.basic_blocks
    }
}