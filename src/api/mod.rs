pub mod analysis;
pub mod core;
pub mod export;
pub mod extractors;
pub mod results;

pub use analysis::{
    generate_basic_blocks, generate_cfg_ir, generate_cfg_ir_jsonl, generate_cfgs, generate_cfgs_v2,
    BasicBlockResult, CfgResult, AnalysisOptions,
    InterproceduralResult, InterproceduralStats, InterproceduralOptions,
    analyze_interprocedural_cfg, generate_summary_edges, compute_program_reachability,
    analyze_exceptional_control_flow,
    IncrementalAnalyzer, IncrementalAnalysisOptions, IncrementalResult, PerformanceMetrics,
};
pub use core::{SourceCode, AnalysisResult};
pub use export::{functions_to_jsonl, items_to_jsonl};
pub use results::{
    BasicBlockResultSet, FunctionResultSet, list_functions, list_top_level_items,
    CfgResultSet, ComplexityResultSet, WorkspaceStats,
};