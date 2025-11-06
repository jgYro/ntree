/// JavaScript-specific analyzers and extractors.

pub mod symbol_extractor;
pub mod import_extractor;

pub use symbol_extractor::JavaScriptSymbolExtractor;
pub use import_extractor::JavaScriptImportExtractor;