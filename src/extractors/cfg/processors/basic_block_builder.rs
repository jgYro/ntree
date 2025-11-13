use super::super::core::{get_statement_text, CfgContext};
use crate::models::{BasicBlock, BasicBlockEdge, BasicBlockGraph};
use tree_sitter::Node;

/// Builder for coalescing straight-line statements into basic blocks.
pub struct BasicBlockBuilder {
    graph: BasicBlockGraph,
    pub current_block: Option<BasicBlock>,
    current_id: usize,
    _ctx: CfgContext,
}

impl BasicBlockBuilder {
    /// Create a new basic block builder.
    pub fn new() -> Self {
        BasicBlockBuilder {
            graph: BasicBlockGraph::new(),
            current_block: None,
            current_id: 0,
            _ctx: CfgContext::new(),
        }
    }

    /// Start a new basic block.
    pub fn start_block(&mut self) -> usize {
        // Finish current block if exists
        self.finish_current_block();

        // Start new block
        let block_id = self.current_id;
        self.current_id += 1;
        self.current_block = Some(BasicBlock::new(block_id, Vec::new(), String::new()));
        block_id
    }

    /// Add a statement to the current basic block.
    pub fn add_statement(&mut self, stmt: Node, source: &str) {
        let stmt_text = get_statement_text(stmt, source);

        match &mut self.current_block {
            Some(block) => {
                block.add_statement(stmt_text);
                // Update span to include this statement
                let start = stmt.start_byte();
                let end = stmt.end_byte();
                block.span = format!("{}:{}", start, end);
            }
            None => {
                // Start a new block if none exists
                let _block_id = self.start_block();
                if let Some(block) = &mut self.current_block {
                    block.add_statement(stmt_text);
                    let start = stmt.start_byte();
                    let end = stmt.end_byte();
                    block.span = format!("{}:{}", start, end);
                }
            }
        }
    }

    /// Check if a statement is a terminator that ends a basic block.
    pub fn is_terminator(stmt: Node) -> bool {
        matches!(
            stmt.kind(),
            "if_expression"
                | "while_expression"
                | "for_expression"
                | "match_expression"
                | "return_expression"
                | "break_expression"
                | "continue_expression"
                | "loop_expression"
        )
    }

    /// Add an edge between basic blocks.
    pub fn add_edge(&mut self, from: usize, to: usize, kind: String) {
        self.graph.add_edge(BasicBlockEdge::new(from, to, kind));
    }

    /// Finish the current basic block and add it to the graph.
    pub fn finish_current_block(&mut self) {
        if let Some(block) = self.current_block.take() {
            if !block.is_empty() {
                self.graph.add_block(block);
            }
        }
    }

    /// Build the final basic block graph.
    pub fn build(mut self) -> BasicBlockGraph {
        self.finish_current_block();
        self.graph
    }

    /// Get the current basic block ID.
    pub fn current_block_id(&self) -> Option<usize> {
        self.current_block.as_ref().map(|block| block.bb)
    }
}
