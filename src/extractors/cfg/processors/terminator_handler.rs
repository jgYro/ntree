use super::basic_block_builder::BasicBlockBuilder;
use tree_sitter::Node;

/// Handle terminator statements (control flow constructs) in basic block processing.
pub fn process_terminator(
    builder: &mut BasicBlockBuilder,
    stmt: Node,
    source: &str,
    current_block_id: usize,
) -> usize {
    match stmt.kind() {
        "return_expression" => {
            // Create a block for the return statement
            let return_block_id = builder.start_block();
            builder.add_statement(stmt, source);
            builder.finish_current_block();

            // Add edge from current to return block
            builder.add_edge(current_block_id, return_block_id, "next".to_string());

            return_block_id
        }
        "if_expression" | "while_expression" | "for_expression" | "match_expression" => {
            // For now, treat control flow as a single block
            // In a full implementation, we'd recursively process branches
            let control_block_id = builder.start_block();
            builder.add_statement(stmt, source);
            builder.finish_current_block();

            // Add edge from current to control block
            builder.add_edge(current_block_id, control_block_id, "next".to_string());

            control_block_id
        }
        "break_expression" | "continue_expression" => {
            // Create block for break/continue
            let control_block_id = builder.start_block();
            builder.add_statement(stmt, source);
            builder.finish_current_block();

            // Add edge from current to control block
            builder.add_edge(current_block_id, control_block_id, "next".to_string());

            control_block_id
        }
        _ => {
            // Unknown terminator, treat as regular statement
            if builder.current_block_id().is_none() {
                builder.start_block();
            }
            builder.add_statement(stmt, source);
            current_block_id
        }
    }
}