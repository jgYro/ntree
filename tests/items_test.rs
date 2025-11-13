use ntree::TopLevelItem;

#[test]
fn test_top_level_item_creation() {
    let item = TopLevelItem::new(
        "test.rs".to_string(),
        "function_item".to_string(),
        Some("my_func".to_string()),
        10,
        5,
        15,
        20,
    );

    assert_eq!(item.file, "test.rs");
    assert_eq!(item.kind, "function_item");
    assert_eq!(item.identifier, Some("my_func".to_string()));
    assert_eq!(item.start_line, 10);
    assert_eq!(item.start_column, 5);
    assert_eq!(item.end_line, 15);
    assert_eq!(item.end_column, 20);
}

#[test]
fn test_top_level_item_serialization() {
    let item = TopLevelItem::new(
        "test.rs".to_string(),
        "struct_item".to_string(),
        Some("MyStruct".to_string()),
        1,
        0,
        5,
        10,
    );

    let json = match serde_json::to_string(&item) {
        Ok(j) => j,
        Err(e) => panic!("Failed to serialize: {}", e),
    };

    assert!(json.contains("\"file\":\"test.rs\""));
    assert!(json.contains("\"kind\":\"struct_item\""));
    assert!(json.contains("\"identifier\":\"MyStruct\""));
}
