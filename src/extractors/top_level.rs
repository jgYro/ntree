use crate::models::TopLevelItem;
use tree_sitter::Node;

/// Extracts all top-level items from a parsed syntax tree.
///
/// # Arguments
/// * `file_path` - Path to the source file
/// * `root_node` - Root node of the syntax tree
/// * `source` - Source code content
///
/// # Returns
/// List of top-level items with their types, names, and locations
pub fn extract_top_level_items(
    file_path: &str,
    root_node: Node,
    source: &str,
) -> Vec<TopLevelItem> {
    let mut items = Vec::new();
    let mut cursor = root_node.walk();

    for child in root_node.named_children(&mut cursor) {
        let kind = child.kind().to_string();
        let identifier = extract_identifier(child, source);

        let start_pos = child.start_position();
        let end_pos = child.end_position();

        let item = TopLevelItem::new(
            file_path.to_string(),
            kind,
            identifier,
            start_pos.row,
            start_pos.column,
            end_pos.row,
            end_pos.column,
        );

        items.push(item);
    }

    items
}

/// Extracts the identifier name from a node if it exists.
fn extract_identifier(node: Node, source: &str) -> Option<String> {
    let mut cursor = node.walk();

    for child in node.named_children(&mut cursor) {
        match child.kind() {
            "identifier" | "type_identifier" => {
                let start = child.start_byte();
                let end = child.end_byte();
                return Some(source[start..end].to_string());
            }
            _ => {}
        }
    }

    None
}
