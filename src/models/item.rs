use serde::{Deserialize, Serialize};

/// Represents a top-level item in a source file.
///
/// Top-level items include functions, structs, enums, traits, etc.
/// Coordinates are 0-based internally but displayed as 1-based in output.
#[derive(Debug, Serialize, Deserialize, PartialEq)]
pub struct TopLevelItem {
    /// Path to the source file
    pub file: String,
    /// Kind of item (e.g., "function_item", "struct_item")
    pub kind: String,
    /// Optional identifier name
    pub identifier: Option<String>,
    /// Starting line (0-based)
    pub start_line: usize,
    /// Starting column (0-based)
    pub start_column: usize,
    /// Ending line (0-based)
    pub end_line: usize,
    /// Ending column (0-based)
    pub end_column: usize,
}

impl TopLevelItem {
    /// Creates a new TopLevelItem.
    ///
    /// # Arguments
    /// * `file` - Path to the source file
    /// * `kind` - Type of the item
    /// * `identifier` - Optional name of the item
    /// * `start_line` - Starting line (0-based)
    /// * `start_column` - Starting column (0-based)
    /// * `end_line` - Ending line (0-based)
    /// * `end_column` - Ending column (0-based)
    pub fn new(
        file: String,
        kind: String,
        identifier: Option<String>,
        start_line: usize,
        start_column: usize,
        end_line: usize,
        end_column: usize,
    ) -> Self {
        TopLevelItem {
            file,
            kind,
            identifier,
            start_line,
            start_column,
            end_line,
            end_column,
        }
    }
}
