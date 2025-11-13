use super::basic_block::{BasicBlock, BasicBlockEdge, BasicBlockEdgeWrapper};

/// Represents a complete basic block graph.
#[derive(Debug, Clone)]
pub struct BasicBlockGraph {
    pub blocks: Vec<BasicBlock>,
    pub edges: Vec<BasicBlockEdge>,
}

impl BasicBlockGraph {
    /// Create a new empty basic block graph.
    pub fn new() -> Self {
        BasicBlockGraph {
            blocks: Vec::new(),
            edges: Vec::new(),
        }
    }

    /// Add a basic block to the graph.
    pub fn add_block(&mut self, block: BasicBlock) {
        self.blocks.push(block);
    }

    /// Add an edge to the graph.
    pub fn add_edge(&mut self, edge: BasicBlockEdge) {
        self.edges.push(edge);
    }

    /// Convert to JSONL format.
    pub fn to_jsonl(&self) -> String {
        let mut jsonl = String::new();

        // Add basic blocks
        for block in &self.blocks {
            match serde_json::to_string(block) {
                Ok(json) => {
                    jsonl.push_str(&json);
                    jsonl.push('\n');
                }
                Err(_) => continue,
            }
        }

        // Add edges
        for edge in &self.edges {
            let wrapper = BasicBlockEdgeWrapper { edge: edge.clone() };
            match serde_json::to_string(&wrapper) {
                Ok(json) => {
                    jsonl.push_str(&json);
                    jsonl.push('\n');
                }
                Err(_) => continue,
            }
        }

        jsonl
    }
}
