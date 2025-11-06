use ntree::{create_tree_from_file, SourceCode};
use ntree::analyzers::language_specific::python::PythonImportExtractor;

fn main() -> Result<(), Box<dyn std::error::Error>> {
    println!("=== Import/Export Extraction Demo ===");

    let python_file = "test_samples/test_python_imports.py";

    // Test direct AST extraction
    println!("📄 Direct AST Import Extraction:");
    let root = create_tree_from_file(python_file)?;
    let source = std::fs::read_to_string(python_file)?;
    let file_path = std::path::PathBuf::from(python_file);

    let (imports, exports) = PythonImportExtractor::extract_dependencies(root, &source, &file_path)?;

    println!("  Imports found: {}", imports.len());
    for (i, import) in imports.iter().enumerate() {
        println!("    {}. {} -> {} ({})",
            i+1,
            import.source_file.file_name().unwrap().to_string_lossy(),
            import.target_module,
            match import.import_type {
                ntree::ImportType::Module => "module",
                ntree::ImportType::Symbol => "symbol",
                ntree::ImportType::Wildcard => "wildcard",
                ntree::ImportType::Relative => "relative",
            }
        );
        println!("       Syntax: {}", import.import_syntax);
    }

    println!("  Exports found: {}", exports.len());

    // Test integrated workspace analysis with import extraction
    println!("\n📁 Workspace Analysis with Import Tracking:");
    let analysis = SourceCode::new("test_samples/")?
        .search_workspace(true)
        .analyze()?;

    // Export complete dataset including dependency edges
    let dataset = analysis.export_dataset()?;
    let stats = dataset.stats();

    println!("📊 Complete Dataset Statistics:");
    println!("  Files: {}", stats.files);
    println!("  Symbols: {}", stats.symbols);
    println!("  Function facts: {}", stats.function_facts);
    println!("  Import edges: {}", stats.import_edges);
    println!("  Export edges: {}", stats.export_edges);

    // Show structured data export
    println!("\n📋 Structured JSONL Export Sample:");
    let jsonl = analysis.to_dataset_jsonl()?;
    let lines: Vec<&str> = jsonl.lines().take(3).collect();
    for (i, line) in lines.iter().enumerate() {
        println!("  {}: {}", i+1, line);
    }

    if jsonl.lines().count() > 3 {
        println!("  ... and {} more lines", jsonl.lines().count() - 3);
    }

    println!("\n🎯 Import extraction foundation ready!");
    println!("Next: Extract imports from all files during workspace analysis");

    Ok(())
}