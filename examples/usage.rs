use ntree::SourceCode;

fn main() -> Result<(), Box<dyn std::error::Error>> {
    println!("=== ntree Usage Example ===\n");

    // Use test sample file that has actual functions
    let test_file = "test_sample.rs";

    // Modern unified API - recommended approach
    println!("Using Modern API:");
    let analysis = SourceCode::new(test_file)?.analyze()?;

    // Show basic information
    println!("  Functions: {}", analysis.functions().len());
    println!("  CFGs: {}", analysis.cfgs().len());
    println!("  Complexity results: {}", analysis.complexity().len());

    // Show function details
    println!("\nFunction Details:");
    for func in analysis.functions().all() {
        println!("  - {}: {}", func.function, func.span);
    }

    // Show complexity results
    println!("\nComplexity Analysis:");
    for result in analysis.complexity().all() {
        println!("  {}: complexity {}", result.function, result.cyclomatic);
    }

    // Show CFG for first function
    if let Some(cfg) = analysis.cfgs().all().first() {
        println!("\nCFG Example ({}): ", cfg.function_name);
        println!("{}", cfg.mermaid);
    }

    // Export everything to JSONL
    println!("\nJSONL Export (first 3 lines):");
    let jsonl = analysis.to_jsonl()?;
    for (i, line) in jsonl.lines().take(3).enumerate() {
        println!("  {}: {}", i + 1, line);
    }

    Ok(())
}