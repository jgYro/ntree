use ntree::SourceCode;

fn main() -> Result<(), Box<dyn std::error::Error>> {
    println!("=== Workspace Analysis Demo ===");

    // Analyze the current ntree codebase using unified API
    let analysis = SourceCode::new("src/")?.search_workspace(true).analyze()?;

    println!("Discovering and analyzing files in src/...");

    println!("✅ Workspace analysis completed!");

    // Show file discovery results
    if let Some(files_by_lang) = analysis.files_by_language() {
        println!("\n📁 Files Discovered:");
        for (language, files) in files_by_lang {
            println!("  {}: {} files", language, files.len());
        }
    }

    // Show symbol statistics
    println!("\n🔍 Symbol Statistics:");
    println!("  Total symbols: {}", analysis.symbol_count());

    if let Some(stats) = analysis.workspace_stats() {
        println!("  Total files: {}", stats.total_files);
        println!("  Languages: {}", stats.languages);
    }

    // Show some example symbols
    println!("\n📋 Sample Functions Found:");
    let symbol_search = analysis.symbols().named("new").regex(false);
    let some_symbols = symbol_search.search()?;

    for (i, symbol) in some_symbols.iter().take(5).enumerate() {
        println!(
            "  {}. {} in {} ({})",
            i + 1,
            symbol.name,
            symbol
                .file_path
                .file_name()
                .map(|n| n.to_string_lossy())
                .unwrap_or("unknown".into()),
            symbol.span
        );
    }

    Ok(())
}
