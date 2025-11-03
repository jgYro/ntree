use ntree::generate_cfgs;
use std::io::Write;
use tempfile::NamedTempFile;

fn main() -> Result<(), Box<dyn std::error::Error>> {
    // Example with early return
    let code = r#"
fn process_data(x: i32) -> i32 {
    if x < 0 {
        return -1;  // Early return - connects to EXIT with "exit" edge
    }
    let result = x * 2;
    println!("Processed: {}", result);
    return result;      // Normal return - also uses "exit" edge
    let dead_code = 0;  // This won't appear in CFG (unreachable)
}
"#;

    let mut temp_file = NamedTempFile::new()?;
    temp_file.write_all(code.as_bytes())?;

    let cfgs = generate_cfgs(temp_file.path())?;

    for cfg in cfgs {
        println!("Function: {}\n", cfg.function_name);

        println!("JSONL Output:");
        println!("{}", cfg.jsonl);

        println!("\nMermaid Diagram:");
        println!("{}", cfg.mermaid);

        println!("\nKey features:");
        println!("- Return statements connect to EXIT with 'exit' edge type");
        println!("- Code after return is not included (dead code elimination)");
        println!("- Exit edges shown with dotted arrows in Mermaid");
    }

    Ok(())
}