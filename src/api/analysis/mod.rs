pub mod analysis_runner;
pub mod cfg;
pub mod incremental;
pub mod interprocedural;
pub mod options;

pub use cfg::{
    generate_basic_blocks, generate_cfg_ir, generate_cfg_ir_jsonl, generate_cfgs, generate_cfgs_v2,
    BasicBlockResult, CfgResult,
};
pub use incremental::{
    IncrementalAnalysisOptions, IncrementalAnalyzer, IncrementalResult, PerformanceMetrics,
};
pub use interprocedural::{
    analyze_exceptional_control_flow, analyze_interprocedural_cfg, compute_program_reachability,
    generate_summary_edges, InterproceduralOptions, InterproceduralResult, InterproceduralStats,
};
pub use options::AnalysisOptions;
