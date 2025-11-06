pub mod analysis_runner;
pub mod cfg;
pub mod data_methods;
pub mod dependency_extractors;
pub mod export_utils;
pub mod function_results;
pub mod functions;
pub mod items;
pub mod jsonl;
pub mod language_extractors;
pub mod options;
pub mod result_access;
pub mod result_sets;
pub mod source_code;
pub mod symbol_extractors;
pub mod symbol_methods;
pub mod unified_analysis;
pub mod workspace_methods;

pub use cfg::{
    generate_basic_blocks, generate_cfg_ir, generate_cfg_ir_jsonl, generate_cfgs, generate_cfgs_v2,
    BasicBlockResult, CfgResult,
};
pub use function_results::{BasicBlockResultSet, FunctionResultSet};
pub use functions::list_functions;
pub use items::list_top_level_items;
pub use jsonl::{functions_to_jsonl, items_to_jsonl};
pub use options::AnalysisOptions;
pub use result_sets::{CfgResultSet, ComplexityResultSet};
pub use unified_analysis::AnalysisResult;
pub use source_code::SourceCode;
pub use workspace_methods::WorkspaceStats;