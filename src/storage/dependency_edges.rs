use std::path::PathBuf;
use serde::{Deserialize, Serialize};
use super::symbol_core::SymbolId;

/// Import relationship between files or modules.
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub struct ImportEdge {
    pub source_file: PathBuf,
    pub target_module: String,
    pub imported_symbol: Option<String>,
    pub import_type: ImportType,
    pub span: String,
    pub import_syntax: String,
}

/// Export relationship from files or modules.
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub struct ExportEdge {
    pub source_file: PathBuf,
    pub exported_symbol: SymbolId,
    pub export_type: ExportType,
    pub target_module: Option<String>,
    pub span: String,
    pub visibility: String,
}

/// Types of import relationships.
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub enum ImportType {
    /// Import entire module
    Module,
    /// Import specific symbol(s)
    Symbol,
    /// Wildcard import (from x import *)
    Wildcard,
    /// Relative import
    Relative,
}

/// Types of export relationships.
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub enum ExportType {
    /// Public export
    Public,
    /// Default export (JavaScript/TypeScript)
    Default,
    /// Named export
    Named,
    /// Re-export from another module
    ReExport,
}

impl ImportEdge {
    /// Create a new import edge.
    pub fn new(
        source_file: PathBuf,
        target_module: String,
        imported_symbol: Option<String>,
        import_type: ImportType,
        span: String,
        import_syntax: String,
    ) -> Self {
        ImportEdge {
            source_file,
            target_module,
            imported_symbol,
            import_type,
            span,
            import_syntax,
        }
    }
}

impl ExportEdge {
    /// Create a new export edge.
    pub fn new(
        source_file: PathBuf,
        exported_symbol: SymbolId,
        export_type: ExportType,
        target_module: Option<String>,
        span: String,
        visibility: String,
    ) -> Self {
        ExportEdge {
            source_file,
            exported_symbol,
            export_type,
            target_module,
            span,
            visibility,
        }
    }
}