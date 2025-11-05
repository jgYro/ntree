pub mod analyzers;
pub mod api;
pub mod core;
pub mod export;
pub mod extractors;
pub mod language;
pub mod models;
pub mod storage;

// Core exports
pub use core::{create_tree_from_file, read_file, NTreeError};

// Model exports
pub use models::{
    BasicBlock, BasicBlockEdge, BasicBlockGraph, CFGEdgeIR, CFGNodeIR, CfgEdge, CfgNode,
    ControlFlowGraph, EarlyExitIR, EarlyExitKind, ForLoopIR, FunctionCFGIR, FunctionSpan,
    LoopKind, TopLevelItem,
};

// Export format exports
pub use export::{export_mermaid, export_mermaid_validated, export_jsonl, validate_mermaid, escape_mermaid_label};

// API exports
pub use api::{
    functions_to_jsonl, generate_basic_blocks, generate_cfg_ir, generate_cfg_ir_jsonl,
    generate_cfgs, generate_cfgs_v2, items_to_jsonl, list_functions, list_top_level_items,
    BasicBlockResult, CfgResult,
    // Unified API
    SourceCode, AnalysisResult, AnalysisOptions,
    ComplexityResultSet, CfgResultSet, FunctionResultSet, BasicBlockResultSet,
    WorkspaceStats,
};

// Language exports
pub use language::{SupportedLanguage, LanguageConfig, detect_language_config};

// Storage exports
pub use storage::{
    FileRecord, ContentHash, FileWalker, ParseCache, CacheKey,
    TopLevelSymbol, FunctionFacts, SymbolStore, SymbolId,
    SymbolSearcher, ConstructorDetector,
};

// Analyzer exports
pub use analyzers::{
    ComplexityAnalyzer, ComplexityResult, EarlyExitNormalizer, ForLoopNormalizer,
};