use serde_json;
use crate::core::NTreeError;
use super::file_record::FileRecord;
use super::symbol_core::{TopLevelSymbol, FunctionFacts};
use super::dependency_edges::{ImportEdge, ExportEdge};
use super::data_export::DataSet;

/// JSONL export utilities for all data types.
pub struct JsonlExporter;

impl JsonlExporter {
    /// Export complete dataset to JSONL format.
    pub fn export_dataset(dataset: &DataSet) -> Result<String, NTreeError> {
        let mut jsonl = String::new();

        // Export each data type with type labels
        jsonl.push_str(&Self::export_files(&dataset.files)?);
        jsonl.push_str(&Self::export_symbols(&dataset.symbols)?);
        jsonl.push_str(&Self::export_function_facts(&dataset.function_facts)?);
        jsonl.push_str(&Self::export_import_edges(&dataset.import_edges)?);
        jsonl.push_str(&Self::export_export_edges(&dataset.export_edges)?);

        Ok(jsonl)
    }

    /// Export files to JSONL with type label.
    fn export_files(files: &[FileRecord]) -> Result<String, NTreeError> {
        let mut jsonl = String::new();
        for file in files {
            match serde_json::to_string(file) {
                Ok(json) => {
                    jsonl.push_str(&format!("{{\"type\":\"File\",\"data\":{}}}\n", json));
                }
                Err(e) => return Err(NTreeError::ParseError(format!("File export failed: {}", e))),
            }
        }
        Ok(jsonl)
    }

    /// Export symbols to JSONL with type label.
    fn export_symbols(symbols: &[TopLevelSymbol]) -> Result<String, NTreeError> {
        let mut jsonl = String::new();
        for symbol in symbols {
            match serde_json::to_string(symbol) {
                Ok(json) => {
                    jsonl.push_str(&format!("{{\"type\":\"Symbol\",\"data\":{}}}\n", json));
                }
                Err(e) => return Err(NTreeError::ParseError(format!("Symbol export failed: {}", e))),
            }
        }
        Ok(jsonl)
    }

    /// Export function facts to JSONL.
    fn export_function_facts(facts: &[FunctionFacts]) -> Result<String, NTreeError> {
        let mut jsonl = String::new();
        for fact in facts {
            match serde_json::to_string(fact) {
                Ok(json) => {
                    jsonl.push_str(&format!("{{\"type\":\"FunctionFacts\",\"data\":{}}}\n", json));
                }
                Err(e) => return Err(NTreeError::ParseError(format!("FunctionFacts export failed: {}", e))),
            }
        }
        Ok(jsonl)
    }

    /// Export import edges to JSONL.
    fn export_import_edges(imports: &[ImportEdge]) -> Result<String, NTreeError> {
        let mut jsonl = String::new();
        for import in imports {
            match serde_json::to_string(import) {
                Ok(json) => {
                    jsonl.push_str(&format!("{{\"type\":\"ImportEdge\",\"data\":{}}}\n", json));
                }
                Err(e) => return Err(NTreeError::ParseError(format!("ImportEdge export failed: {}", e))),
            }
        }
        Ok(jsonl)
    }

    /// Export export edges to JSONL.
    fn export_export_edges(exports: &[ExportEdge]) -> Result<String, NTreeError> {
        let mut jsonl = String::new();
        for export in exports {
            match serde_json::to_string(export) {
                Ok(json) => {
                    jsonl.push_str(&format!("{{\"type\":\"ExportEdge\",\"data\":{}}}\n", json));
                }
                Err(e) => return Err(NTreeError::ParseError(format!("ExportEdge export failed: {}", e))),
            }
        }
        Ok(jsonl)
    }
}