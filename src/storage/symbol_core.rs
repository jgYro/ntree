use serde::{Deserialize, Serialize};
use std::path::PathBuf;

/// Unique identifier for a symbol across the codebase.
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq, Hash)]
pub struct SymbolId(String);

impl SymbolId {
    /// Create a new symbol ID from file path and symbol name.
    pub fn new(file_path: &PathBuf, symbol_name: &str) -> Self {
        SymbolId(format!("{}::{}", file_path.display(), symbol_name))
    }

    /// Create a symbol ID from a string (for testing).
    pub fn from_string(id: String) -> Self {
        SymbolId(id)
    }

    /// Get the string representation.
    pub fn as_str(&self) -> &str {
        &self.0
    }
}

/// Top-level symbol information (functions, classes, etc.).
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct TopLevelSymbol {
    /// Unique identifier
    pub id: SymbolId,
    /// Symbol name
    pub name: String,
    /// Symbol kind (function, class, interface, etc.)
    pub kind: String,
    /// Qualified name including namespace/module
    pub qualname: String,
    /// Source location span
    pub span: String,
    /// File where symbol is defined
    pub file_path: PathBuf,
}

impl TopLevelSymbol {
    /// Create a new top-level symbol.
    pub fn new(
        file_path: PathBuf,
        name: String,
        kind: String,
        qualname: String,
        span: String,
    ) -> Self {
        let id = SymbolId::new(&file_path, &name);
        TopLevelSymbol {
            id,
            name,
            kind,
            qualname,
            span,
            file_path,
        }
    }
}

/// Function-specific facts for detailed analysis.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct FunctionFacts {
    pub sym_id: SymbolId,
    pub params: Vec<String>,
    pub span: String,
    pub body_span: Option<String>,
    pub complexity: u32,
    pub loc: u32,
    pub return_type: Option<String>,
    pub is_async: bool,
    pub is_private: bool,
}

impl FunctionFacts {
    /// Create function facts from analysis results.
    pub fn from_function_analysis(
        symbol: &TopLevelSymbol,
        function_span: &crate::models::FunctionSpan,
        complexity_result: &crate::analyzers::ComplexityResult,
    ) -> Self {
        FunctionFacts {
            sym_id: symbol.id.clone(),
            params: Vec::new(), // TODO: Extract from AST
            span: function_span.span.clone(),
            body_span: function_span.body.clone(),
            complexity: complexity_result.cyclomatic,
            loc: 0,            // TODO: Calculate from span
            return_type: None, // TODO: Extract from AST
            is_async: false,   // TODO: Detect from AST
            is_private: symbol.name.starts_with('_'),
        }
    }
}

/// Statistics about the symbol store.
#[derive(Debug, Clone)]
pub struct SymbolStoreStats {
    pub total_symbols: usize,
    pub total_functions: usize,
    pub total_files: usize,
}
