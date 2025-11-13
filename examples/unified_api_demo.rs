use ntree::SourceCode;

fn main() -> Result<(), Box<dyn std::error::Error>> {
    println!("=== Unified SourceCode API Demo ===\n");

    // 1. Single file analysis (existing functionality)
    println!("📄 Single File Analysis:");
    let single_analysis = SourceCode::new("test_sample.rs")?
        .with_complexity_analysis(true)
        .analyze()?;

    println!("  Functions: {}", single_analysis.functions().len());
    println!("  Complexity: {}", single_analysis.complexity().len());
    println!("  CFGs: {}", single_analysis.cfgs().len());
    println!("  Symbols: {}", single_analysis.symbol_count());

    // 2. Workspace analysis (new unified functionality)
    println!("\n📁 Workspace Analysis:");
    let workspace_analysis = SourceCode::new("src/")?
        .search_workspace(true)
        .with_complexity_analysis(true)
        .analyze()?;

    println!(
        "  Mode: {}",
        if workspace_analysis.is_workspace_mode() {
            "Workspace"
        } else {
            "Single File"
        }
    );

    if let Some(files_by_lang) = workspace_analysis.files_by_language() {
        for (lang, files) in files_by_lang {
            println!("  {}: {} files", lang, files.len());
        }
    }

    if let Some(stats) = workspace_analysis.workspace_stats() {
        println!("  Total files: {}", stats.total_files);
        println!("  Total size: {} bytes", stats.total_size);
        println!("  Languages: {}", stats.languages);
    }

    // 3. Symbol search with fluent API (parameterized search)
    println!("\n🔍 Symbol Search Examples:");

    // Exact constructor search with proper lifetimes
    let search1 = workspace_analysis
        .symbols()
        .named("new")
        .regex(true)
        .kind("function");
    let exact_constructors = search1.search()?;

    println!("  Exact 'new' constructors: {}", exact_constructors.len());

    // Getter functions search
    let search2 = workspace_analysis.symbols().named("get_").regex(true);
    let getters = search2.search()?;

    println!("  Getter functions: {}", getters.len());

    // Functions in specific file
    let search3 = workspace_analysis.symbols().in_file("cfg").kind("function");
    let cfg_functions = search3.search()?;

    println!("  Functions in cfg files: {}", cfg_functions.len());

    // Show some example results
    println!("\n📋 Sample Symbol Search Results:");
    for (i, symbol) in exact_constructors.iter().take(3).enumerate() {
        println!(
            "  {}. {} in {}",
            i + 1,
            symbol.name,
            symbol.file_path.file_name().unwrap().to_string_lossy()
        );
    }

    println!("\n🎯 All working through single SourceCode API!");

    Ok(())
}
