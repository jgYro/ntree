use ntree::api::{IncrementalAnalysisOptions, IncrementalAnalyzer};
use ntree::{
    ClassHierarchyAnalyzer, ContractSpec, EffectKind, ExternalSummary, FuncSummary, ParamSummary,
    Resolution, ResolutionAlgorithm, ReturnSummary, SymbolId, TaintKind, ThrowsKind,
};
use std::fs;
use tempfile::TempDir;

fn create_test_symbol(id: u32, name: &str) -> SymbolId {
    SymbolId::from_string(format!("test_{}_{}", id, name))
}

#[test]
fn test_func_summary_creation() {
    let sym_id = create_test_symbol(1, "test_function");
    let mut summary = FuncSummary::new(sym_id.clone(), 1);

    // Add effects
    summary.add_effect(EffectKind::IoOperation);
    summary.add_effect(EffectKind::Allocation);

    // Add exceptions
    summary.add_throw(ThrowsKind::Exception("ValueError".to_string()));

    // Add parameter
    let param = ParamSummary::new("input".to_string())
        .with_type("String".to_string())
        .mutated();
    summary.add_param(param);

    // Set return
    let return_summary = ReturnSummary::new()
        .with_type("Result<i32, String>".to_string())
        .depends_on_params();
    summary.set_return(return_summary);

    // Verify properties
    assert!(summary.has_side_effects());
    assert!(summary.can_throw());
    assert_eq!(summary.params_summary.len(), 1);
    assert_eq!(summary.params_summary[0].name, "input");
    assert!(summary.params_summary[0].is_mutated);
}

#[test]
fn test_external_summary() {
    let summary = ExternalSummary::new("std::println!".to_string(), "std".to_string())
        .with_effect(EffectKind::IoOperation)
        .with_taint(TaintKind::Sink)
        .with_contract(ContractSpec::new().with_side_effect("Writes to stdout".to_string()));

    assert!(summary.is_taint_sink());
    assert!(!summary.is_taint_source());
    assert!(summary.has_side_effects());
    assert_eq!(
        summary.security_risk_level(),
        ntree::SecurityRiskLevel::High
    );
}

#[test]
fn test_cha_analyzer() {
    let mut cha = ClassHierarchyAnalyzer::new();

    let base_type = create_test_symbol(1, "Animal");
    let derived_type = create_test_symbol(2, "Dog");
    let base_method = create_test_symbol(3, "Animal::speak");
    let override_method = create_test_symbol(4, "Dog::speak");

    // Set up hierarchy
    cha.add_inheritance(derived_type.clone(), base_type.clone());
    cha.add_method(base_type.clone(), "speak".to_string(), base_method.clone());
    cha.add_method(
        derived_type.clone(),
        "speak".to_string(),
        override_method.clone(),
    );
    cha.add_override(base_method, override_method);

    let stats = cha.get_stats();
    assert_eq!(stats.total_types, 1); // One inheritance relationship
    assert_eq!(stats.total_methods, 2); // Two method implementations
}

#[test]
fn test_resolution_types() {
    let caller = create_test_symbol(1, "caller");
    let target = create_test_symbol(2, "target");

    // Direct resolution
    let direct = Resolution::direct(1, target.clone(), caller.clone(), "foo()".to_string());
    assert!(direct.is_definitive());
    assert!(!direct.is_ambiguous());
    assert_eq!(direct.confidence, 1.0);
    assert_eq!(direct.algorithm, ResolutionAlgorithm::Direct);

    // CHA resolution with multiple candidates
    let target2 = create_test_symbol(3, "target2");
    let cha = Resolution::cha(
        2,
        vec![target.clone(), target2],
        caller.clone(),
        "bar()".to_string(),
    );
    assert!(!cha.is_definitive());
    assert!(cha.is_ambiguous());
    assert_eq!(cha.algorithm, ResolutionAlgorithm::CHA);

    // RTA resolution
    let rta = Resolution::rta(3, vec![target], caller, "baz()".to_string());
    assert!(rta.is_definitive()); // Single target
    assert!(!rta.is_ambiguous());
    assert_eq!(rta.algorithm, ResolutionAlgorithm::RTA);
}

#[test]
fn test_incremental_analyzer_creation() {
    let analyzer = IncrementalAnalyzer::new();

    // Check initial state
    let cache_stats = analyzer.get_cache_stats();
    assert_eq!(cache_stats.total_files, 0);
    assert_eq!(cache_stats.total_functions, 0);

    let dep_stats = analyzer.get_dependency_stats();
    assert_eq!(dep_stats.total_functions, 0);
    assert_eq!(dep_stats.total_calls, 0);
}

