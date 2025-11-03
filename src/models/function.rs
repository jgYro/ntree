use serde::{Deserialize, Serialize};

/// Represents a function with its full span and body span.
///
/// # Coordinate System
/// - Internally uses Tree-sitter's 0-based coordinates
/// - Output strings use 1-based coordinates for human readability
///
/// # Body Span
/// The body span includes the opening and closing braces `{` and `}`.
#[derive(Debug, Serialize, Deserialize, PartialEq)]
pub struct FunctionSpan {
    pub function: String,
    pub span: String,
    pub body: Option<String>,
}

impl FunctionSpan {
    pub fn new(function: String, span: String, body: Option<String>) -> Self {
        FunctionSpan {
            function,
            span,
            body,
        }
    }

    /// Formats a span from 0-based coordinates to 1-based human-readable format.
    ///
    /// # Arguments
    /// * `start_line` - 0-based starting line number
    /// * `start_col` - 0-based starting column number
    /// * `end_line` - 0-based ending line number
    /// * `end_col` - 0-based ending column number
    ///
    /// # Returns
    /// A string in format "line:col–line:col" using 1-based coordinates
    pub fn format_span(start_line: usize, start_col: usize, end_line: usize, end_col: usize) -> String {
        format!("{}:{}–{}:{}", start_line + 1, start_col + 1, end_line + 1, end_col + 1)
    }
}