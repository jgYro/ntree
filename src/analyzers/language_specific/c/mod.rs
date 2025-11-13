pub mod import_extractor;
/// C-specific analyzers and extractors.
pub mod symbol_extractor;

pub use import_extractor::CImportExtractor;
pub use symbol_extractor::CSymbolExtractor;
