/// Storage backends for IR data and symbol tracking.

pub mod call_edge;
pub mod call_graph_table;
pub mod constructor_detector;
pub mod cycle_detector;
pub mod data_export;
pub mod dependency_analysis;
pub mod dependency_edges;
pub mod dependency_graph;
pub mod export_table;
pub mod external;
pub mod file_record;
pub mod file_walker;
pub mod incremental;
pub mod interprocedural;
pub mod interprocedural_cfg;
pub mod jsonl_exporter;
pub mod module_graph;
pub mod module_normalizer;
pub mod name_binding;
pub mod name_resolver;
pub mod parse_cache;
pub mod resolution;
pub mod resolution_engine;
pub mod sqlite_storage;
pub mod symbol_core;
pub mod symbol_search;
pub mod symbol_store;

pub use file_record::{FileRecord, ContentHash};
pub use file_walker::FileWalker;
pub use parse_cache::{ParseCache, CacheKey, CachedParseResult, EXTRACTOR_VERSION};
pub use sqlite_storage::SQLiteStorage;
pub use constructor_detector::ConstructorDetector;
pub use data_export::{DataSet, DataSetStats};
pub use dependency_analysis::DependencyAnalysis;
pub use dependency_edges::{ImportEdge, ExportEdge, ImportType, ExportType};
pub use dependency_graph::DependencyGraph;
pub use export_table::{ExportTable, ExportTableStats};
pub use jsonl_exporter::JsonlExporter;
pub use module_graph::{Module, ModuleId, ModuleEdge, ModuleType, EdgeKind};
pub use module_normalizer::ModuleNormalizer;
pub use call_edge::{CallEdge, CallConfidence, CallType};
pub use call_graph_table::{CallGraph, CallGraphStats};
// Incremental analysis
pub use incremental::{
    IncrementalCache, FuncSummary, EffectKind, ThrowsKind, ParamSummary, ReturnSummary,
    InvalidationEngine, ReverseDependencyIndex
};
// Interprocedural analysis
pub use interprocedural::{
    InterproceduralEdge, InterproceduralEdgeKind, CallSiteSummary, EntryPoint,
    ReachabilityInfo, FunctionExit, ExceptionalEdge, ExceptionExitKind
};
pub use interprocedural_cfg::InterproceduralCFG;
// Call resolution
pub use resolution::{
    ClassHierarchyAnalyzer, RapidTypeAnalyzer, TypeInstantiated, Resolution,
    ResolutionAlgorithm, CallSiteId
};
// External libraries
pub use external::{
    ExternalLibraryHandler, ExternalSummary, TaintKind, ContractSpec, SecurityRiskLevel, DependencyIndexer
};
pub use name_binding::{NameBinding, ResolutionOrigin, ResolutionResult};
pub use name_resolver::NameResolver;
pub use symbol_core::{TopLevelSymbol, FunctionFacts, SymbolId, SymbolStoreStats};
pub use symbol_search::{SymbolQuery, SymbolSearcher};
pub use symbol_store::SymbolStore;