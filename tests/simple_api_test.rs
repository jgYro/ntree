use ntree::{SourceCode, AnalysisResult};
use tempfile::TempDir;
use std::fs;

/// Test the simple, clean public API.
#[test]
fn test_simple_api_usage() {
    let temp_dir = TempDir::new().expect("Failed to create temp dir");
    let test_file = temp_dir.path().join("test.rs");

    fs::write(&test_file, r#"
fn main() {
    println!("Hello, world!");
    let result = calculate(5, 10);
    process_data(result);
}

fn calculate(a: i32, b: i32) -> i32 {
    if a > b {
        a + b
    } else {
        a * b
    }
}

fn process_data(value: i32) {
    match value {
        x if x > 50 => println!("Large: {}", x),
        x if x > 10 => println!("Medium: {}", x),
        _ => println!("Small"),
    }
}
"#).expect("Failed to write test file");

    // Simple API: just SourceCode::new() and analyze()
    let analysis = SourceCode::new(&test_file)
        .expect("Failed to create SourceCode")
        .with_incremental_analysis(true)
        .with_advanced_call_resolution(true)
        .with_external_library_analysis(true)
        .analyze()
        .expect("Analysis failed");

    // Access results through unified interface
    println!("Functions found: {}", analysis.functions().len());

    // Complexity analysis
    let high_complexity = analysis.complexity().filter_by_complexity(3);
    println!("High complexity functions: {}", high_complexity.len());

    // CFG analysis
    let cfgs = analysis.cfgs();
    println!("CFGs generated: {}", cfgs.len());

    // Advanced features through result methods
    let interproc = analysis.interprocedural();
    let stats = interproc.call_stats();
    println!("Call graph: {} functions, {} call sites",
             stats.total_functions, stats.total_call_sites);

    let incremental = analysis.incremental();
    let metrics = incremental.performance_metrics();
    println!("Performance: {} total functions, cache hit ratio: {:.1}%",
             metrics.total_functions, incremental.cache_hit_ratio() * 100.0);

    let external = analysis.external_libraries();
    let libs = external.referenced_libraries();
    println!("External libraries: {:?}", libs);

    // Security analysis
    let security = external.security_analysis();
    println!("Security: {} sources, {} sinks",
             security.taint_sources.len(), security.taint_sinks.len());
}

#[test]
fn test_basic_api_still_works() {
    let temp_dir = TempDir::new().expect("Failed to create temp dir");
    let test_file = temp_dir.path().join("simple.rs");

    fs::write(&test_file, "fn hello() { println!(\"Hello\"); }").expect("Failed to write");

    // Most basic usage - should still work
    let analysis = SourceCode::new(&test_file)
        .expect("Failed to create SourceCode")
        .analyze()
        .expect("Analysis failed");

    // Basic access patterns should work
    assert!(analysis.functions().len() >= 1);
    assert!(!analysis.to_jsonl().unwrap().is_empty());
}

#[test]
fn test_workspace_api() {
    let temp_dir = TempDir::new().expect("Failed to create temp dir");
    let src_dir = temp_dir.path().join("src");
    fs::create_dir(&src_dir).expect("Failed to create src dir");

    fs::write(src_dir.join("main.rs"), "fn main() {}").expect("Failed to write");
    fs::write(src_dir.join("lib.rs"), "fn helper() {}").expect("Failed to write");

    // Workspace analysis with simple API
    let analysis = SourceCode::new(&src_dir)
        .expect("Failed to create SourceCode")
        .search_workspace(true)
        .with_incremental_analysis(true)
        .analyze()
        .expect("Workspace analysis failed");

    // Should find multiple files
    if let Some(files_by_lang) = analysis.files_by_language() {
        assert!(!files_by_lang.is_empty());
    }

    // Advanced features accessible through result methods
    let interproc = analysis.interprocedural();
    let entry_points = interproc.entry_points();
    println!("Entry points: {:?}", entry_points);

    let unreachable = interproc.unreachable_functions();
    println!("Unreachable functions: {:?}", unreachable);
}