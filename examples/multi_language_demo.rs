use ntree::SourceCode;

fn main() -> Result<(), Box<dyn std::error::Error>> {
    println!("=== ntree Multi-Language Analysis Demo ===\n");

    let test_files = vec![
        ("Rust", "test_sample.rs"),
        ("JavaScript", "test_samples/test_sample.js"),
        ("TypeScript", "test_samples/test_sample.ts"),
        ("Java", "test_samples/TestSample.java"),
        ("C", "test_samples/test_sample.c"),
        ("C++", "test_samples/test_sample.cpp"),
        ("Python", "/Users/jerichogregory/Yro/projects/Maximus/NASH/Individual_Rust_Feats/nash/code/test.py"),
    ];

    for (language, file_path) in &test_files {
        println!("🔍 Analyzing {} file: {}", language, file_path);

        match SourceCode::new(file_path) {
            Ok(source) => {
                match source.with_complexity_analysis(true).analyze() {
                    Ok(analysis) => {
                        println!("  ✅ {} analysis successful!", language);
                        println!("     Functions: {}", analysis.functions().len());
                        println!("     CFGs: {}", analysis.cfgs().len());
                        println!("     Complexity: {}", analysis.complexity().len());

                        // Show complexity summary
                        let complexity_results = analysis.complexity();
                        if !complexity_results.all().is_empty() {
                            let total_complexity: u32 =
                                complexity_results.all().iter().map(|c| c.cyclomatic).sum();
                            let avg_complexity =
                                total_complexity as f32 / complexity_results.len() as f32;
                            println!("     Average complexity: {:.1}", avg_complexity);

                            // Show highest complexity function
                            if let Some(max_complexity) =
                                complexity_results.all().iter().max_by_key(|c| c.cyclomatic)
                            {
                                println!(
                                    "     Most complex: {} ({})",
                                    max_complexity.function, max_complexity.cyclomatic
                                );
                            }
                        }
                    }
                    Err(e) => {
                        println!("  ❌ {} analysis failed: {:?}", language, e);
                    }
                }
            }
            Err(e) => {
                println!("  ❌ {} file error: {:?}", language, e);
            }
        }
        println!();
    }

    // Demonstrate cross-language JSON output
    println!("📊 Cross-Language Complexity Comparison:");
    println!(
        "{:<12} {:<10} {:<10} {:<15}",
        "Language", "Functions", "CFGs", "Avg Complexity"
    );
    println!("{}", "-".repeat(50));

    for (language, file_path) in &test_files[..5] {
        // Test first 5 languages
        match SourceCode::new(file_path).and_then(|s| s.analyze()) {
            Ok(analysis) => {
                let complexity_results = analysis.complexity();
                let avg_complexity = if !complexity_results.all().is_empty() {
                    let total: u32 = complexity_results.all().iter().map(|c| c.cyclomatic).sum();
                    total as f32 / complexity_results.len() as f32
                } else {
                    0.0
                };

                println!(
                    "{:<12} {:<10} {:<10} {:<15.1}",
                    language,
                    analysis.functions().len(),
                    analysis.cfgs().len(),
                    avg_complexity
                );
            }
            Err(_) => {
                println!(
                    "{:<12} {:<10} {:<10} {:<15}",
                    language, "Error", "Error", "Error"
                );
            }
        }
    }

    println!("\n🎉 ntree now supports 7 programming languages!");
    println!("Use the same unified API for all languages - automatic detection!");

    Ok(())
}
