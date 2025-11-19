use ntree::api::{
    functions_to_jsonl, generate_cfg_ir, generate_cfgs, list_functions, list_top_level_items,
};
use ntree::ComplexityAnalyzer;

fn main() -> Result<(), Box<dyn std::error::Error>> {
    println!("=== Legacy API Demo (still supported) ===\n");

    let test_file = "test_sample.rs";

    // List top-level items using legacy API
    println!("1. Top-level items:");
    match list_top_level_items(test_file) {
        Ok(items) => {
            for item in items {
                println!(
                    "   {:?}: {} at {}:{}",
                    item.kind,
                    item.identifier.unwrap_or_else(|| "unnamed".to_string()),
                    item.start_line,
                    item.start_column
                );
            }
        }
        Err(e) => eprintln!("Failed to list top-level items: {:?}", e),
    }

    // List functions using legacy API
    println!("\n2. Functions:");
    match list_functions(test_file) {
        Ok(functions) => match functions_to_jsonl(&functions) {
            Ok(jsonl) => {
                println!("   JSONL format:");
                for line in jsonl.lines() {
                    println!("     {}", line);
                }
            }
            Err(e) => eprintln!("Failed to convert functions to JSONL: {:?}", e),
        },
        Err(e) => eprintln!("Failed to parse functions: {:?}", e),
    }

    // Generate CFGs using legacy API
    println!("\n3. Control Flow Graphs:");
    let cfgs = generate_cfgs(test_file)?;
    for cfg_result in &cfgs {
        println!("   Function: {}", cfg_result.function_name);
        println!("   Mermaid diagram:\n{}", cfg_result.mermaid);
        println!("   ---");
    }

    // Direct complexity analysis using legacy API
    println!("\n4. Complexity Analysis (Direct):");
    let cfg_ir_results = generate_cfg_ir(test_file)?;
    let analyzer = ComplexityAnalyzer::new();

    for cfg_ir in cfg_ir_results {
        match analyzer.analyze(&cfg_ir) {
            Ok(result) => {
                println!(
                    "   {}: complexity {}, unreachable {:?}",
                    result.function, result.cyclomatic, result.unreachable
                );
            }
            Err(e) => eprintln!("   Complexity analysis failed: {}", e),
        }
    }

    println!("\n=== Comparison ===");
    println!("Legacy API: Multiple function calls, manual coordination");
    println!("Modern API: Single call with automatic coordination");
    println!("Both APIs produce equivalent results!");

    Ok(())
}
