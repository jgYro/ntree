pub mod api;
pub mod core;
pub mod extractors;
pub mod language;
pub mod models;

// Core exports
pub use core::{create_tree_from_file, read_file, NTreeError};

// Model exports
pub use models::{CfgEdge, CfgNode, ControlFlowGraph, FunctionSpan, TopLevelItem};

// API exports
pub use api::{
    functions_to_jsonl, generate_cfgs, generate_cfgs_v2, items_to_jsonl, list_functions,
    list_top_level_items, CfgResult,
};