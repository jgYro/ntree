use ntree::{create_tree_from_file, NTreeError};
use std::fs;

#[test]
fn test_create_tree_from_rust_file() {
    let test_file = "test_parser.rs";
    let content = "fn hello() { println!(\"world\"); }";

    match fs::write(test_file, content) {
        Ok(_) => {}
        Err(e) => panic!("Failed to write test file: {}", e),
    }

    let node = match create_tree_from_file(test_file) {
        Ok(n) => n,
        Err(e) => panic!("Failed to create tree from file: {:?}", e),
    };

    assert_eq!(node.kind(), "source_file");
    assert!(node.child_count() > 0);

    match fs::remove_file(test_file) {
        Ok(_) => {}
        Err(e) => panic!("Failed to remove test file: {}", e),
    }
}

#[test]
fn test_create_tree_from_nonexistent_file() {
    let result = create_tree_from_file("nonexistent.rs");
    assert!(result.is_err());
    match result {
        Err(NTreeError::IoError(_)) => {}
        _ => panic!("Expected IoError for nonexistent file"),
    }
}