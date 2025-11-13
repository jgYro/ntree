use ntree::api::generate_cfgs;
use std::io::Write;
use tempfile::NamedTempFile;

#[test]
fn test_simple_two_statement_function() {
    let code = r#"
fn calculate() {
    let acc = 0;
    return acc;
}
"#;

    let mut temp_file = match NamedTempFile::with_suffix(".rs") {
        Ok(f) => f,
        Err(_) => panic!("Failed to create temp file"),
    };

    match temp_file.write_all(code.as_bytes()) {
        Ok(_) => {}
        Err(_) => panic!("Failed to write to temp file"),
    }

    let cfgs = match generate_cfgs(temp_file.path()) {
        Ok(c) => c,
        Err(e) => panic!("Failed to generate CFGs: {:?}", e),
    };

    assert_eq!(cfgs.len(), 1);
    let cfg = &cfgs[0];
    assert_eq!(cfg.function_name, "calculate");

    // Parse JSONL to count nodes and edges
    let lines: Vec<&str> = cfg.jsonl.lines().collect();

    // Count nodes (should be 4: ENTRY, let acc = 0, return acc, EXIT)
    let node_count = lines
        .iter()
        .filter(|line| line.contains("\"cfg_node\""))
        .count();
    assert_eq!(node_count, 4);

    // Count edges (should be 3: ENTRY->s1, s1->s2, s2->EXIT with exit edge)
    let edge_count = lines
        .iter()
        .filter(|line| line.contains("\"cfg_edge\""))
        .count();
    assert_eq!(edge_count, 3);

    // Verify specific nodes
    assert!(cfg.jsonl.contains("\"label\":\"ENTRY\""));
    assert!(cfg.jsonl.contains("\"label\":\"let acc = 0;\""));
    assert!(cfg.jsonl.contains("\"label\":\"return acc;\""));
    assert!(cfg.jsonl.contains("\"label\":\"EXIT\""));

    // Verify Mermaid output contains expected elements
    assert!(cfg.mermaid.contains("graph TD"));
    assert!(cfg.mermaid.contains("0([ENTRY])"));
    assert!(cfg.mermaid.contains("([EXIT])"));
    assert!(cfg.mermaid.contains("0 --> 1"));
    // Return statement now uses exit edge
    assert!(cfg.mermaid.contains("-.->") || cfg.mermaid.contains("exit"));
}

#[test]
fn test_empty_function() {
    let code = r#"
fn empty() {
}
"#;

    let mut temp_file = match NamedTempFile::with_suffix(".rs") {
        Ok(f) => f,
        Err(_) => panic!("Failed to create temp file"),
    };

    match temp_file.write_all(code.as_bytes()) {
        Ok(_) => {}
        Err(_) => panic!("Failed to write to temp file"),
    }

    let cfgs = match generate_cfgs(temp_file.path()) {
        Ok(c) => c,
        Err(e) => panic!("Failed to generate CFGs: {:?}", e),
    };

    assert_eq!(cfgs.len(), 1);
    let cfg = &cfgs[0];
    assert_eq!(cfg.function_name, "empty");

    // Parse JSONL to count nodes and edges
    let lines: Vec<&str> = cfg.jsonl.lines().collect();

    // Count nodes (should be 2: ENTRY, EXIT)
    let node_count = lines
        .iter()
        .filter(|line| line.contains("\"cfg_node\""))
        .count();
    assert_eq!(node_count, 2);

    // Count edges (should be 1: ENTRY->EXIT)
    let edge_count = lines
        .iter()
        .filter(|line| line.contains("\"cfg_edge\""))
        .count();
    assert_eq!(edge_count, 1);
}

#[test]
fn test_multiple_statements() {
    let code = r#"
fn process(x: i32) {
    let mut result = x;
    result = result * 2;
    result = result + 10;
    println!("Result: {}", result);
    return result;
}
"#;

    let mut temp_file = match NamedTempFile::with_suffix(".rs") {
        Ok(f) => f,
        Err(_) => panic!("Failed to create temp file"),
    };

    match temp_file.write_all(code.as_bytes()) {
        Ok(_) => {}
        Err(_) => panic!("Failed to write to temp file"),
    }

    let cfgs = match generate_cfgs(temp_file.path()) {
        Ok(c) => c,
        Err(e) => panic!("Failed to generate CFGs: {:?}", e),
    };

    assert_eq!(cfgs.len(), 1);
    let cfg = &cfgs[0];

    // Parse JSONL to count nodes
    let lines: Vec<&str> = cfg.jsonl.lines().collect();

    // Count nodes (should be 7: ENTRY + 5 statements + EXIT)
    let node_count = lines
        .iter()
        .filter(|line| line.contains("\"cfg_node\""))
        .count();
    assert_eq!(node_count, 7);

    // Count edges (should be 6: connecting all nodes linearly)
    let edge_count = lines
        .iter()
        .filter(|line| line.contains("\"cfg_edge\""))
        .count();
    assert_eq!(edge_count, 6);
}

#[test]
fn test_jsonl_edge_format() {
    let code = r#"
fn simple() {
    let x = 1;
}
"#;

    let mut temp_file = match NamedTempFile::with_suffix(".rs") {
        Ok(f) => f,
        Err(_) => panic!("Failed to create temp file"),
    };

    match temp_file.write_all(code.as_bytes()) {
        Ok(_) => {}
        Err(_) => panic!("Failed to write to temp file"),
    }

    let cfgs = match generate_cfgs(temp_file.path()) {
        Ok(c) => c,
        Err(e) => panic!("Failed to generate CFGs: {:?}", e),
    };

    let cfg = &cfgs[0];

    // Verify edge format matches spec (at least has next edge from ENTRY)
    assert!(cfg
        .jsonl
        .contains("\"cfg_edge\":{\"from\":0,\"to\":1,\"kind\":\"next\"}"));
}
