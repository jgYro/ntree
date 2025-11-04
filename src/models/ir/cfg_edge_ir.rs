use serde::{Deserialize, Serialize};

/// Language-neutral CFG edge representation.
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub struct CFGEdgeIR {
    #[serde(rename = "type")]
    pub edge_type: String,
    /// Function this edge belongs to
    pub func: String,
    /// Source node identifier
    pub from: String,
    /// Target node identifier
    pub to: String,
    /// Edge kind (e.g., "true", "false", "next", "error", "exception")
    pub kind: String,
}

impl CFGEdgeIR {
    /// Create a new language-neutral CFG edge.
    pub fn new(func: String, from: String, to: String, kind: String) -> Self {
        CFGEdgeIR {
            edge_type: "CFGEdge".to_string(),
            func,
            from,
            to,
            kind,
        }
    }
}