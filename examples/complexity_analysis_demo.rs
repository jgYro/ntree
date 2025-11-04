use ntree::SourceCode;

fn main() -> Result<(), Box<dyn std::error::Error>> {
    println!("=== Complexity Analysis Demo ===\n");

    let test_file = "test_sample.rs";

    // Analyze with focus on complexity
    let analysis = SourceCode::new(test_file)?
        .with_complexity_analysis(true)
        .with_cfg_generation(true)  // Needed for complexity calculation
        .with_basic_blocks(false)
        .analyze()?;

    println!("Analyzing file: {}\n", test_file);

    // Overall statistics
    let complexity = analysis.complexity();
    let total_functions = complexity.len();
    let high_complexity_count = complexity.filter_by_complexity(3).len();

    println!("Total functions: {}", total_functions);
    println!("High complexity (>=3): {}", high_complexity_count);

    // Detailed complexity breakdown
    println!("\nComplexity Breakdown:");
    println!("{:<20} {:<12} {:<15}", "Function", "Complexity", "Unreachable");
    println!("{}", "-".repeat(50));

    for result in analysis.complexity().all() {
        let unreachable_display = if result.unreachable.is_empty() {
            "None".to_string()
        } else {
            format!("{:?}", result.unreachable)
        };

        println!("{:<20} {:<12} {:<15}",
                 result.function,
                 result.cyclomatic,
                 unreachable_display);
    }

    // Functions with unreachable code
    let with_dead_code = analysis.complexity().with_unreachable_code();
    if !with_dead_code.is_empty() {
        println!("\nFunctions with unreachable code:");
        for result in with_dead_code {
            println!("  {}: {:?}", result.function, result.unreachable);
        }
    } else {
        println!("\nNo unreachable code detected.");
    }

    // Complexity recommendations
    println!("\nRecommendations:");
    let high_complexity = analysis.complexity().filter_by_complexity(5);
    if high_complexity.is_empty() {
        println!("✓ All functions have acceptable complexity (< 5)");
    } else {
        println!("⚠ Consider refactoring high complexity functions:");
        for result in high_complexity {
            println!("  - {} (complexity: {})", result.function, result.cyclomatic);
        }
    }

    // Export complexity data to JSONL
    println!("\nComplexity Data (JSONL format):");
    let complexity_jsonl = analysis.complexity().to_jsonl()?;
    for line in complexity_jsonl.lines() {
        println!("  {}", line);
    }

    Ok(())
}