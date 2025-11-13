use super::dependency_edges::ImportEdge;
use super::export_table::ExportTable;
use super::module_graph::ModuleId;
use super::name_binding::NameBinding;
use super::symbol_core::SymbolId;
use crate::core::NTreeError;
use std::collections::HashMap;
use std::path::PathBuf;

/// Cross-file name resolver with confidence tracking.
#[derive(Debug)]
pub struct NameResolver {
    export_table: ExportTable,
    /// file_path -> imported_alias -> (module_id, original_name)
    import_mappings: HashMap<PathBuf, HashMap<String, (ModuleId, String)>>,
}

impl NameResolver {
    /// Create new name resolver.
    pub fn new() -> Self {
        NameResolver {
            export_table: ExportTable::new(),
            import_mappings: HashMap::new(),
        }
    }

    /// Build resolver from import/export data.
    pub fn from_dependency_data(
        import_edges: &[ImportEdge],
        symbol_exports: &[(ModuleId, String, SymbolId)],
    ) -> Result<Self, NTreeError> {
        let mut resolver = Self::new();

        // Build export table
        for (module_id, exported_name, symbol_id) in symbol_exports {
            resolver.export_table.add_export(
                module_id.clone(),
                exported_name.clone(),
                symbol_id.clone(),
            );
        }

        // Build import mappings
        for import in import_edges {
            resolver.add_import_mapping(import)?;
        }

        Ok(resolver)
    }

    /// Resolve a name usage to symbol ID with confidence.
    pub fn resolve_name(&self, file_path: &PathBuf, name: &str, site_span: String) -> NameBinding {
        use super::resolution_engine::ResolutionEngine;
        let engine = ResolutionEngine::new(&self.export_table, &self.import_mappings);
        engine.resolve_name(file_path, name, site_span)
    }

    /// Add import mapping from import edge.
    fn add_import_mapping(&mut self, import: &ImportEdge) -> Result<(), NTreeError> {
        let module_id = ModuleId::from_language_path(&import.target_module, "python");

        // Handle different import types
        match &import.imported_symbol {
            Some(symbol_name) => {
                // from module import symbol
                self.import_mappings
                    .entry(import.source_file.clone())
                    .or_insert_with(HashMap::new)
                    .insert(symbol_name.clone(), (module_id, symbol_name.clone()));
            }
            None => {
                // import module (module name becomes alias)
                let module_name = import
                    .target_module
                    .split('.')
                    .last()
                    .unwrap_or(&import.target_module)
                    .to_string();

                self.import_mappings
                    .entry(import.source_file.clone())
                    .or_insert_with(HashMap::new)
                    .insert(module_name, (module_id, import.target_module.clone()));
            }
        }

        Ok(())
    }

    /// Get import mappings for debugging.
    pub fn get_import_mappings(&self) -> &HashMap<PathBuf, HashMap<String, (ModuleId, String)>> {
        &self.import_mappings
    }

    /// Get export table for debugging.
    pub fn get_export_table(&self) -> &ExportTable {
        &self.export_table
    }
}
