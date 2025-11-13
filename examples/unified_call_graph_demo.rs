use ntree::SourceCode;

fn main() -> Result<(), Box<dyn std::error::Error>> {
    println!("=== Unified API Call Graph Demo ===");

    // Single entry point for everything
    let analysis = SourceCode::new("test_samples/")?
        .search_workspace(true)
        .with_complexity_analysis(true)
        .analyze()?;

    println!("📊 Complete Analysis Results:");
    println!(
        "  Files: {}",
        analysis.files_by_language().map(|f| f.len()).unwrap_or(0)
    );
    println!("  Symbols: {}", analysis.symbol_count());

    // All functionality through single result object
    println!("\n🔍 Symbol Analysis:");
    let constructors = analysis.symbols().named("^(new|__init__)$").regex(true);
    let constructor_results = constructors.search()?;
    println!("  Constructors found: {}", constructor_results.len());

    // Call graph through unified API
    println!("\n📞 Call Graph Analysis:");
    let call_graph = analysis.call_graph();
    let call_stats = call_graph.stats();
    println!("  Total call sites: {}", call_stats.total_call_sites);
    println!("  Direct calls: {}", call_stats.direct_calls);
    println!("  Dynamic calls: {}", call_stats.dynamic_calls);

    // Dependency graph through unified API
    println!("\n🔗 Dependency Analysis:");
    match analysis.dependencies() {
        Ok(dep_graph) => {
            let dep_analysis = dep_graph.analyze_dependencies();
            println!("  {}", dep_analysis.summary());

            if let Some(order) = dep_analysis.processing_order() {
                println!("  Linking order: {} modules", order.len());
            }
        }
        Err(e) => println!("  Dependency error: {:?}", e),
    }

    // Name resolution through unified API
    println!("\n🏷️ Name Resolution:");
    if let Some(resolver) = analysis.name_bindings() {
        let mappings = resolver.get_import_mappings();
        println!("  Import mappings: {} files", mappings.len());
    } else {
        println!("  Name resolver not available (need export data)");
    }

    // Complete dataset export
    println!("\n📄 Data Export:");
    let dataset = analysis.export_dataset()?;
    let stats = dataset.stats();
    println!("  Complete dataset ready:");
    println!("    Files: {}", stats.files);
    println!("    Symbols: {}", stats.symbols);
    println!("    Import edges: {}", stats.import_edges);
    println!("    Export edges: {}", stats.export_edges);

    println!("\n🎉 Single SourceCode API provides everything!");
    println!("  ✅ Symbol search and extraction");
    println!("  ✅ Call graph construction");
    println!("  ✅ Dependency analysis");
    println!("  ✅ Name resolution");
    println!("  ✅ Complete data export");

    Ok(())
}
