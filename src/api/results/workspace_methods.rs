use crate::analyzers::{WorkspaceDataFlowAnalyzer, WorkspaceDataFlowResult};
use crate::api::analysis::options::AnalysisOptions;
use crate::api::extractors::language_extractors::LanguageExtractors;
use crate::core::NTreeError;
use crate::storage::{FileRecord, FileWalker, InterproceduralCFG, NameResolver, SymbolStore};
use serde::{Deserialize, Serialize};
use std::collections::HashMap;
use std::path::PathBuf;

/// Workspace-specific analysis methods.
pub struct WorkspaceMethods;

impl WorkspaceMethods {
    /// Populate workspace data from file discovery.
    pub fn populate_workspace_data(
        workspace_path: &PathBuf,
        _options: &AnalysisOptions,
        symbol_store: &mut SymbolStore,
    ) -> Result<(Vec<FileRecord>, HashMap<String, Vec<FileRecord>>), NTreeError> {
        let walker = FileWalker::new(workspace_path);
        let file_records = walker.discover_files()?;

        let mut files_by_language = HashMap::new();

        // Process each file
        for file_record in &file_records {
            // Group by language
            files_by_language
                .entry(file_record.language.clone())
                .or_insert_with(Vec::new)
                .push(file_record.clone());

            // Extract symbols using language-specific extractors
            LanguageExtractors::extract_symbols(&file_record.path, symbol_store)?;
        }

        Ok((file_records, files_by_language))
    }

    /// Perform workspace-wide data flow analysis using existing infrastructure.
    pub fn analyze_workspace_data_flow(
        workspace_path: &PathBuf,
        options: &AnalysisOptions,
        file_records: &[FileRecord],
        symbol_store: &SymbolStore,
    ) -> Result<Option<WorkspaceDataFlowResult>, NTreeError> {
        // Only run if data flow analysis is enabled
        if !options.data_flow_analysis
            && !options.variable_lifecycle_tracking
            && !options.def_use_chains
        {
            return Ok(None);
        }

        // Create minimal interprocedural CFG and name resolver for data flow analysis
        let interprocedural_cfg = InterproceduralCFG::new();
        let name_resolver = NameResolver::new();

        // Use workspace data flow analyzer
        let mut analyzer = WorkspaceDataFlowAnalyzer::new();
        let result = analyzer.analyze_workspace(
            workspace_path,
            file_records,
            symbol_store,
            &name_resolver,
            &interprocedural_cfg,
        )?;

        Ok(Some(result))
    }

    /// Get workspace statistics.
    pub fn get_workspace_stats(file_records: &[FileRecord]) -> WorkspaceStats {
        let total_files = file_records.len();
        let total_size: u64 = file_records.iter().map(|f| f.size).sum();

        WorkspaceStats {
            total_files,
            total_size,
            languages: file_records
                .iter()
                .map(|f| f.language.clone())
                .collect::<std::collections::HashSet<_>>()
                .len(),
        }
    }
}

/// Statistics about workspace analysis.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct WorkspaceStats {
    pub total_files: usize,
    pub total_size: u64,
    pub languages: usize,
}
