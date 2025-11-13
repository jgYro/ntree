use ntree::{items_to_jsonl, list_top_level_items};
use std::fs;

#[test]
fn test_full_example() {
    // Create a test Rust file
    let test_file = "example.rs";
    let content = r#"use std::collections::HashMap;

pub fn process_data(input: &str) -> String {
    input.to_uppercase()
}

pub struct DataProcessor {
    cache: HashMap<String, String>,
}

impl DataProcessor {
    pub fn new() -> Self {
        DataProcessor {
            cache: HashMap::new(),
        }
    }
}

mod tests {
    use super::*;

    #[test]
    fn test_process() {
        assert_eq!(process_data("hello"), "HELLO");
    }
}"#;

    match fs::write(test_file, content) {
        Ok(_) => {}
        Err(e) => panic!("Failed to write test file: {}", e),
    }

    // Parse and list top-level items
    let items = match list_top_level_items(test_file) {
        Ok(i) => i,
        Err(e) => panic!("Failed to list items: {:?}", e),
    };

    // Convert to JSONL
    let jsonl = match items_to_jsonl(&items) {
        Ok(j) => j,
        Err(e) => panic!("Failed to convert to JSONL: {:?}", e),
    };

    // Verify we found expected items
    println!("Found {} top-level items:", items.len());
    for item in &items {
        println!(
            "  - {} {:?} at line {}",
            item.kind, item.identifier, item.start_line
        );
    }

    println!("\nJSONL output:\n{}", jsonl);

    // Assertions
    assert!(items.len() >= 4); // use, function, struct, impl
    assert!(jsonl.lines().count() == items.len());

    // Clean up
    match fs::remove_file(test_file) {
        Ok(_) => {}
        Err(e) => panic!("Failed to remove test file: {}", e),
    }
}
