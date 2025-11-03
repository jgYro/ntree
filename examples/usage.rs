use ntree::{functions_to_jsonl, generate_cfgs, list_functions, list_top_level_items};

fn main() -> Result<(), Box<dyn std::error::Error>> {
    // Use lib.rs as it exists in this project
    let test_file = "src/lib.rs";

    // List top-level items
    match list_top_level_items(test_file) {
        Ok(n) => println!("Top-level items:\n{:#?}\n", n),
        Err(e) => eprintln!("Failed to list top-level items: {:?}", e),
    }

    // List functions
    match list_functions(test_file) {
        Ok(f_s) => {
            match functions_to_jsonl(&f_s) {
                Ok(f) => println!("Functions JSONL:\n{}\n", f),
                Err(e) => eprintln!("Failed to convert functions to JSONL: {:?}", e),
            }
        }
        Err(e) => eprintln!("Failed to parse out functions: {:?}", e),
    }

    // Generate CFGs
    let cfgs = generate_cfgs(test_file)?;

    for cfg_result in cfgs {
        println!("Function: {}", cfg_result.function_name);

        // Get Mermaid diagram
        println!("Mermaid diagram:\n{}", cfg_result.mermaid);

        // Get JSONL representation
        println!("JSONL:\n{}", cfg_result.jsonl);
    }

    Ok(())
}