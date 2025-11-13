use super::symbol_core::{FunctionFacts, SymbolId, SymbolStoreStats, TopLevelSymbol};
use crate::core::NTreeError;
use std::collections::HashMap;
use std::path::PathBuf;

/// Store for managing symbols and their relationships across files.
#[derive(Debug)]
pub struct SymbolStore {
    symbols: HashMap<SymbolId, TopLevelSymbol>,
    functions: HashMap<SymbolId, FunctionFacts>,
    files: HashMap<PathBuf, Vec<SymbolId>>,
}

impl SymbolStore {
    /// Create a new symbol store.
    pub fn new() -> Self {
        SymbolStore {
            symbols: HashMap::new(),
            functions: HashMap::new(),
            files: HashMap::new(),
        }
    }

    /// Add a symbol to the store.
    pub fn add_symbol(&mut self, symbol: TopLevelSymbol) {
        let file_path = symbol.file_path.clone();
        let symbol_id = symbol.id.clone();

        self.symbols.insert(symbol_id.clone(), symbol);

        // Track symbols by file
        self.files
            .entry(file_path)
            .or_insert_with(Vec::new)
            .push(symbol_id);
    }

    /// Add function facts.
    pub fn add_function_facts(&mut self, facts: FunctionFacts) {
        self.functions.insert(facts.sym_id.clone(), facts);
    }

    /// Get symbol by ID.
    pub fn get_symbol(&self, id: &SymbolId) -> Option<&TopLevelSymbol> {
        self.symbols.get(id)
    }

    /// Get function facts by symbol ID.
    pub fn get_function_facts(&self, id: &SymbolId) -> Option<&FunctionFacts> {
        self.functions.get(id)
    }

    /// Get all symbols in a file.
    pub fn get_file_symbols(&self, file_path: &PathBuf) -> Vec<&TopLevelSymbol> {
        match self.files.get(file_path) {
            Some(symbol_ids) => symbol_ids
                .iter()
                .filter_map(|id| self.symbols.get(id))
                .collect(),
            None => Vec::new(),
        }
    }

    /// Get all symbols (for external processing).
    pub fn get_all_symbols(&self) -> impl Iterator<Item = &TopLevelSymbol> {
        self.symbols.values()
    }

    /// Basic symbol search by substring.
    pub fn find_symbols_by_name(&self, pattern: &str) -> Vec<&TopLevelSymbol> {
        self.symbols
            .values()
            .filter(|symbol| symbol.name.contains(pattern))
            .collect()
    }

    /// Find symbols by exact name match.
    pub fn find_symbols_exact(&self, name: &str) -> Vec<&TopLevelSymbol> {
        self.symbols
            .values()
            .filter(|symbol| symbol.name == name)
            .collect()
    }

    /// Find symbol by exact name (first match).
    pub fn find_by_name(&self, name: &str) -> Result<SymbolId, NTreeError> {
        for symbol in self.symbols.values() {
            if symbol.name == name {
                return Ok(symbol.id.clone());
            }
        }
        Err(NTreeError::InvalidInput(format!(
            "Symbol not found: {}",
            name
        )))
    }

    /// Find symbols matching a pattern.
    pub fn find_symbols_matching(&self, pattern: &str) -> Result<Vec<SymbolId>, NTreeError> {
        let mut matches = Vec::new();
        for symbol in self.symbols.values() {
            if symbol.name.contains(pattern) {
                matches.push(symbol.id.clone());
            }
        }
        Ok(matches)
    }

    /// Get statistics about the symbol store.
    pub fn stats(&self) -> SymbolStoreStats {
        SymbolStoreStats {
            total_symbols: self.symbols.len(),
            total_functions: self.functions.len(),
            total_files: self.files.len(),
        }
    }
}

impl Default for SymbolStore {
    fn default() -> Self {
        Self::new()
    }
}
