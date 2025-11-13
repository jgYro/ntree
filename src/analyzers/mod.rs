/// Language-specific analyzers and language-agnostic IR normalization.
pub mod complexity_analyzer;
pub mod data_flow_analyzer;
pub mod early_exit_normalizer;
pub mod for_loop_normalizer;
pub mod language_specific;
pub mod variable_lifecycle_analyzer;
pub mod workspace_data_flow_analyzer;

pub use complexity_analyzer::{ComplexityAnalyzer, ComplexityResult};
pub use data_flow_analyzer::DataFlowAnalyzer;
pub use early_exit_normalizer::EarlyExitNormalizer;
pub use for_loop_normalizer::ForLoopNormalizer;
pub use variable_lifecycle_analyzer::VariableLifecycleAnalyzer;
pub use workspace_data_flow_analyzer::{
    CrossFileVariable, WorkspaceDataFlowAnalyzer, WorkspaceDataFlowResult,
};
