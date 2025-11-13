use super::module_graph::ModuleId;
use super::symbol_core::SymbolId;
use std::collections::HashMap;

/// Maps exported names to their symbol IDs within modules.
#[derive(Debug, Clone)]
pub struct ExportTable {
    /// module_id -> exported_name -> symbol_id
    exports: HashMap<ModuleId, HashMap<String, SymbolId>>,
}

impl ExportTable {
    /// Create new export table.
    pub fn new() -> Self {
        ExportTable {
            exports: HashMap::new(),
        }
    }

    /// Add an exported symbol to a module.
    pub fn add_export(&mut self, module_id: ModuleId, exported_name: String, symbol_id: SymbolId) {
        self.exports
            .entry(module_id)
            .or_insert_with(HashMap::new)
            .insert(exported_name, symbol_id);
    }

    /// Find symbol ID for an exported name in a module.
    pub fn resolve_export(&self, module_id: &ModuleId, name: &str) -> Option<&SymbolId> {
        self.exports
            .get(module_id)
            .and_then(|module_exports| module_exports.get(name))
    }

    /// Get all exports for a module.
    pub fn get_module_exports(&self, module_id: &ModuleId) -> Vec<&String> {
        match self.exports.get(module_id) {
            Some(module_exports) => module_exports.keys().collect(),
            None => Vec::new(),
        }
    }

    /// Find all modules that export a given name.
    pub fn find_exporters(&self, name: &str) -> Vec<&ModuleId> {
        self.exports
            .iter()
            .filter_map(|(module_id, exports)| {
                if exports.contains_key(name) {
                    Some(module_id)
                } else {
                    None
                }
            })
            .collect()
    }

    /// Get statistics about exports.
    pub fn stats(&self) -> ExportTableStats {
        let total_modules = self.exports.len();
        let total_exports = self.exports.values().map(|exports| exports.len()).sum();

        ExportTableStats {
            modules_with_exports: total_modules,
            total_exports,
        }
    }
}

/// Statistics about the export table.
#[derive(Debug, Clone)]
pub struct ExportTableStats {
    pub modules_with_exports: usize,
    pub total_exports: usize,
}

impl Default for ExportTable {
    fn default() -> Self {
        Self::new()
    }
}
