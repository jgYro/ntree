use super::symbol_core::TopLevelSymbol;
use super::symbol_store::SymbolStore;
use crate::core::NTreeError;
use regex::Regex;

/// Query parameters for advanced symbol searching.
#[derive(Debug, Clone)]
pub struct SymbolQuery {
    /// Pattern to match symbol names
    pub name_pattern: Option<String>,
    /// Symbol kind filter (function, class, etc.)
    pub kind: Option<String>,
    /// File path pattern filter
    pub file_pattern: Option<String>,
    /// Whether to use regex matching (default: false)
    pub use_regex: bool,
}

impl SymbolQuery {
    /// Create a new symbol query.
    pub fn new() -> Self {
        SymbolQuery {
            name_pattern: None,
            kind: None,
            file_pattern: None,
            use_regex: false,
        }
    }

    /// Set name pattern for searching.
    pub fn with_name_pattern(mut self, pattern: String, use_regex: bool) -> Self {
        self.name_pattern = Some(pattern);
        self.use_regex = use_regex;
        self
    }

    /// Set kind filter.
    pub fn with_kind(mut self, kind: String) -> Self {
        self.kind = Some(kind);
        self
    }

    /// Set file pattern filter.
    pub fn with_file_pattern(mut self, pattern: String, use_regex: bool) -> Self {
        self.file_pattern = Some(pattern);
        self.use_regex = use_regex;
        self
    }

    /// Enable regex matching for all patterns.
    pub fn with_regex(mut self) -> Self {
        self.use_regex = true;
        self
    }
}

impl Default for SymbolQuery {
    fn default() -> Self {
        Self::new()
    }
}

/// Symbol search utilities.
pub struct SymbolSearcher;

impl SymbolSearcher {
    /// Find symbols using regex pattern.
    pub fn find_symbols_regex<'a>(
        store: &'a SymbolStore,
        pattern: &str,
    ) -> Result<Vec<&'a TopLevelSymbol>, NTreeError> {
        let regex = match Regex::new(pattern) {
            Ok(r) => r,
            Err(e) => {
                return Err(NTreeError::ParseError(format!(
                    "Invalid regex pattern: {}",
                    e
                )))
            }
        };

        let matches = store
            .get_all_symbols()
            .filter(|symbol| regex.is_match(&symbol.name))
            .collect();

        Ok(matches)
    }

    /// Find constructor functions (exact match for "new").
    pub fn find_constructors(store: &SymbolStore) -> Vec<&TopLevelSymbol> {
        store.find_symbols_exact("new")
    }
}
