use super::super::core::is_statement_node;
use super::basic_block_builder::BasicBlockBuilder;
use super::terminator_handler::process_terminator;
use crate::models::BasicBlockGraph;
use tree_sitter::Node;

/// Process a function block to create basic blocks by coalescing straight-line statements.
pub fn build_basic_blocks_from_block(block_node: Node, source: &str) -> BasicBlockGraph {
    let mut builder = BasicBlockBuilder::new();

    // Start with entry block
    let entry_id = builder.start_block();
    if let Some(current_block) = builder.current_block.as_mut() {
        current_block.stmts.push("ENTRY".to_string());
        current_block.span = "entry".to_string();
    }

    // Process statements and accumulate into blocks
    process_block_statements(&mut builder, block_node, source, entry_id);

    // Add exit block
    let _exit_id = builder.start_block();
    if let Some(current_block) = builder.current_block.as_mut() {
        current_block.stmts.push("EXIT".to_string());
        current_block.span = "exit".to_string();
    }

    builder.build()
}

/// Process statements in a block, accumulating straight-line statements into basic blocks.
fn process_block_statements(
    builder: &mut BasicBlockBuilder,
    block: Node,
    source: &str,
    mut current_block_id: usize,
) -> usize {
    let mut cursor = block.walk();

    for child in block.named_children(&mut cursor) {
        if !is_statement_node(child) {
            continue;
        }

        // Check if this statement is a terminator
        if BasicBlockBuilder::is_terminator(child) {
            // Finish current block before processing terminator
            builder.finish_current_block();

            // Process the terminator (creates new blocks and edges)
            current_block_id = process_terminator(builder, child, source, current_block_id);
        } else {
            // Regular statement - add to current block
            // Start new block if none exists
            if builder.current_block_id().is_none() {
                current_block_id = builder.start_block();
            }

            builder.add_statement(child, source);
        }
    }

    current_block_id
}
