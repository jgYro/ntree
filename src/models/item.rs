use serde::{Deserialize, Serialize};

#[derive(Debug, Serialize, Deserialize, PartialEq)]
pub struct TopLevelItem {
    pub file: String,
    pub kind: String,
    pub identifier: Option<String>,
    pub start_line: usize,
    pub start_column: usize,
    pub end_line: usize,
    pub end_column: usize,
}

impl TopLevelItem {
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