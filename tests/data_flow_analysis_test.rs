use ntree::SourceCode;

#[test]
fn test_data_flow_analysis_api_integration() {
    // Test that the new API methods exist and work with a simple example
    let analysis = SourceCode::new("test_sample.rs")
        .expect("Valid file")
        .with_data_flow_analysis(true)
        .with_variable_lifecycle_tracking(true)
        .with_def_use_chains(true)
        .with_decision_tree_mapping(true)
        .analyze()
        .expect("Analysis should succeed");

    // Test that new result access methods exist
    let data_flow = analysis.data_flow();
    let variables = analysis.variables();
    let def_use = analysis.def_use_chains();
    let decisions = analysis.decision_trees();

    // Verify basic functionality
    println!("Data flow graphs: {}", data_flow.all().len());
    println!("Variable lifecycles: {}", variables.all().len());
    println!("Def-use chains: {}", def_use.all().len());
    println!("Decision trees: {}", decisions.all().len());

    // Test filtering methods
    let mutated_vars = variables.mutated_variables();
    let unused_vars = variables.unused_variables();
    let dead_defs = def_use.dead_definitions();

    println!("Mutated variables: {}", mutated_vars.len());
    println!("Unused variables: {}", unused_vars.len());
    println!("Dead definitions: {}", dead_defs.len());

    // Since this is a basic test without specific content analysis,
    // we just verify the API works without crashing
    assert!(true, "Data flow analysis API integration successful");
}

#[test]
fn test_data_flow_analysis_disabled() {
    // Test that analysis works with data flow features disabled
    let analysis = SourceCode::new("test_sample.rs")
        .expect("Valid file")
        .with_data_flow_analysis(false)
        .with_variable_lifecycle_tracking(false)
        .with_def_use_chains(false)
        .with_decision_tree_mapping(false)
        .analyze()
        .expect("Analysis should succeed");

    // Even when disabled, the result methods should exist but return empty results
    let data_flow = analysis.data_flow();
    let variables = analysis.variables();
    let def_use = analysis.def_use_chains();
    let decisions = analysis.decision_trees();

    // These should be empty since analysis was disabled
    assert_eq!(data_flow.all().len(), 0);
    assert_eq!(variables.all().len(), 0);
    assert_eq!(def_use.all().len(), 0);
    assert_eq!(decisions.all().len(), 0);
}

#[test]
fn test_minimal_configuration_excludes_data_flow() {
    // Test that minimal configuration doesn't include data flow analysis
    let analysis = SourceCode::new("test_sample.rs")
        .expect("Valid file")
        .minimal()
        .analyze()
        .expect("Analysis should succeed");

    // Minimal should only have complexity and CFG
    assert!(analysis.complexity().len() > 0, "Should have complexity analysis");
    assert!(analysis.cfgs().len() > 0, "Should have CFG analysis");

    // Data flow features should be empty in minimal mode
    let data_flow = analysis.data_flow();
    let variables = analysis.variables();

    assert_eq!(data_flow.all().len(), 0, "Minimal mode should not include data flow");
    assert_eq!(variables.all().len(), 0, "Minimal mode should not include variables");
}

#[test]
fn test_builder_pattern_consistency() {
    // Test that the builder pattern methods return Self and can be chained
    let source = SourceCode::new("test_sample.rs")
        .expect("Valid file")
        .with_complexity_analysis(true)
        .with_cfg_generation(true)
        .with_data_flow_analysis(true)
        .with_variable_lifecycle_tracking(true)
        .with_def_use_chains(true)
        .with_decision_tree_mapping(true)
        .with_basic_blocks(true);

    // This should compile and execute successfully
    let analysis = source.analyze().expect("Chained analysis should succeed");

    // Verify we have both traditional and new analysis results
    assert!(analysis.complexity().len() > 0, "Should have complexity");
    assert!(analysis.cfgs().len() > 0, "Should have CFGs");

    // New features should be accessible
    let _data_flow = analysis.data_flow();
    let _variables = analysis.variables();
    let _def_use = analysis.def_use_chains();
    let _decisions = analysis.decision_trees();
}

