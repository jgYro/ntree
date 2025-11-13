use ntree::SourceCode;
use std::fs;
use tempfile::TempDir;

/// Create a test workspace with multiple files and a Cargo.toml (Rust project).
fn create_test_workspace() -> Result<TempDir, std::io::Error> {
    let temp_dir = TempDir::new()?;
    let base_path = temp_dir.path();

    // Create src directory
    fs::create_dir_all(base_path.join("src"))?;

    // Create Cargo.toml (project root indicator)
    fs::write(
        base_path.join("Cargo.toml"),
        r#"
[package]
name = "test_workspace"
version = "0.1.0"
edition = "2021"

[dependencies]
"#,
    )?;

    // Create main.rs with cross-file dependencies
    fs::write(
        base_path.join("src/main.rs"),
        r#"
mod utils;
mod calculator;

use utils::helper_function;
use calculator::Calculator;

fn main() {
    let input = "test data";
    let processed = helper_function(input);

    let calc = Calculator::new();
    let result = calc.calculate(10, 5);

    println!("Processed: {}, Result: {}", processed, result);
}

fn process_data(data: &str) -> String {
    if data.is_empty() {
        return "empty".to_string();
    }

    let mut result = data.to_string();
    result.push_str("_processed");
    result
}
"#,
    )?;

    // Create utils.rs module
    fs::write(
        base_path.join("src/utils.rs"),
        r#"
pub fn helper_function(input: &str) -> String {
    if input.len() > 10 {
        format!("Long: {}", input)
    } else {
        format!("Short: {}", input)
    }
}

pub fn validate_input(input: &str) -> bool {
    !input.is_empty() && input.len() < 100
}
"#,
    )?;

    // Create calculator.rs module
    fs::write(
        base_path.join("src/calculator.rs"),
        r#"
pub struct Calculator {
    precision: u32,
}

impl Calculator {
    pub fn new() -> Self {
        Calculator { precision: 2 }
    }

    pub fn calculate(&self, a: i32, b: i32) -> i32 {
        let mut result = a;

        if b > 0 {
            result += b;
        } else {
            result -= b.abs();
        }

        result * self.precision as i32
    }

    pub fn set_precision(&mut self, precision: u32) {
        self.precision = precision;
    }
}
"#,
    )?;

    Ok(temp_dir)
}

#[test]
fn test_workspace_data_flow_analysis() {
    let workspace = create_test_workspace().expect("Failed to create test workspace");
    let workspace_path = workspace.path();

    // Test workspace analysis with data flow enabled
    let analysis = SourceCode::new(workspace_path)
        .expect("Valid workspace path")
        .search_workspace(true) // Enable workspace mode
        .with_data_flow_analysis(true) // Enable data flow
        .with_variable_lifecycle_tracking(true) // Enable variable tracking
        .with_def_use_chains(true) // Enable def-use analysis
        .analyze()
        .expect("Workspace analysis should succeed");

    // Verify workspace mode is active
    assert!(analysis.is_workspace_mode(), "Should be in workspace mode");

    // Verify workspace statistics
    let workspace_stats = analysis
        .workspace_stats()
        .expect("Should have workspace stats");
    assert!(
        workspace_stats.total_files >= 3,
        "Should find at least 3 source files"
    ); // main.rs, utils.rs, calculator.rs
    assert_eq!(
        workspace_stats.languages, 1,
        "Should detect 1 language (Rust)"
    );

    // Verify files grouped by language
    let files_by_lang = analysis
        .files_by_language()
        .expect("Should have files by language");
    assert!(files_by_lang.contains_key("rust"), "Should have Rust files");
    assert!(
        files_by_lang.get("rust").unwrap().len() >= 3,
        "Should have multiple Rust files"
    );

    // Test data flow analysis results
    let data_flow = analysis.data_flow();
    println!("Workspace data flow graphs: {}", data_flow.all().len());
    assert!(
        data_flow.all().len() >= 3,
        "Should have data flow for multiple functions"
    );

    // Test variable lifecycle results
    let variables = analysis.variables();
    println!("Workspace variables: {}", variables.all().len());
    assert!(
        variables.all().len() >= 5,
        "Should track variables across files"
    );

    // Test cross-file variable tracking
    let cross_file_vars = analysis.cross_file_variables();
    println!("Cross-file variables: {}", cross_file_vars.all().len());

    // Verify we can access all result types
    let def_use = analysis.def_use_chains();
    let decisions = analysis.decision_trees();

    println!("Def-use chains: {}", def_use.all().len());
    println!("Decision trees: {}", decisions.all().len());
}

