use super::dependency_edges::{ExportEdge, ImportEdge};
use super::file_record::FileRecord;
use super::jsonl_exporter::JsonlExporter;
use super::symbol_core::{FunctionFacts, TopLevelSymbol};
use crate::core::NTreeError;

/// Complete dataset for export to JSONL or database.
#[derive(Debug)]
pub struct DataSet {
    pub files: Vec<FileRecord>,
    pub symbols: Vec<TopLevelSymbol>,
    pub function_facts: Vec<FunctionFacts>,
    pub import_edges: Vec<ImportEdge>,
    pub export_edges: Vec<ExportEdge>,
}

impl DataSet {
    /// Create a new empty dataset.
    pub fn new() -> Self {
        DataSet {
            files: Vec::new(),
            symbols: Vec::new(),
            function_facts: Vec::new(),
            import_edges: Vec::new(),
            export_edges: Vec::new(),
        }
    }

    /// Add file record.
    pub fn add_file(&mut self, file: FileRecord) {
        self.files.push(file);
    }

    /// Add symbol.
    pub fn add_symbol(&mut self, symbol: TopLevelSymbol) {
        self.symbols.push(symbol);
    }

    /// Add function facts.
    pub fn add_function_facts(&mut self, facts: FunctionFacts) {
        self.function_facts.push(facts);
    }

    /// Add import edge.
    pub fn add_import_edge(&mut self, import: ImportEdge) {
        self.import_edges.push(import);
    }

    /// Add export edge.
    pub fn add_export_edge(&mut self, export: ExportEdge) {
        self.export_edges.push(export);
    }

    /// Export complete dataset to JSONL format.
    pub fn to_jsonl(&self) -> Result<String, NTreeError> {
        JsonlExporter::export_dataset(self)
    }

    /// Get statistics about the dataset.
    pub fn stats(&self) -> DataSetStats {
        DataSetStats {
            files: self.files.len(),
            symbols: self.symbols.len(),
            function_facts: self.function_facts.len(),
            import_edges: self.import_edges.len(),
            export_edges: self.export_edges.len(),
        }
    }
}

/// Statistics about a dataset.
#[derive(Debug, Clone)]
pub struct DataSetStats {
    pub files: usize,
    pub symbols: usize,
    pub function_facts: usize,
    pub import_edges: usize,
    pub export_edges: usize,
}

impl Default for DataSet {
    fn default() -> Self {
        Self::new()
    }
}
