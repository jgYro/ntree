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
    /// Enable data flow analysis
    pub data_flow_analysis: bool,
    /// Enable variable lifecycle tracking
    pub variable_lifecycle_tracking: bool,
    /// Enable decision tree mapping
    pub decision_tree_mapping: bool,
    /// Enable def-use chain analysis
    pub def_use_chains: bool,
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
            data_flow_analysis: true,
            variable_lifecycle_tracking: true,
            decision_tree_mapping: true,
            def_use_chains: true,
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
            data_flow_analysis: false,
            variable_lifecycle_tracking: false,
            decision_tree_mapping: false,
            def_use_chains: false,
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
            data_flow_analysis: false,
            variable_lifecycle_tracking: false,
            decision_tree_mapping: false,
            def_use_chains: false,
        }
    }

    /// Check if any analysis is enabled.
    pub fn has_any_enabled(&self) -> bool {
        self.complexity_analysis
            || self.cfg_generation
            || self.early_exit_analysis
            || self.loop_analysis
            || self.basic_blocks
            || self.data_flow_analysis
            || self.variable_lifecycle_tracking
            || self.decision_tree_mapping
            || self.def_use_chains
    }
}