#[test]
fn test_workspace_project_detection() {
    let workspace = create_test_workspace().expect("Failed to create test workspace");
    let workspace_path = workspace.path();

    // Test that the API works even without explicit project detection calls
    let analysis = SourceCode::new(workspace_path)
        .expect("Valid workspace path")
        .search_workspace(true)
        .with_complexity_analysis(true)
        .with_cfg_generation(true)
        .analyze()
        .expect("Analysis should succeed");

    // Verify existing workspace functionality still works
    assert!(analysis.is_workspace_mode());
    assert!(
        analysis.complexity().len() > 0,
        "Should have complexity results"
    );
    assert!(analysis.cfgs().len() > 0, "Should have CFG results");
    assert!(
        analysis.functions().len() > 0,
        "Should have function results"
    );

    // Symbol search should work across the workspace
    let symbols = analysis.symbols();
    let function_search = symbols.kind("function");
    let all_functions = function_search.search().expect("Symbol search should work");
    assert!(
        all_functions.len() >= 4,
        "Should find functions across multiple files"
    );
}

#[test]
fn test_workspace_vs_single_file_consistency() {
    let workspace = create_test_workspace().expect("Failed to create test workspace");
    let workspace_path = workspace.path();
    let main_file = workspace_path.join("src/main.rs");

    // Analyze same file both ways
    let single_analysis = SourceCode::new(&main_file)
        .expect("Valid file")
        .with_data_flow_analysis(true)
        .with_variable_lifecycle_tracking(true)
        .analyze()
        .expect("Single file analysis should succeed");

    let workspace_analysis = SourceCode::new(workspace_path)
        .expect("Valid workspace")
        .search_workspace(true)
        .with_data_flow_analysis(true)
        .with_variable_lifecycle_tracking(true)
        .analyze()
        .expect("Workspace analysis should succeed");

    // Single file should work
    assert!(!single_analysis.is_workspace_mode());
    assert!(single_analysis.functions().len() > 0);

    // Workspace should have more comprehensive results
    assert!(workspace_analysis.is_workspace_mode());
    assert!(workspace_analysis.functions().len() >= single_analysis.functions().len());

    // Workspace should have cross-file capabilities
    let cross_file_vars = workspace_analysis.cross_file_variables();
    println!(
        "Cross-file variables in workspace: {}",
        cross_file_vars.all().len()
    );
}

#[test]
fn test_data_flow_disabled_in_workspace() {
    let workspace = create_test_workspace().expect("Failed to create test workspace");
    let workspace_path = workspace.path();

    // Test with data flow disabled
    let analysis = SourceCode::new(workspace_path)
        .expect("Valid workspace")
        .search_workspace(true)
        .with_data_flow_analysis(false)
        .with_variable_lifecycle_tracking(false)
        .analyze()
        .expect("Analysis should succeed");

    // Should still have basic workspace functionality
    assert!(analysis.is_workspace_mode());
    assert!(analysis.functions().len() > 0);

    // Data flow results should be empty
    let data_flow = analysis.data_flow();
    let variables = analysis.variables();

    assert_eq!(data_flow.all().len(), 0, "Data flow should be disabled");
    assert_eq!(
        variables.all().len(),
        0,
        "Variable tracking should be disabled"
    );
}
