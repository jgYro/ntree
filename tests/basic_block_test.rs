use ntree::{generate_basic_blocks, generate_cfgs};
use std::io::Write;
use tempfile::NamedTempFile;

/// Test basic block coalescing - straight-line statements should be grouped.
/// Verifies that node count drops compared to statement-per-node CFG.
#[test]
fn test_basic_block_coalescing() {
    let code = r#"
fn test_coalescing() {
    let mut acc = 0;
    acc += 1;
    acc += 2;
    acc += 3;
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

    // Generate both CFG and Basic Blocks
    let cfgs = match generate_cfgs(temp_file.path()) {
        Ok(c) => c,
        Err(e) => panic!("Failed to generate CFGs: {:?}", e),
    };

    let basic_blocks = match generate_basic_blocks(temp_file.path()) {
        Ok(bb) => bb,
        Err(e) => panic!("Failed to generate basic blocks: {:?}", e),
    };

    assert_eq!(cfgs.len(), 1);
    assert_eq!(basic_blocks.len(), 1);

    let cfg = &cfgs[0];
    let bb_result = &basic_blocks[0];

    println!("CFG JSONL:");
    println!("{}", cfg.jsonl);
    println!("\nBasic Block JSONL:");
    println!("{}", bb_result.jsonl);

    // Count CFG nodes vs Basic Blocks
    let cfg_node_count = cfg.jsonl.matches("\"cfg_node\":").count();
    let bb_block_count = bb_result.jsonl.matches("\"bb\":").count();

    // Basic blocks should have fewer nodes (coalesced statements)
    assert!(bb_block_count < cfg_node_count);
    println!("CFG nodes: {}, Basic blocks: {}", cfg_node_count, bb_block_count);

    // Verify basic block contains multiple statements
    assert!(bb_result.jsonl.contains("\"stmts\":["));
    assert!(bb_result.jsonl.contains("\"let mut acc = 0;\""));
    assert!(bb_result.jsonl.contains("\"acc += 1;\""));
}

/// Test that terminators properly end basic blocks.
#[test]
fn test_terminator_blocks() {
    let code = r#"
fn test_terminators() {
    let x = 1;
    let y = 2;
    if x > y {
        return x;
    } else {
        return y;
    }
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

    let basic_blocks = match generate_basic_blocks(temp_file.path()) {
        Ok(bb) => bb,
        Err(e) => panic!("Failed to generate basic blocks: {:?}", e),
    };

    assert_eq!(basic_blocks.len(), 1);
    let bb_result = &basic_blocks[0];

    println!("Terminator Basic Block JSONL:");
    println!("{}", bb_result.jsonl);

    // Should have at least entry and exit blocks
    let bb_count = bb_result.jsonl.matches("\"bb\":").count();
    assert!(bb_count >= 2); // At least: entry block and exit block

    // Should contain control flow statement
    assert!(bb_result.jsonl.contains("\"if x > y"));

    // Verify spans are included
    assert!(bb_result.jsonl.contains("\"span\":"));
}

/// Test that empty blocks are handled correctly.
#[test]
fn test_empty_function_basic_blocks() {
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

    let basic_blocks = match generate_basic_blocks(temp_file.path()) {
        Ok(bb) => bb,
        Err(e) => panic!("Failed to generate basic blocks: {:?}", e),
    };

    assert_eq!(basic_blocks.len(), 1);
    let bb_result = &basic_blocks[0];

    println!("Empty function Basic Block JSONL:");
    println!("{}", bb_result.jsonl);

    // Should have at least entry and exit blocks
    let bb_count = bb_result.jsonl.matches("\"bb\":").count();
    assert!(bb_count >= 2);

    // Should contain ENTRY and EXIT
    assert!(bb_result.jsonl.contains("\"ENTRY\""));
    assert!(bb_result.jsonl.contains("\"EXIT\""));
}