use ntree::SourceCode;

fn main() -> Result<(), Box<dyn std::error::Error>> {
    println!("=== Complete Data Export Demo ===");

    // Test single file export
    println!("\n📄 Single File Data Export:");
    let file_analysis = SourceCode::new("test_samples/test_python_classes.py")?
        .with_complexity_analysis(true)
        .analyze()?;

    let file_dataset = file_analysis.export_dataset()?;
    let file_stats = file_dataset.stats();

    println!("  Files: {}", file_stats.files);
    println!("  Symbols: {}", file_stats.symbols);
    println!("  Function facts: {}", file_stats.function_facts);
    println!("  Import edges: {}", file_stats.import_edges);
    println!("  Export edges: {}", file_stats.export_edges);

    // Test workspace export
    println!("\n📁 Workspace Data Export:");
    let workspace_analysis = SourceCode::new("src/")?
        .search_workspace(true)
        .with_complexity_analysis(true)
        .analyze()?;

    let workspace_dataset = workspace_analysis.export_dataset()?;
    let workspace_stats = workspace_dataset.stats();

    println!("  Files: {}", workspace_stats.files);
    println!("  Symbols: {}", workspace_stats.symbols);
    println!("  Function facts: {}", workspace_stats.function_facts);
    println!("  Import edges: {}", workspace_stats.import_edges);
    println!("  Export edges: {}", workspace_stats.export_edges);

    // Test JSONL export
    println!("\n📊 JSONL Export Sample:");
    let jsonl = file_analysis.to_dataset_jsonl()?;
    let lines: Vec<&str> = jsonl.lines().take(5).collect();
    for (i, line) in lines.iter().enumerate() {
        println!("  {}: {}", i+1, line);
    }

    if jsonl.lines().count() > 5 {
        println!("  ... and {} more lines", jsonl.lines().count() - 5);
    }

    println!("\n🎯 Data Export Schema:");
    println!("  ✅ File records with metadata");
    println!("  ✅ Symbols with qualified names");
    println!("  ✅ Function facts with complexity");
    println!("  ⏳ Import/Export edges (next step)");

    // Test constructor detection in both modes
    println!("\n🔍 Constructor Detection Verification:");

    // Single file constructors
    let file_constructors = file_analysis.symbols()
        .named("^(__init__|__new__|new)$")
        .regex(true);
    let file_constructor_results = file_constructors.search()?;
    println!("  Single file constructors: {}", file_constructor_results.len());

    // Workspace constructors
    let workspace_constructors = workspace_analysis.symbols()
        .named("^(__init__|__new__|new)$")
        .regex(true);
    let workspace_constructor_results = workspace_constructors.search()?;
    println!("  Workspace constructors: {}", workspace_constructor_results.len());

    Ok(())
}