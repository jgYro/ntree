use ntree::{WorkspaceAnalyzer, SymbolSearcher};

fn main() -> Result<(), Box<dyn std::error::Error>> {
    println!("=== Regex Symbol Search Demo ===");

    // Analyze the current ntree codebase
    let mut analyzer = WorkspaceAnalyzer::new("src/");
    let _results = analyzer.analyze_workspace()?;

    let symbol_store = analyzer.symbol_store();

    // 1. Find exact constructors (only functions named "new")
    println!("🔧 Constructor Functions (exact match for 'new'):");
    let constructors = symbol_store.find_symbols_exact("new");
    for constructor in &constructors {
        println!("  {} in {}", constructor.name,
                constructor.file_path.file_name().unwrap().to_string_lossy());
    }

    // 2. Find all symbols containing "new" (including new_iterator, etc.)
    println!("\n📝 All symbols containing 'new' (substring):");
    let new_related = symbol_store.find_symbols_by_name("new");
    for symbol in &new_related {
        println!("  {} in {}", symbol.name,
                symbol.file_path.file_name().unwrap().to_string_lossy());
    }

    // 3. Use regex to find exact constructors (^new$)
    println!("\n🎯 Exact constructors using regex (^new$):");
    match SymbolSearcher::find_symbols_regex(symbol_store, "^new$") {
        Ok(exact_constructors) => {
            for constructor in exact_constructors {
                if let Some(facts) = symbol_store.get_function_facts(&constructor.id) {
                    println!("  {} in {} - complexity: {}",
                            constructor.name,
                            constructor.file_path.file_name().unwrap().to_string_lossy(),
                            facts.complexity);
                } else {
                    println!("  {} in {}",
                            constructor.name,
                            constructor.file_path.file_name().unwrap().to_string_lossy());
                }
            }
        }
        Err(e) => println!("  Regex error: {}", e),
    }

    // 4. Use regex to find getter methods (^get_\w+)
    println!("\n📥 Getter functions (^get_\\w+):");
    match SymbolSearcher::find_symbols_regex(symbol_store, r"^get_\w+") {
        Ok(getters) => {
            for getter in getters.iter().take(5) {
                println!("  {} in {}",
                        getter.name,
                        getter.file_path.file_name().unwrap().to_string_lossy());
            }
            if getters.len() > 5 {
                println!("  ... and {} more getters", getters.len() - 5);
            }
        }
        Err(e) => println!("  Regex error: {}", e),
    }

    // 5. Use regex to find test functions (containing "test")
    println!("\n🧪 Test functions (.*test.*):");
    match SymbolSearcher::find_symbols_regex(symbol_store, r".*test.*") {
        Ok(test_functions) => {
            for test_fn in test_functions.iter().take(3) {
                println!("  {} in {}",
                        test_fn.name,
                        test_fn.file_path.file_name().unwrap().to_string_lossy());
            }
            if test_functions.len() > 3 {
                println!("  ... and {} more test functions", test_functions.len() - 3);
            }
        }
        Err(e) => println!("  Regex error: {}", e),
    }

    println!("\n📊 Summary:");
    println!("  Exact 'new': {} functions", constructors.len());
    println!("  Contains 'new': {} symbols", new_related.len());

    let stats = symbol_store.stats();
    println!("  Total symbols: {}", stats.total_symbols);
    println!("  Total functions: {}", stats.total_functions);

    Ok(())
}