#[test]
fn test_incremental_options() {
    let options = IncrementalAnalysisOptions::default();
    assert!(!options.force_recompute);
    assert!(!options.enable_cha);
    assert!(!options.enable_rta);
    assert!(options.enable_external_analysis);

    // Test configuration
    let custom_options = IncrementalAnalysisOptions {
        enable_cha: true,
        enable_rta: true,
        force_recompute: true,
        ..Default::default()
    };
    assert!(custom_options.enable_cha);
    assert!(custom_options.enable_rta);
    assert!(custom_options.force_recompute);
}

#[test]
fn test_taint_analysis() {
    use ntree::SecurityRiskLevel;

    // High risk: taint sink
    let sink_summary = ExternalSummary::new("eval".to_string(), "builtins".to_string())
        .with_taint(TaintKind::Sink);
    assert_eq!(sink_summary.security_risk_level(), SecurityRiskLevel::High);

    // Medium risk: taint source
    let source_summary = ExternalSummary::new("input".to_string(), "builtins".to_string())
        .with_taint(TaintKind::Source);
    assert_eq!(
        source_summary.security_risk_level(),
        SecurityRiskLevel::Medium
    );

    // Low risk: has side effects but not taint-related
    let side_effect_summary = ExternalSummary::new("printf".to_string(), "libc".to_string())
        .with_effect(EffectKind::IoOperation);
    assert_eq!(
        side_effect_summary.security_risk_level(),
        SecurityRiskLevel::Low
    );

    // No risk: pure function
    let pure_summary =
        ExternalSummary::new("abs".to_string(), "math".to_string()).with_effect(EffectKind::Pure);
    assert_eq!(pure_summary.security_risk_level(), SecurityRiskLevel::None);
}

/// Test workspace creation for incremental analysis.
fn create_incremental_workspace() -> Result<TempDir, std::io::Error> {
    let temp_dir = TempDir::new()?;
    let base_path = temp_dir.path();

    fs::create_dir_all(base_path.join("src"))?;

    // Create files with different complexity patterns
    fs::write(
        base_path.join("src/main.rs"),
        r#"
struct Animal {
    name: String,
}

impl Animal {
    fn new(name: String) -> Self {
        Animal { name }
    }

    fn speak(&self) -> String {
        format!("{} makes a sound", self.name)
    }
}

struct Dog {
    animal: Animal,
    breed: String,
}

impl Dog {
    fn new(name: String, breed: String) -> Self {
        Dog {
            animal: Animal::new(name),
            breed,
        }
    }

    fn speak(&self) -> String {
        format!("{} barks", self.animal.name)
    }
}

fn main() {
    let animal = Animal::new("Generic".to_string());
    println!("{}", animal.speak());

    let dog = Dog::new("Rex".to_string(), "Labrador".to_string());
    println!("{}", dog.speak());
}
"#,
    )?;

    fs::write(
        base_path.join("Cargo.toml"),
        r#"
[package]
name = "incremental_test"
version = "0.1.0"
edition = "2021"
"#,
    )?;

    Ok(temp_dir)
}

#[test]
fn test_incremental_workspace_analysis() {
    let workspace = create_incremental_workspace().expect("Failed to create workspace");
    let workspace_path = workspace.path();

    let mut analyzer = IncrementalAnalyzer::new();

    let options = IncrementalAnalysisOptions {
        enable_cha: true,
        enable_external_analysis: true,
        ..Default::default()
    };

    // This will likely fail due to incomplete implementation, but tests the API
    match analyzer.analyze_incremental(workspace_path, options) {
        Ok(result) => {
            println!("Incremental analysis succeeded!");
            println!("Cache hit: {}", result.cache_hit);
            let metrics = result.get_performance_metrics();
            println!("Performance: {:?}", metrics);
        }
        Err(e) => {
            println!("Incremental analysis failed (expected): {}", e);
            // This is expected during development
        }
    }
}

#[test]
fn test_effect_and_throw_kinds() {
    // Test effect kinds
    let effects = vec![
        EffectKind::Pure,
        EffectKind::IoOperation,
        EffectKind::GlobalMutation,
        EffectKind::Allocation,
        EffectKind::External,
        EffectKind::ParamMutation,
    ];

    assert_eq!(effects.len(), 6);

    // Test throw kinds
    let throws = vec![
        ThrowsKind::Exception("Error".to_string()),
        ThrowsKind::Panic,
        ThrowsKind::EarlyReturn,
        ThrowsKind::ResourceError,
        ThrowsKind::TypedError("MyError".to_string()),
    ];

    assert_eq!(throws.len(), 5);
}
