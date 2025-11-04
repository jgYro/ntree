use crate::models::FunctionSpan;
use crate::api::BasicBlockResult;

/// Filtered view of function analysis results.
#[derive(Debug)]
pub struct FunctionResultSet<'a> {
    pub(super) data: &'a [FunctionSpan],
}

impl<'a> FunctionResultSet<'a> {
    /// Filter functions by name pattern.
    pub fn filter_by_name(self, pattern: &str) -> Vec<&'a FunctionSpan> {
        self.data
            .iter()
            .filter(|func| func.function.contains(pattern))
            .collect()
    }

    /// Get function by exact name.
    pub fn find_by_name(self, name: &str) -> Option<&'a FunctionSpan> {
        self.data.iter().find(|func| func.function == name)
    }

    /// Get all function names.
    pub fn names(&self) -> Vec<&str> {
        self.data.iter().map(|func| func.function.as_str()).collect()
    }

    /// Get all results as a slice.
    pub fn all(&self) -> &'a [FunctionSpan] {
        self.data
    }

    /// Get number of functions.
    pub fn len(&self) -> usize {
        self.data.len()
    }

    /// Check if empty.
    pub fn is_empty(&self) -> bool {
        self.data.is_empty()
    }
}

/// Filtered view of basic block analysis results.
#[derive(Debug)]
pub struct BasicBlockResultSet<'a> {
    pub(super) data: &'a [BasicBlockResult],
}

impl<'a> BasicBlockResultSet<'a> {
    /// Filter basic blocks by function name pattern.
    pub fn filter_by_name(self, pattern: &str) -> Vec<&'a BasicBlockResult> {
        self.data
            .iter()
            .filter(|result| result.function_name.contains(pattern))
            .collect()
    }

    /// Get basic blocks for a specific function.
    pub fn for_function(self, function_name: &str) -> Option<&'a BasicBlockResult> {
        self.data
            .iter()
            .find(|result| result.function_name == function_name)
    }

    /// Export to JSONL format.
    pub fn to_jsonl(&self) -> String {
        self.data
            .iter()
            .map(|block| block.jsonl.clone())
            .collect::<Vec<_>>()
            .join("\n")
    }

    /// Get all results as a slice.
    pub fn all(&self) -> &'a [BasicBlockResult] {
        self.data
    }

    /// Get number of results.
    pub fn len(&self) -> usize {
        self.data.len()
    }

    /// Check if empty.
    pub fn is_empty(&self) -> bool {
        self.data.is_empty()
    }
}