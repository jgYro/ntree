use serde::{Deserialize, Serialize};

/// Represents a basic block - a sequence of straight-line statements.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct BasicBlock {
    /// Basic block identifier
    pub bb: usize,
    /// List of statements in this basic block
    pub stmts: Vec<String>,
    /// Source code span information
    pub span: String,
}

/// Represents an edge between basic blocks.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct BasicBlockEdge {
    /// Source basic block ID
    pub from: usize,
    /// Target basic block ID
    pub to: usize,
    /// Edge kind (e.g., "true", "false", "next", "back")
    pub kind: String,
}

/// Wrapper for basic block edges to match expected JSON format.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct BasicBlockEdgeWrapper {
    pub edge: BasicBlockEdge,
}

impl BasicBlock {
    /// Create a new basic block.
    pub fn new(id: usize, statements: Vec<String>, span: String) -> Self {
        BasicBlock {
            bb: id,
            stmts: statements,
            span,
        }
    }

    /// Add a statement to this basic block.
    pub fn add_statement(&mut self, stmt: String) {
        self.stmts.push(stmt);
    }

    /// Check if this basic block is empty.
    pub fn is_empty(&self) -> bool {
        self.stmts.is_empty()
    }
}

impl BasicBlockEdge {
    /// Create a new basic block edge.
    pub fn new(from: usize, to: usize, kind: String) -> Self {
        BasicBlockEdge { from, to, kind }
    }
}