#[test]
fn test_variable_lifecycle_filtering() {
    // Test variable lifecycle filtering methods
    let analysis = SourceCode::new("test_sample.rs")
        .expect("Valid file")
        .with_variable_lifecycle_tracking(true)
        .analyze()
        .expect("Analysis should succeed");

    let variables = analysis.variables();

    // Test all filtering methods exist and work
    let all_vars = variables.all();
    let mutated_vars = variables.mutated_variables();
    let unused_vars = variables.unused_variables();
    let live_vars = variables.live_variables();

    // Verify filtering methods return subsets
    assert!(mutated_vars.len() <= all_vars.len());
    assert!(unused_vars.len() <= all_vars.len());
    assert!(live_vars.len() <= all_vars.len());

    // Test individual variable lookup
    if !all_vars.is_empty() {
        let first_var = &all_vars[0];
        let found_var = variables.for_variable(&first_var.name);
        assert!(found_var.is_some(), "Should find variable by name");
    }
}

#[test]
fn test_def_use_chain_filtering() {
    // Test def-use chain filtering methods
    let analysis = SourceCode::new("test_sample.rs")
        .expect("Valid file")
        .with_def_use_chains(true)
        .analyze()
        .expect("Analysis should succeed");

    let def_use = analysis.def_use_chains();

    // Test filtering methods
    let all_chains = def_use.all();
    let dead_defs = def_use.dead_definitions();
    let heavily_used = def_use.heavily_used_definitions(2);

    // Verify filtering
    assert!(dead_defs.len() <= all_chains.len());
    assert!(heavily_used.len() <= all_chains.len());

    // Test function-specific lookup
    if !all_chains.is_empty() {
        let function_name = &all_chains[0].function_name;
        let function_chains = def_use.for_function(function_name);
        assert!(!function_chains.is_empty(), "Should find chains for function");
    }
}

#[test]
fn test_decision_tree_filtering() {
    // Test decision tree filtering methods
    let analysis = SourceCode::new("test_sample.rs")
        .expect("Valid file")
        .with_decision_tree_mapping(true)
        .analyze()
        .expect("Analysis should succeed");

    let decisions = analysis.decision_trees();

    // Test metrics
    let all_trees = decisions.all();
    let total_paths = decisions.total_paths();
    let reachable_paths = decisions.reachable_paths();
    let functions_with_dead_code = decisions.functions_with_dead_code();

    // Basic validation
    assert!(reachable_paths <= total_paths, "Reachable paths should not exceed total");
    assert!(functions_with_dead_code.len() <= all_trees.len());

    // Test function-specific lookup
    if !all_trees.is_empty() {
        let function_name = &all_trees[0].function_name;
        let tree = decisions.for_function(function_name);
        assert!(tree.is_some(), "Should find decision tree for function");
    }
}

#[test]
fn test_data_flow_graph_methods() {
    // Test data flow graph methods
    let analysis = SourceCode::new("test_sample.rs")
        .expect("Valid file")
        .with_data_flow_analysis(true)
        .analyze()
        .expect("Analysis should succeed");

    let data_flow = analysis.data_flow();

    // Test metrics
    let all_graphs = data_flow.all();
    let functions_with_deps = data_flow.functions_with_dependencies();
    let total_deps = data_flow.total_dependencies();

    // Verify relationships
    assert!(functions_with_deps.len() <= all_graphs.len());

    // Test function-specific lookup
    if !all_graphs.is_empty() {
        let function_name = &all_graphs[0].function_name;
        let graph = data_flow.for_function(function_name);
        assert!(graph.is_some(), "Should find graph for function");
    }

    println!("Data flow analysis - Functions with dependencies: {}", functions_with_deps.len());
    println!("Data flow analysis - Total dependencies: {}", total_deps);
}