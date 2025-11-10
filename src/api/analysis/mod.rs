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
    IncrementalAnalyzer, IncrementalAnalysisOptions, IncrementalResult, PerformanceMetrics,
};
pub use interprocedural::{
    InterproceduralResult, InterproceduralStats, InterproceduralOptions,
    analyze_interprocedural_cfg, generate_summary_edges, compute_program_reachability,
    analyze_exceptional_control_flow,
};
pub use options::AnalysisOptions;