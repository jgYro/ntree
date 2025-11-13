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
pub mod project_detector;
pub mod resolution;
pub mod resolution_engine;
pub mod sqlite_storage;
pub mod symbol_core;
pub mod symbol_search;
pub mod symbol_store;

pub use call_edge::{CallConfidence, CallEdge, CallType};
pub use call_graph_table::{CallGraph, CallGraphStats};
pub use constructor_detector::ConstructorDetector;
pub use data_export::{DataSet, DataSetStats};
pub use dependency_analysis::DependencyAnalysis;
pub use dependency_edges::{ExportEdge, ExportType, ImportEdge, ImportType};
pub use dependency_graph::DependencyGraph;
pub use export_table::{ExportTable, ExportTableStats};
pub use file_record::{ContentHash, FileRecord};
pub use file_walker::FileWalker;
pub use jsonl_exporter::JsonlExporter;
pub use module_graph::{EdgeKind, Module, ModuleEdge, ModuleId, ModuleType};
pub use module_normalizer::ModuleNormalizer;
pub use parse_cache::{CacheKey, CachedParseResult, ParseCache, EXTRACTOR_VERSION};
pub use project_detector::{ProjectDetector, ProjectInfo, ProjectType};
pub use sqlite_storage::SQLiteStorage;
// Incremental analysis
pub use incremental::{
    EffectKind, FuncSummary, IncrementalCache, InvalidationEngine, ParamSummary, ReturnSummary,
    ReverseDependencyIndex, ThrowsKind,
};
// Interprocedural analysis
pub use interprocedural::{
    CallSiteSummary, EntryPoint, ExceptionExitKind, ExceptionalEdge, FunctionExit,
    InterproceduralEdge, InterproceduralEdgeKind, ReachabilityInfo,
};
pub use interprocedural_cfg::InterproceduralCFG;
// Call resolution
pub use resolution::{
    CallSiteId, ClassHierarchyAnalyzer, RapidTypeAnalyzer, Resolution, ResolutionAlgorithm,
    TypeInstantiated,
};
// External libraries
pub use external::{
    ContractSpec, DependencyIndexer, ExternalLibraryHandler, ExternalSummary, SecurityRiskLevel,
    TaintKind,
};
pub use name_binding::{NameBinding, ResolutionOrigin, ResolutionResult};
pub use name_resolver::NameResolver;
pub use symbol_core::{FunctionFacts, SymbolId, SymbolStoreStats, TopLevelSymbol};
pub use symbol_search::{SymbolQuery, SymbolSearcher};
pub use symbol_store::SymbolStore;
