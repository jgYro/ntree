use crate::core::NTreeError;
use crate::models::{FunctionSpan, TopLevelItem};

/// Converts items to JSONL format with one line per item.
pub fn items_to_jsonl(items: &[TopLevelItem]) -> Result<String, NTreeError> {
    let mut jsonl = String::new();

    for item in items {
        match serde_json::to_string(item) {
            Ok(json) => {
                jsonl.push_str(&json);
                jsonl.push('\n');
            }
            Err(e) => {
                return Err(NTreeError::ParseError(format!(
                    "Failed to serialize item: {}",
                    e
                )))
            }
        }
    }

    Ok(jsonl)
}

/// Converts functions to JSONL format with one line per function.
pub fn functions_to_jsonl(functions: &[FunctionSpan]) -> Result<String, NTreeError> {
    let mut jsonl = String::new();

    for function in functions {
        match serde_json::to_string(function) {
            Ok(json) => {
                jsonl.push_str(&json);
                jsonl.push('\n');
            }
            Err(e) => {
                return Err(NTreeError::ParseError(format!(
                    "Failed to serialize function: {}",
                    e
                )))
            }
        }
    }

    Ok(jsonl)
}
