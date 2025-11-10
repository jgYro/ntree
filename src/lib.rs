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

// Simple public API
pub use api::{SourceCode, AnalysisResult};

// Essential functions for existing tests (not part of main API)
pub use api::{
    generate_basic_blocks, generate_cfgs, items_to_jsonl, list_top_level_items,
};

// Language exports
pub use language::{SupportedLanguage, LanguageConfig, detect_language_config};

// Storage exports
pub use storage::{
    FileRecord, ContentHash, FileWalker, ParseCache, CacheKey,
    TopLevelSymbol, FunctionFacts, SymbolStore, SymbolId,
    SymbolSearcher, ConstructorDetector,
    // Module dependency graph
    DependencyGraph, Module, ModuleId, ModuleEdge, ModuleType, EdgeKind,
    DependencyAnalysis, ImportEdge, ExportEdge, ImportType, ExportType,
    DataSet, JsonlExporter, ModuleNormalizer,
    // Global symbol table
    NameResolver, NameBinding, ResolutionOrigin, ExportTable,
    // Interprocedural CFG
    InterproceduralCFG, InterproceduralEdge, InterproceduralEdgeKind,
    CallSiteSummary, EntryPoint, ReachabilityInfo, FunctionExit,
    ExceptionalEdge, ExceptionExitKind,
    // Incremental & cache
    IncrementalCache, FuncSummary, EffectKind, ThrowsKind, ParamSummary, ReturnSummary, InvalidationEngine,
    // Call resolution
    ClassHierarchyAnalyzer, RapidTypeAnalyzer, TypeInstantiated, Resolution,
    ResolutionAlgorithm, CallSiteId,
    // External libraries
    ExternalLibraryHandler, ExternalSummary, TaintKind, ContractSpec, SecurityRiskLevel, DependencyIndexer,
};

// Analyzer exports
pub use analyzers::{
    ComplexityAnalyzer, ComplexityResult, EarlyExitNormalizer, ForLoopNormalizer,
};