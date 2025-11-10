use crate::core::NTreeError;
use crate::analyzers::ComplexityResult;
use crate::api::analysis::CfgResult;

/// Filtered view of complexity analysis results.
#[derive(Debug)]
pub struct ComplexityResultSet<'a> {
    pub(crate) data: &'a [ComplexityResult],
}

impl<'a> ComplexityResultSet<'a> {
    /// Filter complexity results by minimum cyclomatic complexity.
    pub fn filter_by_complexity(self, min_complexity: u32) -> Vec<&'a ComplexityResult> {
        self.data
            .iter()
            .filter(|result| result.cyclomatic >= min_complexity)
            .collect()
    }

    /// Filter complexity results by function name pattern.
    pub fn filter_by_name(self, pattern: &str) -> Vec<&'a ComplexityResult> {
        self.data
            .iter()
            .filter(|result| result.function.contains(pattern))
            .collect()
    }

    /// Get results with unreachable code.
    pub fn with_unreachable_code(self) -> Vec<&'a ComplexityResult> {
        self.data
            .iter()
            .filter(|result| !result.unreachable.is_empty())
            .collect()
    }

    /// Export to JSONL format.
    pub fn to_jsonl(&self) -> Result<String, NTreeError> {
        let mut jsonl = String::new();
        for result in self.data {
            match serde_json::to_string(result) {
                Ok(json) => {
                    jsonl.push_str(&json);
                    jsonl.push('\n');
                }
                Err(e) => {
                    return Err(NTreeError::ParseError(format!("JSON serialization failed: {}", e)))
                }
            }
        }
        Ok(jsonl)
    }

    /// Get all results as a slice.
    pub fn all(&self) -> &'a [ComplexityResult] {
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

/// Filtered view of CFG analysis results.
#[derive(Debug)]
pub struct CfgResultSet<'a> {
    pub(crate) data: &'a [CfgResult],
}

impl<'a> CfgResultSet<'a> {
    /// Filter CFG results by function name pattern.
    pub fn filter_by_name(self, pattern: &str) -> Vec<&'a CfgResult> {
        self.data
            .iter()
            .filter(|result| result.function_name.contains(pattern))
            .collect()
    }

    /// Get CFG for a specific function.
    pub fn for_function(self, function_name: &str) -> Option<&'a CfgResult> {
        self.data
            .iter()
            .find(|result| result.function_name == function_name)
    }

    /// Export all CFGs to Mermaid format.
    pub fn to_mermaid(&self) -> String {
        self.data
            .iter()
            .map(|cfg| cfg.mermaid.clone())
            .collect::<Vec<_>>()
            .join("\n\n")
    }

    /// Export to JSONL format.
    pub fn to_jsonl(&self) -> String {
        self.data
            .iter()
            .map(|cfg| cfg.jsonl.clone())
            .collect::<Vec<_>>()
            .join("\n")
    }

    /// Get all results as a slice.
    pub fn all(&self) -> &'a [CfgResult] {
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