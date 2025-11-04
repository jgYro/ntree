use ntree::SourceCode;

fn main() -> Result<(), Box<dyn std::error::Error>> {
    println!("=== ntree Modern API Demo ===\n");

    // Use the sample file for demonstration
    let test_file = "test_sample.rs";

    // Simple analysis with all features enabled
    println!("1. Complete Analysis:");
    let analysis = SourceCode::new(test_file)?.analyze()?;

    println!("   Functions found: {}", analysis.functions().len());
    println!("   CFGs generated: {}", analysis.cfgs().len());
    println!("   Complexity results: {}", analysis.complexity().len());

    // Show function names
    println!("\n   Function names:");
    for name in analysis.functions().names() {
        println!("     - {}", name);
    }

    // Show complexity analysis
    println!("\n2. Complexity Analysis:");
    for result in analysis.complexity().all() {
        println!("   {}: complexity {} ", result.function, result.cyclomatic);
        if !result.unreachable.is_empty() {
            println!("     Unreachable blocks: {:?}", result.unreachable);
        }
    }

    // Selective analysis
    println!("\n3. Selective Analysis (CFG only):");
    let cfg_only = SourceCode::new(test_file)?
        .with_complexity_analysis(false)
        .with_cfg_generation(true)
        .with_basic_blocks(false)
        .analyze()?;

    println!("   CFGs: {}, Complexity: {}",
             cfg_only.cfgs().len(),
             cfg_only.complexity().len());

    // Show CFG for specific function
    if let Some(cfg) = cfg_only.cfgs().for_function("complex_function") {
        println!("\n4. CFG for 'complex_function':");
        println!("   Mermaid diagram:\n{}", cfg.mermaid);
    }

    // Filter high complexity functions
    println!("\n5. High Complexity Functions (>= 2):");
    let high_complexity = analysis.complexity().filter_by_complexity(2);
    for result in high_complexity {
        println!("   {}: {}", result.function, result.cyclomatic);
    }

    // Export to JSONL
    println!("\n6. JSONL Export Sample:");
    let jsonl = analysis.to_jsonl()?;
    let lines: Vec<&str> = jsonl.lines().take(3).collect();
    for line in lines {
        println!("   {}", line);
    }
    if jsonl.lines().count() > 3 {
        println!("   ... ({} more lines)", jsonl.lines().count() - 3);
    }

    Ok(())
}