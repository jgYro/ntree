use ntree::create_tree_from_file;
use std::fs;

#[test]
fn test_create_tree_from_file() {
    let test_file = "test_integration.rs";
    let content = "fn main() { println!(\"Hello\"); }";

    match fs::write(test_file, content) {
        Ok(_) => {}
        Err(e) => panic!("Failed to write test file: {}", e),
    }

    let root = match create_tree_from_file(test_file) {
        Ok(r) => r,
        Err(e) => panic!("Failed to create tree from file: {:?}", e),
    };

    assert_eq!(root.kind(), "source_file");

    match fs::remove_file(test_file) {
        Ok(_) => {}
        Err(e) => panic!("Failed to remove test file: {}", e),
    }
}
