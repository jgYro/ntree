/// Storage backends for IR data and symbol tracking.

pub mod constructor_detector;
pub mod file_record;
pub mod file_walker;
pub mod parse_cache;
pub mod sqlite_storage;
pub mod symbol_core;
pub mod symbol_search;
pub mod symbol_store;

pub use file_record::{FileRecord, ContentHash};
pub use file_walker::FileWalker;
pub use parse_cache::{ParseCache, CacheKey, CachedParseResult, EXTRACTOR_VERSION};
pub use sqlite_storage::SQLiteStorage;
pub use constructor_detector::ConstructorDetector;
pub use symbol_core::{TopLevelSymbol, FunctionFacts, SymbolId, SymbolStoreStats};
pub use symbol_search::{SymbolQuery, SymbolSearcher};
pub use symbol_store::SymbolStore;