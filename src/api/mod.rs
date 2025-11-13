pub mod analysis;
pub mod core;
pub mod export;
pub mod extractors;
pub mod results;

pub use analysis::{
    analyze_exceptional_control_flow, analyze_interprocedural_cfg, compute_program_reachability,
    generate_basic_blocks, generate_cfg_ir, generate_cfg_ir_jsonl, generate_cfgs, generate_cfgs_v2,
    generate_summary_edges, AnalysisOptions, BasicBlockResult, CfgResult,
    IncrementalAnalysisOptions, IncrementalAnalyzer, IncrementalResult, InterproceduralOptions,
    InterproceduralResult, InterproceduralStats, PerformanceMetrics,
};
pub use core::{AnalysisResult, SourceCode};
pub use export::{functions_to_jsonl, items_to_jsonl};
pub use results::{
    list_functions, list_top_level_items, BasicBlockResultSet, CfgResultSet, ComplexityResultSet,
    FunctionResultSet, WorkspaceStats,
};
