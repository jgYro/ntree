/// C-specific analyzers and extractors.

pub mod symbol_extractor;
pub mod import_extractor;

pub use symbol_extractor::CSymbolExtractor;
pub use import_extractor::CImportExtractor;