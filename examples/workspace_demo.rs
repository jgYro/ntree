use ntree::WorkspaceAnalyzer;

fn main() -> Result<(), Box<dyn std::error::Error>> {
    println!("=== Workspace Analysis Demo ===");

    // Analyze the current ntree codebase as a test
    let mut analyzer = WorkspaceAnalyzer::new("src/");

    println!("Discovering and analyzing files in src/...");

    match analyzer.analyze_workspace() {
        Ok(results) => {
            println!("✅ Workspace analysis completed!");

            // Show file discovery results
            println!("\n📁 Files Discovered:");
            for language in results.languages() {
                let files = results.get_language_files(&language);
                println!("  {}: {} files", language, files.len());

                // Show first few files for each language
                for (i, file) in files.iter().take(3).enumerate() {
                    let relative = file.relative_path("src/").unwrap_or_else(|| file.path.clone());
                    println!("    {}. {} ({})", i + 1, relative.display(), file.size);
                }
                if files.len() > 3 {
                    println!("    ... and {} more files", files.len() - 3);
                }
            }

            // Show analysis statistics
            println!("\n📊 Analysis Statistics:");
            println!("  Total files: {}", results.stats.total_files);
            println!("  Files analyzed: {}", results.stats.files_analyzed);
            println!("  Files cached: {}", results.stats.files_cached);

            // Show symbol store statistics
            let symbol_stats = analyzer.symbol_store().stats();
            println!("\n🔍 Symbol Statistics:");
            println!("  Total symbols: {}", symbol_stats.total_symbols);
            println!("  Total functions: {}", symbol_stats.total_functions);
            println!("  Files with symbols: {}", symbol_stats.total_files);

            // Show some example symbols
            println!("\n📋 Sample Functions Found:");
            let some_symbols = analyzer.symbol_store().find_symbols_by_name("new");
            for (i, symbol) in some_symbols.iter().take(5).enumerate() {
                println!("  {}. {} in {} ({})",
                    i + 1,
                    symbol.name,
                    symbol.file_path.file_name().unwrap_or_default().to_string_lossy(),
                    symbol.span
                );

                if let Some(facts) = analyzer.symbol_store().get_function_facts(&symbol.id) {
                    println!("     Complexity: {}, Body: {}",
                        facts.complexity,
                        facts.body_span.as_ref().unwrap_or(&"unknown".to_string())
                    );
                }
            }

            if some_symbols.len() > 5 {
                println!("  ... and {} more functions with 'new'", some_symbols.len() - 5);
            }

            println!("\n🎯 Cache Performance:");
            println!("  Hit rate: {:.1}%", results.stats.cache_hit_rate);
        }
        Err(e) => {
            println!("❌ Workspace analysis failed: {:?}", e);
        }
    }

    Ok(())
}