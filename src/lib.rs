pub mod analyzers;
pub mod api;
pub mod core;
pub mod export;
pub mod extractors;
pub mod language;
pub mod models;

// Core exports
pub use core::{create_tree_from_file, read_file, NTreeError};

// Model exports
pub use models::{CfgEdge, CfgNode, ControlFlowGraph, ForLoopIR, FunctionSpan, LoopKind, TopLevelItem};

// Export format exports
pub use export::{export_mermaid, export_mermaid_validated, export_jsonl, validate_mermaid, escape_mermaid_label};

// API exports
pub use api::{
    functions_to_jsonl, generate_cfgs, generate_cfgs_v2, items_to_jsonl, list_functions,
    list_top_level_items, CfgResult,
};