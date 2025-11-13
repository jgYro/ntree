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
    /// Data source for this node (e.g., "tree-sitter", "compiler", "lsp")
    pub source: String,
    /// Confidence level ("exact", "inferred", "uncertain")
    pub confidence: String,
}

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
    /// Data source for this edge (e.g., "tree-sitter", "compiler", "lsp")
    pub source: String,
    /// Confidence level ("exact", "inferred", "uncertain")
    pub confidence: String,
}

/// Complete IR representation for a function's CFG.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct FunctionCFGIR {
    /// Function identifier
    pub function_name: String,
    /// All CFG nodes in this function
    pub nodes: Vec<CFGNodeIR>,
    /// All CFG edges in this function
    pub edges: Vec<CFGEdgeIR>,
    /// Source file information
    pub source_file: Option<String>,
}

impl CFGNodeIR {
    /// Create a new language-neutral CFG node with tree-sitter provenance.
    pub fn new(func: String, id: String, label: String, span: String) -> Self {
        CFGNodeIR {
            node_type: "CFGNode".to_string(),
            func,
            id,
            label,
            span,
            source: "tree-sitter".to_string(),
            confidence: "exact".to_string(),
        }
    }

    /// Create a new CFG node with custom provenance and confidence.
    pub fn with_provenance(
        func: String,
        id: String,
        label: String,
        span: String,
        source: String,
        confidence: String,
    ) -> Self {
        CFGNodeIR {
            node_type: "CFGNode".to_string(),
            func,
            id,
            label,
            span,
            source,
            confidence,
        }
    }
}

impl CFGEdgeIR {
    /// Create a new language-neutral CFG edge with tree-sitter provenance.
    pub fn new(func: String, from: String, to: String, kind: String) -> Self {
        CFGEdgeIR {
            edge_type: "CFGEdge".to_string(),
            func,
            from,
            to,
            kind,
            source: "tree-sitter".to_string(),
            confidence: "exact".to_string(),
        }
    }

    /// Create a new CFG edge with custom provenance and confidence.
    pub fn with_provenance(
        func: String,
        from: String,
        to: String,
        kind: String,
        source: String,
        confidence: String,
    ) -> Self {
        CFGEdgeIR {
            edge_type: "CFGEdge".to_string(),
            func,
            from,
            to,
            kind,
            source,
            confidence,
        }
    }
}

impl FunctionCFGIR {
    /// Create a new function CFG IR.
    pub fn new(function_name: String, source_file: Option<String>) -> Self {
        FunctionCFGIR {
            function_name,
            nodes: Vec::new(),
            edges: Vec::new(),
            source_file,
        }
    }

    /// Add a node to this function's CFG.
    pub fn add_node(&mut self, node: CFGNodeIR) {
        self.nodes.push(node);
    }

    /// Add an edge to this function's CFG.
    pub fn add_edge(&mut self, edge: CFGEdgeIR) {
        self.edges.push(edge);
    }

    /// Convert to JSONL format with stable schema.
    pub fn to_jsonl(&self) -> String {
        let mut jsonl = String::new();

        // Serialize all nodes
        for node in &self.nodes {
            match serde_json::to_string(node) {
                Ok(json) => {
                    jsonl.push_str(&json);
                    jsonl.push('\n');
                }
                Err(_) => continue,
            }
        }

        // Serialize all edges
        for edge in &self.edges {
            match serde_json::to_string(edge) {
                Ok(json) => {
                    jsonl.push_str(&json);
                    jsonl.push('\n');
                }
                Err(_) => continue,
            }
        }

        jsonl
    }

    /// Count nodes in this CFG.
    pub fn node_count(&self) -> usize {
        self.nodes.len()
    }

    /// Count edges in this CFG.
    pub fn edge_count(&self) -> usize {
        self.edges.len()
    }
}
