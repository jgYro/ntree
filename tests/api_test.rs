use ntree::{items_to_jsonl, list_top_level_items, TopLevelItem};
use std::fs;

#[test]
fn test_list_top_level_items() {
    let test_file = "test_api.rs";
    let content = r#"
pub fn hello() {
    println!("Hello");
}

struct MyStruct {
    field: i32,
}

impl MyStruct {
    fn new() -> Self {
        MyStruct { field: 0 }
    }
}
"#;

    match fs::write(test_file, content) {
        Ok(_) => {}
        Err(e) => panic!("Failed to write test file: {}", e),
    }

    let items = match list_top_level_items(test_file) {
        Ok(i) => i,
        Err(e) => panic!("Failed to list items: {:?}", e),
    };

    assert!(items.len() > 0);

    let has_function = items.iter().any(|item| item.kind == "function_item");
    let has_struct = items.iter().any(|item| item.kind == "struct_item");
    let has_impl = items.iter().any(|item| item.kind == "impl_item");

    assert!(has_function);
    assert!(has_struct);
    assert!(has_impl);

    match fs::remove_file(test_file) {
        Ok(_) => {}
        Err(e) => panic!("Failed to remove test file: {}", e),
    }
}

#[test]
fn test_items_to_jsonl() {
    let items = vec![
        TopLevelItem::new(
            "file1.rs".to_string(),
            "function_item".to_string(),
            Some("func1".to_string()),
            0,
            0,
            5,
            10,
        ),
        TopLevelItem::new(
            "file1.rs".to_string(),
            "struct_item".to_string(),
            Some("Struct1".to_string()),
            10,
            0,
            15,
            20,
        ),
    ];

    let jsonl = match items_to_jsonl(&items) {
        Ok(j) => j,
        Err(e) => panic!("Failed to convert to JSONL: {:?}", e),
    };

    let lines: Vec<&str> = jsonl.lines().collect();
    assert_eq!(lines.len(), 2);

    assert!(lines[0].contains("\"identifier\":\"func1\""));
    assert!(lines[1].contains("\"identifier\":\"Struct1\""));
}