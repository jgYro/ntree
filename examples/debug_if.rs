use ntree::generate_cfgs_v2;
use std::io::Write;
use tempfile::NamedTempFile;

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let code = r#"
fn test_if(x: i32) {
    if x > 0 {
        println!("positive");
    } else {
        println!("non-positive");
    }
    println!("done");
}
"#;

    let mut temp_file = NamedTempFile::new()?;
    temp_file.write_all(code.as_bytes())?;

    let cfgs = generate_cfgs_v2(temp_file.path())?;

    for cfg in cfgs {
        println!("Function: {}", cfg.function_name);
        println!("\nJSONL:");
        for line in cfg.jsonl.lines() {
            println!("  {}", line);
        }

        println!("\nMermaid:");
        println!("{}", cfg.mermaid);

        println!("\nAnalysis:");
        println!("- Nodes with 'if': {}",
            cfg.jsonl.lines()
                .filter(|l| l.contains("\"label\":") && l.contains("if"))
                .count()
        );
        println!("- True edges: {}",
            cfg.jsonl.lines()
                .filter(|l| l.contains("\"kind\":\"true\""))
                .count()
        );
        println!("- False edges: {}",
            cfg.jsonl.lines()
                .filter(|l| l.contains("\"kind\":\"false\""))
                .count()
        );
        println!("- Join nodes: {}",
            cfg.jsonl.lines()
                .filter(|l| l.contains("\"label\":\"join\""))
                .count()
        );
    }

    Ok(())
}