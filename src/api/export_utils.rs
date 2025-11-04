use crate::core::NTreeError;
use crate::analyzers::ComplexityResult;
use crate::api::{CfgResult, BasicBlockResult};

/// Utilities for exporting analysis results to various formats.
pub struct ExportUtils;

impl ExportUtils {
    /// Export all analysis results to JSONL format.
    pub fn to_jsonl(
        cfg_data: &[CfgResult],
        basic_block_data: &[BasicBlockResult],
        complexity_data: &[ComplexityResult],
    ) -> Result<String, NTreeError> {
        let mut jsonl = String::new();

        for cfg in cfg_data {
            jsonl.push_str(&cfg.jsonl);
            jsonl.push('\n');
        }

        for block in basic_block_data {
            jsonl.push_str(&block.jsonl);
            jsonl.push('\n');
        }

        for complexity in complexity_data {
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
}