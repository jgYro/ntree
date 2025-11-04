use serde::{Deserialize, Serialize};

/// Language-neutral CFG node representation.
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub struct CFGNodeIR {
    #[serde(rename = "type")]
    pub node_type: String,
    /// Function this node belongs to (e.g., "server::run")
    pub func: String,
    /// Unique node identifier within function
    pub id: String,
    /// Language-agnostic node label
    pub label: String,
    /// Source code span information
    pub span: String,
}

impl CFGNodeIR {
    /// Create a new language-neutral CFG node.
    pub fn new(func: String, id: String, label: String, span: String) -> Self {
        CFGNodeIR {
            node_type: "CFGNode".to_string(),
            func,
            id,
            label,
            span,
        }
    }
}