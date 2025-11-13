use ntree::SourceCode;

fn main() -> Result<(), Box<dyn std::error::Error>> {
    println!("=== Regex Symbol Search Demo ===");

    // Analyze the current ntree codebase using unified API
    let analysis = SourceCode::new("src/")?.search_workspace(true).analyze()?;

    // 1. Find exact constructors using unified API
    println!("🔧 Constructor Functions (exact match for 'new'):");
    let exact_search = analysis.symbols().named("new").regex(true);
    let exact_constructors = exact_search.search()?;

    for constructor in exact_constructors.iter().take(5) {
        println!(
            "  {} in {}",
            constructor.name,
            constructor
                .file_path
                .file_name()
                .map(|n| n.to_string_lossy())
                .unwrap_or("unknown".into())
        );
    }

    // 2. Find getter functions
    println!("\n📥 Getter functions (^get_\\w+):");
    let getter_search = analysis.symbols().named("^get_\\w+").regex(true);
    let getters = getter_search.search()?;

    for getter in getters.iter().take(3) {
        println!(
            "  {} in {}",
            getter.name,
            getter
                .file_path
                .file_name()
                .map(|n| n.to_string_lossy())
                .unwrap_or("unknown".into())
        );
    }

    println!("\n📊 Summary:");
    println!("  Total symbols: {}", analysis.symbol_count());

    if let Some(stats) = analysis.workspace_stats() {
        println!("  Total files: {}", stats.total_files);
        println!("  Languages: {}", stats.languages);
    }

    Ok(())
}
