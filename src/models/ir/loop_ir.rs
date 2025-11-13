use serde::{Deserialize, Serialize};

/// Represents different kinds of for loops in a language-agnostic way.
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub enum LoopKind {
    #[serde(rename = "for_counter")]
    ForCounter,
    #[serde(rename = "for_iterator")]
    ForIterator,
}

/// Language-agnostic representation of for loops.
/// Normalizes different for loop constructs across languages.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ForLoopIR {
    #[serde(rename = "type")]
    pub loop_type: String,
    pub loop_id: String,
    pub kind: LoopKind,

    // Counter loop fields (C/JS/Java style: for(init; cond; update))
    #[serde(skip_serializing_if = "Option::is_none")]
    pub init: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub condition: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub update: Option<String>,

    // Iterator loop fields (Rust/Python style: for x in xs)
    #[serde(skip_serializing_if = "Option::is_none")]
    pub pattern: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub iter_expr: Option<String>,
}

impl ForLoopIR {
    /// Create a new counter-style for loop IR.
    pub fn new_counter(loop_id: String, init: String, condition: String, update: String) -> Self {
        ForLoopIR {
            loop_type: "Loop".to_string(),
            loop_id,
            kind: LoopKind::ForCounter,
            init: Some(init),
            condition: Some(condition),
            update: Some(update),
            pattern: None,
            iter_expr: None,
        }
    }

    /// Create a new iterator-style for loop IR.
    pub fn new_iterator(loop_id: String, pattern: String, iter_expr: String) -> Self {
        ForLoopIR {
            loop_type: "Loop".to_string(),
            loop_id,
            kind: LoopKind::ForIterator,
            init: None,
            condition: None,
            update: None,
            pattern: Some(pattern),
            iter_expr: Some(iter_expr),
        }
    }

    /// Convert to JSONL format.
    pub fn to_jsonl(&self) -> String {
        match serde_json::to_string(self) {
            Ok(json) => json,
            Err(_) => "{}".to_string(),
        }
    }
}
