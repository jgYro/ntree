use std::path::PathBuf;
use crate::core::NTreeError;
use crate::models::FunctionSpan;
use crate::analyzers::ComplexityResult;
use crate::api::{CfgResult, BasicBlockResult};
use super::options::AnalysisOptions;
use super::result_sets::{ComplexityResultSet, CfgResultSet};
use super::function_results::{FunctionResultSet, BasicBlockResultSet};
use super::analysis_runner::AnalysisRunner;
use super::export_utils::ExportUtils;

/// Complete analysis results for a source file.
#[derive(Debug)]
pub struct AnalysisResult {
    complexity_data: Vec<ComplexityResult>,
    cfg_data: Vec<CfgResult>,
    basic_block_data: Vec<BasicBlockResult>,
    function_data: Vec<FunctionSpan>,
}

impl AnalysisResult {
    /// Create analysis result by running configured analyses.
    pub fn from_source_code(
        file_path: PathBuf,
        options: AnalysisOptions,
    ) -> Result<Self, NTreeError> {
        let mut result = AnalysisResult {
            complexity_data: Vec::new(),
            cfg_data: Vec::new(),
            basic_block_data: Vec::new(),
            function_data: Vec::new(),
        };

        // Get function list first (needed for other analyses)
        match AnalysisRunner::run_function_extraction(&file_path) {
            Ok(functions) => result.function_data = functions,
            Err(e) => return Err(e),
        }

        // Run CFG generation if enabled
        if options.cfg_generation {
            match AnalysisRunner::run_cfg_generation(&file_path) {
                Ok(cfgs) => result.cfg_data = cfgs,
                Err(e) => return Err(e),
            }
        }

        // Run basic block generation if enabled
        if options.basic_blocks {
            match AnalysisRunner::run_basic_block_generation(&file_path) {
                Ok(blocks) => result.basic_block_data = blocks,
                Err(e) => return Err(e),
            }
        }

        // Run complexity analysis if enabled
        if options.complexity_analysis {
            match AnalysisRunner::run_complexity_analysis(&file_path) {
                Ok(complexity) => result.complexity_data = complexity,
                Err(e) => return Err(e),
            }
        }

        Ok(result)
    }

    /// Get complexity analysis results.
    pub fn complexity(&self) -> ComplexityResultSet {
        ComplexityResultSet {
            data: &self.complexity_data,
        }
    }

    /// Get CFG analysis results.
    pub fn cfgs(&self) -> CfgResultSet {
        CfgResultSet {
            data: &self.cfg_data,
        }
    }

    /// Get function information.
    pub fn functions(&self) -> FunctionResultSet {
        FunctionResultSet {
            data: &self.function_data,
        }
    }

    /// Get basic block information.
    pub fn basic_blocks(&self) -> BasicBlockResultSet {
        BasicBlockResultSet {
            data: &self.basic_block_data,
        }
    }

    /// Export all results to JSONL format.
    pub fn to_jsonl(&self) -> Result<String, NTreeError> {
        ExportUtils::to_jsonl(&self.cfg_data, &self.basic_block_data, &self.complexity_data)
    }
}