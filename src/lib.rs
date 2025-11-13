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
    ActionType,
    BasicBlock,
    BasicBlockEdge,
    BasicBlockGraph,
    BranchType,
    CFGEdgeIR,
    CFGNodeIR,
    CfgEdge,
    CfgNode,
    ConditionOperator,
    ControlFlowGraph,
    DataDependencyEdge,
    // Data flow analysis types
    DataFlowGraph,
    DataFlowNode,
    DecisionAction,
    DecisionBranch,
    DecisionCondition,
    DecisionPath,
    DecisionTree,
    DecisionTreeNode,
    DecisionTreeSet,
    DefUseChain,
    DefUseChainSet,
    DefUseSite,
    DefUseSiteType,
    DependencyType,
    EarlyExitIR,
    EarlyExitKind,
    ForLoopIR,
    FunctionCFGIR,
    FunctionSpan,
    LoopKind,
    TopLevelItem,
    VariableDefinition,
    VariableEvent,
    VariableEventType,
    VariableLifecycle,
    VariableLifecycleSet,
    VariableScope,
    VariableState,
};

// Export format exports
pub use export::{
    escape_mermaid_label, export_jsonl, export_mermaid, export_mermaid_validated, validate_mermaid,
};

// Simple public API
pub use api::{AnalysisResult, SourceCode};

// Essential functions for existing tests (not part of main API)
pub use api::{generate_basic_blocks, generate_cfgs, items_to_jsonl, list_top_level_items};

// Language exports
pub use language::{detect_language_config, LanguageConfig, SupportedLanguage};

// Storage exports
pub use storage::{
    CacheKey,
    CallSiteId,
    CallSiteSummary,
    // Call resolution
    ClassHierarchyAnalyzer,
    ConstructorDetector,
    ContentHash,
    ContractSpec,
    DataSet,
    DependencyAnalysis,
    // Module dependency graph
    DependencyGraph,
    DependencyIndexer,
    EdgeKind,
    EffectKind,
    EntryPoint,
    ExceptionExitKind,
    ExceptionalEdge,
    ExportEdge,
    ExportTable,
    ExportType,
    // External libraries
    ExternalLibraryHandler,
    ExternalSummary,
    FileRecord,
    FileWalker,
    FuncSummary,
    FunctionExit,
    FunctionFacts,
    ImportEdge,
    ImportType,
    // Incremental & cache
    IncrementalCache,
    // Interprocedural CFG
    InterproceduralCFG,
    InterproceduralEdge,
    InterproceduralEdgeKind,
    InvalidationEngine,
    JsonlExporter,
    Module,
    ModuleEdge,
    ModuleId,
    ModuleNormalizer,
    ModuleType,
    NameBinding,
    // Global symbol table
    NameResolver,
    ParamSummary,
    ParseCache,
    // Project detection
    ProjectDetector,
    ProjectInfo,
    ProjectType,
    RapidTypeAnalyzer,
    ReachabilityInfo,
    Resolution,
    ResolutionAlgorithm,
    ResolutionOrigin,
    ReturnSummary,
    SecurityRiskLevel,
    SymbolId,
    SymbolSearcher,
    SymbolStore,
    TaintKind,
    ThrowsKind,
    TopLevelSymbol,
    TypeInstantiated,
};

// Analyzer exports
pub use analyzers::{
    ComplexityAnalyzer, ComplexityResult, CrossFileVariable, DataFlowAnalyzer, EarlyExitNormalizer,
    ForLoopNormalizer, VariableLifecycleAnalyzer, WorkspaceDataFlowAnalyzer,
    WorkspaceDataFlowResult,
};
