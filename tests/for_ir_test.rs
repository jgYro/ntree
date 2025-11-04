use ntree::{ForLoopIR, LoopKind};

/// Test IR normalization for Rust iterator-style for loop
#[test]
fn test_rust_for_iterator_ir() {
    // Test creating iterator-style IR directly
    let iterator_ir = ForLoopIR::new_iterator(
        "L1".to_string(),
        "item".to_string(),
        "items".to_string(),
    );

    // Verify IR structure
    assert_eq!(iterator_ir.loop_type, "Loop");
    assert_eq!(iterator_ir.loop_id, "L1");
    assert_eq!(iterator_ir.kind, LoopKind::ForIterator);

    // Verify iterator fields
    assert_eq!(iterator_ir.pattern, Some("item".to_string()));
    assert_eq!(iterator_ir.iter_expr, Some("items".to_string()));

    // Verify counter fields are None
    assert_eq!(iterator_ir.init, None);
    assert_eq!(iterator_ir.condition, None);
    assert_eq!(iterator_ir.update, None);

    // Test JSONL output
    let jsonl = iterator_ir.to_jsonl();
    assert!(jsonl.contains("\"type\":\"Loop\""));
    assert!(jsonl.contains("\"loop_id\":\"L1\""));
    assert!(jsonl.contains("\"kind\":\"for_iterator\""));
    assert!(jsonl.contains("\"pattern\":\"item\""));
    assert!(jsonl.contains("\"iter_expr\":\"items\""));
}

/// Test that counter-style fields are properly handled
#[test]
fn test_counter_style_ir() {
    // Test creating counter-style IR directly
    let counter_ir = ForLoopIR::new_counter(
        "L2".to_string(),
        "i = 0".to_string(),
        "i < 10".to_string(),
        "i++".to_string(),
    );

    assert_eq!(counter_ir.kind, LoopKind::ForCounter);
    assert_eq!(counter_ir.init, Some("i = 0".to_string()));
    assert_eq!(counter_ir.condition, Some("i < 10".to_string()));
    assert_eq!(counter_ir.update, Some("i++".to_string()));

    // Iterator fields should be None
    assert_eq!(counter_ir.pattern, None);
    assert_eq!(counter_ir.iter_expr, None);

    // Test JSONL output
    let jsonl = counter_ir.to_jsonl();
    assert!(jsonl.contains("\"kind\":\"for_counter\""));
    assert!(jsonl.contains("\"init\":\"i = 0\""));
    assert!(jsonl.contains("\"condition\":\"i < 10\""));
    assert!(jsonl.contains("\"update\":\"i++\""));
}