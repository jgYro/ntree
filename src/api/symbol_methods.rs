use crate::core::NTreeError;
use crate::storage::{SymbolStore, TopLevelSymbol, SymbolSearcher, ConstructorDetector};

/// Symbol search result set with fluent builder pattern.
pub struct SymbolResultSet<'a> {
    store: &'a SymbolStore,
    pattern: Option<String>,
    use_regex: bool,
    kind_filter: Option<String>,
    file_filter: Option<String>,
}

impl<'a> SymbolResultSet<'a> {
    /// Create a new symbol result set.
    pub fn new(store: &'a SymbolStore) -> Self {
        SymbolResultSet {
            store,
            pattern: None,
            use_regex: false,
            kind_filter: None,
            file_filter: None,
        }
    }

    /// Set name pattern for searching.
    pub fn named(mut self, pattern: &str) -> Self {
        self.pattern = Some(pattern.to_string());
        self
    }

    /// Enable regex pattern matching.
    pub fn regex(mut self, enabled: bool) -> Self {
        self.use_regex = enabled;
        self
    }

    /// Filter by symbol kind (function, class, struct, etc.).
    pub fn kind(mut self, kind: &str) -> Self {
        self.kind_filter = Some(kind.to_string());
        self
    }

    /// Filter by file path pattern.
    pub fn in_file(mut self, file_pattern: &str) -> Self {
        self.file_filter = Some(file_pattern.to_string());
        self
    }

    /// Execute the search and return matching symbols.
    pub fn search(&self) -> Result<Vec<&TopLevelSymbol>, NTreeError> {
        let mut results: Vec<&TopLevelSymbol> = self.store.get_all_symbols().collect();

        // Apply name pattern filter
        if let Some(ref pattern) = self.pattern {
            if self.use_regex {
                results = SymbolSearcher::find_symbols_regex(self.store, pattern)?;
            } else {
                results.retain(|symbol| symbol.name.contains(pattern));
            }
        }

        // Apply kind filter
        if let Some(ref kind) = self.kind_filter {
            results.retain(|symbol| symbol.kind == *kind);
        }

        // Apply file filter
        if let Some(ref file_pattern) = self.file_filter {
            results.retain(|symbol| symbol.file_path.to_string_lossy().contains(file_pattern));
        }

        Ok(results)
    }

    /// Find constructor functions across all languages.
    pub fn constructors(&self) -> Result<Vec<&TopLevelSymbol>, NTreeError> {
        ConstructorDetector::find_constructors(self.store)
    }

    /// Get all symbols without filtering.
    pub fn all(&self) -> Vec<&TopLevelSymbol> {
        self.store.get_all_symbols().collect()
    }

    /// Count matching symbols without materializing results.
    pub fn count(&self) -> Result<usize, NTreeError> {
        match self.search() {
            Ok(results) => Ok(results.len()),
            Err(e) => Err(e),
        }
    }
}