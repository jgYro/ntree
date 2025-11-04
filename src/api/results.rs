use std::path::PathBuf;
use crate::core::NTreeError;
use crate::models::FunctionSpan;
use crate::analyzers::{ComplexityResult, ComplexityAnalyzer};
use crate::api::{CfgResult, BasicBlockResult, generate_cfg_ir};
use super::options::AnalysisOptions;
use super::result_sets::{ComplexityResultSet, CfgResultSet};
use super::function_results::{FunctionResultSet, BasicBlockResultSet};

/// Complete analysis results for a source file.
#[derive(Debug)]
pub struct AnalysisResult {
    file_path: PathBuf,
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
            file_path: file_path.clone(),
            complexity_data: Vec::new(),
            cfg_data: Vec::new(),
            basic_block_data: Vec::new(),
            function_data: Vec::new(),
        };

        // Get function list first (needed for other analyses)
        match crate::api::list_functions(&file_path) {
            Ok(functions) => result.function_data = functions,
            Err(e) => return Err(e),
        }

        // Run CFG generation if enabled
        if options.cfg_generation {
            match crate::api::generate_cfgs(&file_path) {
                Ok(cfgs) => result.cfg_data = cfgs,
                Err(e) => return Err(e),
            }
        }

        // Run basic block generation if enabled
        if options.basic_blocks {
            match crate::api::generate_basic_blocks(&file_path) {
                Ok(blocks) => result.basic_block_data = blocks,
                Err(e) => return Err(e),
            }
        }

        // Run complexity analysis if enabled
        if options.complexity_analysis {
            result.run_complexity_analysis()?;
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
        let mut jsonl = String::new();

        for cfg in &self.cfg_data {
            jsonl.push_str(&cfg.jsonl);
            jsonl.push('\n');
        }

        for block in &self.basic_block_data {
            jsonl.push_str(&block.jsonl);
            jsonl.push('\n');
        }

        for complexity in &self.complexity_data {
            match serde_json::to_string(complexity) {
                Ok(json) => {
                    jsonl.push_str(&json);
                    jsonl.push('\n');
                }
                Err(e) => return Err(NTreeError::ParseError(format!("JSON serialization failed: {}", e))),
            }
        }

        Ok(jsonl)
    }

    /// Run complexity analysis on CFG IR data.
    fn run_complexity_analysis(&mut self) -> Result<(), NTreeError> {
        // Generate CFG IR data if needed for complexity analysis
        let cfg_ir_results = match generate_cfg_ir(&self.file_path) {
            Ok(results) => results,
            Err(e) => return Err(e),
        };

        let analyzer = ComplexityAnalyzer::new();
        for cfg_ir in cfg_ir_results {
            match analyzer.analyze(&cfg_ir) {
                Ok(result) => self.complexity_data.push(result),
                Err(e) => return Err(NTreeError::ParseError(format!("Complexity analysis failed: {}", e))),
            }
        }

        Ok(())
    }
}