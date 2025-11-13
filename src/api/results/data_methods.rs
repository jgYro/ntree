use crate::api::core::unified_analysis::AnalysisResult;
use crate::api::extractors::language_extractors::LanguageExtractors;
use crate::core::NTreeError;
use crate::storage::DataSet;

/// Data export methods for AnalysisResult.
impl AnalysisResult {
    /// Export complete dataset (File, Symbol, ImportEdge, ExportEdge, FunctionFacts).
    pub fn export_dataset(&self) -> Result<DataSet, NTreeError> {
        let mut dataset = DataSet::new();

        // Add file records (if workspace mode)
        for file in &self.file_records {
            dataset.add_file(file.clone());
        }

        // Add symbols
        for symbol in self.symbol_store.get_all_symbols() {
            dataset.add_symbol(symbol.clone());
        }

        // Add function facts
        for symbol in self.symbol_store.get_all_symbols() {
            if let Some(facts) = self.symbol_store.get_function_facts(&symbol.id) {
                dataset.add_function_facts(facts.clone());
            }
        }

        // Extract import/export edges from files
        for file in &self.file_records {
            match LanguageExtractors::extract_dependencies(&file.path) {
                Ok((imports, exports)) => {
                    for import in imports {
                        dataset.add_import_edge(import);
                    }
                    for export in exports {
                        dataset.add_export_edge(export);
                    }
                }
                Err(_) => continue, // Skip files with extraction errors
            }
        }

        Ok(dataset)
    }

    /// Export complete dataset to structured JSONL.
    pub fn to_dataset_jsonl(&self) -> Result<String, NTreeError> {
        let dataset = self.export_dataset()?;
        dataset.to_jsonl()
    }

    /// Get dataset statistics.
    pub fn dataset_stats(&self) -> Result<crate::storage::DataSetStats, NTreeError> {
        let dataset = self.export_dataset()?;
        Ok(dataset.stats())
    }
}
