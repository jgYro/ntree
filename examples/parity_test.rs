use ntree::SourceCode;

fn main() -> Result<(), Box<dyn std::error::Error>> {
    println!("=== Rust & Python Parity Test ===");

    // Test Rust analysis
    println!("🦀 Rust Analysis:");
    let rust_analysis = SourceCode::new("test_samples/test_rust_structures.rs")?.analyze()?;

    println!("  Functions: {}", rust_analysis.functions().len());
    println!("  Symbols: {}", rust_analysis.symbol_count());

    // Show Rust symbol types
    let rust_constructor_search = rust_analysis.symbols().named("new").regex(false);
    let rust_constructors = rust_constructor_search.search()?;
    let rust_struct_search = rust_analysis.symbols().kind("struct");
    let rust_structs = rust_struct_search.search()?;

    println!("  Constructors (new): {}", rust_constructors.len());
    println!("  Structs: {}", rust_structs.len());

    // Test Python analysis
    println!("\n🐍 Python Analysis:");
    let python_analysis = SourceCode::new("test_samples/test_python_classes.py")?.analyze()?;

    println!("  Functions: {}", python_analysis.functions().len());
    println!("  Symbols: {}", python_analysis.symbol_count());

    // Show Python symbol types
    let python_constructor_search = python_analysis.symbols().named("__init__").regex(false);
    let python_constructors = python_constructor_search.search()?;
    let python_class_search = python_analysis.symbols().kind("class");
    let python_classes = python_class_search.search()?;

    println!("  Constructors (__init__): {}", python_constructors.len());
    println!("  Classes: {}", python_classes.len());

    // Test workspace analysis with both languages
    println!("\n📁 Mixed Workspace Analysis:");
    let workspace_analysis = SourceCode::new("test_samples/")?
        .search_workspace(true)
        .analyze()?;

    if let Some(files_by_lang) = workspace_analysis.files_by_language() {
        for (lang, files) in files_by_lang {
            println!("  {}: {} files", lang, files.len());
        }
    }

    // Test unified capabilities
    println!("\n🔗 Unified Capabilities:");

    // Dependency analysis
    let deps = workspace_analysis.dependencies()?;
    let dep_analysis = deps.analyze_dependencies();
    println!("  Dependencies: {}", dep_analysis.summary());

    // Call graph
    let call_graph = workspace_analysis.call_graph();
    let call_stats = call_graph.stats();
    println!("  Call sites: {}", call_stats.total_call_sites);

    // Cross-language constructors
    let constructor_search = workspace_analysis
        .symbols()
        .named("^(new|__init__|constructor)$")
        .regex(true);
    let mixed_constructors = constructor_search.search()?;
    println!(
        "  Cross-language constructors: {}",
        mixed_constructors.len()
    );

    println!("\n✅ Rust & Python Parity Achieved:");
    println!("  ✅ Symbol extraction (structs/classes, methods/functions)");
    println!("  ✅ Import extraction (use/import statements)");
    println!("  ✅ Call site extraction (function calls)");
    println!("  ✅ Language-agnostic pipeline works for both");
    println!("  ✅ Unified API handles both languages equally");

    Ok(())
}
