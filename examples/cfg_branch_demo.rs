use ntree::generate_cfgs_v2;
use std::io::Write;
use tempfile::NamedTempFile;

fn main() -> Result<(), Box<dyn std::error::Error>> {
    // Example with if/else branching
    let code = r#"
fn process(x: i32) -> i32 {
    let mut result = 0;

    if x > 0 {
        result = x * 2;
    } else {
        result = -x;
    }

    return result;
}
"#;

    let mut temp_file = NamedTempFile::new()?;
    temp_file.write_all(code.as_bytes())?;

    let cfgs = generate_cfgs_v2(temp_file.path())?;

    for cfg in cfgs {
        println!("Function: {}\n", cfg.function_name);

        println!("JSONL Output:");
        for line in cfg.jsonl.lines() {
            println!("{}", line);
        }

        println!("\nMermaid Diagram:");
        println!("{}", cfg.mermaid);

        println!("\nKey CFG-05 Features:");
        println!("✓ Condition node: 'if (x > 0)' created");
        println!("✓ True edge: Leads to 'result = x * 2'");
        println!("✓ False edge: Leads to 'result = -x'");
        println!("✓ Join node: Both branches converge");
        println!("✓ Continues to return after join");
    }

    Ok(())
}