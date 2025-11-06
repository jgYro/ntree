use ntree::generate_cfgs_v2;
use std::io::Write;
use tempfile::NamedTempFile;

#[test]
fn test_simple_if_with_else() {
    let code = r#"
fn check_value(x: i32) {
    if x > 0 {
        println!("positive");
    } else {
        println!("non-positive");
    }
    println!("done");
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

    let cfgs = match generate_cfgs_v2(temp_file.path()) {
        Ok(c) => c,
        Err(e) => panic!("Failed to generate CFGs: {:?}", e),
    };

    assert_eq!(cfgs.len(), 1);
    let cfg = &cfgs[0];

    println!("JSONL:\n{}", cfg.jsonl);
    println!("\nMermaid:\n{}", cfg.mermaid);

    // Check for condition node
    assert!(cfg.jsonl.contains("\"label\":\"if (x > 0)\""));

    // Check for true and false edges
    assert!(cfg.jsonl.contains("\"kind\":\"true\""));
    assert!(cfg.jsonl.contains("\"kind\":\"false\""));

    // Check for join node
    assert!(cfg.jsonl.contains("\"label\":\"join\""));

    // Check Mermaid has branch labels
    assert!(cfg.mermaid.contains("-->|T|"));
    assert!(cfg.mermaid.contains("-->|F|"));
}

#[test]
fn test_if_without_else() {
    let code = r#"
fn check_positive(x: i32) {
    let mut result = 0;
    if x > 0 {
        result = 1;
    }
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

    let cfgs = match generate_cfgs_v2(temp_file.path()) {
        Ok(c) => c,
        Err(e) => panic!("Failed to generate CFGs: {:?}", e),
    };

    let cfg = &cfgs[0];

    println!("JSONL:\n{}", cfg.jsonl);

    // Check for condition node
    assert!(cfg.jsonl.contains("\"label\":\"if (x > 0)\""));

    // Check for true edge
    assert!(cfg.jsonl.contains("\"kind\":\"true\""));

    // Check for join node (where false path meets after if)
    assert!(cfg.jsonl.contains("\"label\":\"join\""));

    // Check that we have edges going to the join node
    // Find the join node ID first
    let join_node_line = cfg.jsonl.lines()
        .find(|line| line.contains("\"label\":\"join\""))
        .expect("Should have a join node");

    // Extract the node ID from the join node
    let join_id: usize = join_node_line
        .split("\"cfg_node\":")
        .nth(1)
        .and_then(|s| s.split(',').next())
        .and_then(|s| s.parse().ok())
        .expect("Should parse join node ID");

    // Count edges going TO the join node
    let to_join_count = cfg.jsonl.lines()
        .filter(|line| line.contains(&format!("\"to\":{}", join_id)))
        .count();

    // Should have at least 2 edges to join (true path and false path)
    assert!(to_join_count >= 2, "Expected at least 2 edges to join node, got {}", to_join_count);
}

#[test]
fn test_if_else_if_chain() {
    let code = r#"
fn classify(x: i32) -> &'static str {
    if x > 0 {
        return "positive";
    } else if x < 0 {
        return "negative";
    } else {
        return "zero";
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

    let cfgs = match generate_cfgs_v2(temp_file.path()) {
        Ok(c) => c,
        Err(e) => panic!("Failed to generate CFGs: {:?}", e),
    };

    let cfg = &cfgs[0];

    println!("JSONL:\n{}", cfg.jsonl);

    // Should have multiple condition nodes
    assert!(cfg.jsonl.contains("\"label\":\"if (x > 0)\""));
    assert!(cfg.jsonl.contains("\"label\":\"if (x < 0)\""));

    // Should have true and false edges
    let true_count = cfg
        .jsonl
        .lines()
        .filter(|line| line.contains("\"kind\":\"true\""))
        .count();
    assert!(true_count >= 2);

    let false_count = cfg
        .jsonl
        .lines()
        .filter(|line| line.contains("\"kind\":\"false\""))
        .count();
    assert!(false_count >= 1);

    // All paths return, so they should have exit edges
    let exit_count = cfg
        .jsonl
        .lines()
        .filter(|line| line.contains("\"kind\":\"exit\""))
        .count();
    assert_eq!(exit_count, 3);
}

#[test]
fn test_nested_if() {
    let code = r#"
fn nested_check(x: i32, y: i32) {
    if x > 0 {
        if y > 0 {
            println!("both positive");
        } else {
            println!("x positive, y not");
        }
    } else {
        println!("x not positive");
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

    let cfgs = match generate_cfgs_v2(temp_file.path()) {
        Ok(c) => c,
        Err(e) => panic!("Failed to generate CFGs: {:?}", e),
    };

    let cfg = &cfgs[0];

    println!("JSONL:\n{}", cfg.jsonl);
    println!("\nMermaid:\n{}", cfg.mermaid);

    // Should have two condition nodes
    assert!(cfg.jsonl.contains("\"label\":\"if (x > 0)\""));
    assert!(cfg.jsonl.contains("\"label\":\"if (y > 0)\""));

    // Should have multiple join nodes
    let join_count = cfg
        .jsonl
        .lines()
        .filter(|line| line.contains("\"label\":\"join\""))
        .count();
    assert!(join_count >= 1);
}

#[test]
fn test_if_with_return_in_branches() {
    let code = r#"
fn early_exit(x: i32) -> i32 {
    if x < 0 {
        return -1;
    } else {
        println!("processing");
    }
    return x * 2;
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

    let cfgs = match generate_cfgs_v2(temp_file.path()) {
        Ok(c) => c,
        Err(e) => panic!("Failed to generate CFGs: {:?}", e),
    };

    let cfg = &cfgs[0];

    println!("JSONL:\n{}", cfg.jsonl);

    // Check condition
    assert!(cfg.jsonl.contains("\"label\":\"if (x < 0)\""));

    // True branch should have return with exit edge
    assert!(cfg.jsonl.contains("\"label\":\"return -1;\""));
    assert!(cfg.jsonl.contains("\"kind\":\"exit\""));

    // False branch continues to join
    assert!(cfg.jsonl.contains("\"kind\":\"false\""));

    // Final return
    assert!(cfg.jsonl.contains("\"label\":\"return x * 2;\""));
}