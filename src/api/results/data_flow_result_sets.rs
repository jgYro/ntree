use crate::analyzers::CrossFileVariable;
use crate::models::{
    DataFlowGraph, DecisionTree, DecisionTreeSet, DefUseChain, DefUseChainSet, VariableLifecycle,
    VariableLifecycleSet,
};

/// Result set for data flow graphs with filtering and export capabilities.
pub struct DataFlowResultSet<'a> {
    data: &'a [DataFlowGraph],
}

impl<'a> DataFlowResultSet<'a> {
    /// Create new result set.
    pub fn new(data: &'a [DataFlowGraph]) -> Self {
        DataFlowResultSet { data }
    }

    /// Get all data flow graphs.
    pub fn all(&self) -> &[DataFlowGraph] {
        self.data
    }

    /// Get data flow graph for specific function.
    pub fn for_function(&self, function_name: &str) -> Option<&DataFlowGraph> {
        self.data
            .iter()
            .find(|graph| graph.function_name == function_name)
    }

    /// Get functions with data dependencies.
    pub fn functions_with_dependencies(&self) -> Vec<&str> {
        self.data
            .iter()
            .filter(|graph| !graph.edges.is_empty())
            .map(|graph| graph.function_name.as_str())
            .collect()
    }

    /// Get total number of data dependency edges.
    pub fn total_dependencies(&self) -> usize {
        self.data.iter().map(|graph| graph.edges.len()).sum()
    }
}

/// Result set for variable lifecycles with filtering capabilities.
pub struct VariableLifecycleResultSet<'a> {
    data: &'a VariableLifecycleSet,
}

impl<'a> VariableLifecycleResultSet<'a> {
    /// Create new result set.
    pub fn new(data: &'a VariableLifecycleSet) -> Self {
        VariableLifecycleResultSet { data }
    }

    /// Get all variable lifecycles.
    pub fn all(&self) -> Vec<&VariableLifecycle> {
        self.data.all()
    }

    /// Get lifecycle for specific variable.
    pub fn for_variable(&self, variable_name: &str) -> Option<&VariableLifecycle> {
        self.data.get(variable_name)
    }

    /// Get variables that are mutated after definition.
    pub fn mutated_variables(&self) -> Vec<&VariableLifecycle> {
        self.data
            .all()
            .into_iter()
            .filter(|lifecycle| lifecycle.is_mutated())
            .collect()
    }

    /// Get variables that are never used.
    pub fn unused_variables(&self) -> Vec<&VariableLifecycle> {
        self.data
            .all()
            .into_iter()
            .filter(|lifecycle| !lifecycle.is_used())
            .collect()
    }

    /// Get variables live at function exit.
    pub fn live_variables(&self) -> Vec<&VariableLifecycle> {
        self.data
            .all()
            .into_iter()
            .filter(|lifecycle| lifecycle.live_at_exit)
            .collect()
    }
}

/// Result set for def-use chains with filtering capabilities.
pub struct DefUseChainResultSet<'a> {
    data: &'a DefUseChainSet,
}

impl<'a> DefUseChainResultSet<'a> {
    /// Create new result set.
    pub fn new(data: &'a DefUseChainSet) -> Self {
        DefUseChainResultSet { data }
    }

    /// Get all def-use chains.
    pub fn all(&self) -> Vec<&DefUseChain> {
        self.data.all()
    }

    /// Get chains for specific function.
    pub fn for_function(&self, function_name: &str) -> Vec<&DefUseChain> {
        self.data.for_function(function_name)
    }

    /// Get dead definitions (definitions with no uses).
    pub fn dead_definitions(&self) -> Vec<&DefUseChain> {
        self.data.dead_definitions()
    }

    /// Get chains with multiple uses.
    pub fn heavily_used_definitions(&self, min_uses: usize) -> Vec<&DefUseChain> {
        self.data
            .all()
            .into_iter()
            .filter(|chain| chain.use_count() >= min_uses)
            .collect()
    }
}

/// Result set for decision trees with filtering capabilities.
pub struct DecisionTreeResultSet<'a> {
    data: &'a DecisionTreeSet,
}

impl<'a> DecisionTreeResultSet<'a> {
    /// Create new result set.
    pub fn new(data: &'a DecisionTreeSet) -> Self {
        DecisionTreeResultSet { data }
    }

    /// Get all decision trees.
    pub fn all(&self) -> Vec<&DecisionTree> {
        self.data.all()
    }

    /// Get decision tree for specific function.
    pub fn for_function(&self, function_name: &str) -> Option<&DecisionTree> {
        self.data.for_function(function_name)
    }

    /// Get functions with unreachable paths.
    pub fn functions_with_dead_code(&self) -> Vec<&str> {
        self.data
            .all()
            .into_iter()
            .filter(|tree| !tree.unreachable_paths().is_empty())
            .map(|tree| tree.function_name.as_str())
            .collect()
    }

    /// Get total number of decision paths across all functions.
    pub fn total_paths(&self) -> usize {
        self.data.all().iter().map(|tree| tree.paths.len()).sum()
    }

    /// Get total number of reachable paths.
    pub fn reachable_paths(&self) -> usize {
        self.data
            .all()
            .iter()
            .map(|tree| tree.reachable_paths().len())
            .sum()
    }
}

/// Result set for cross-file variables with filtering capabilities.
pub struct CrossFileVariableResultSet<'a> {
    data: &'a [CrossFileVariable],
}

impl<'a> CrossFileVariableResultSet<'a> {
    /// Create new result set.
    pub fn new(data: &'a [CrossFileVariable]) -> Self {
        CrossFileVariableResultSet { data }
    }

    /// Get all cross-file variables.
    pub fn all(&self) -> &[CrossFileVariable] {
        self.data
    }

    /// Get variables imported from external modules.
    pub fn imported_variables(&self) -> Vec<&CrossFileVariable> {
        self.data
            .iter()
            .filter(|var| !var.usage_files.is_empty())
            .collect()
    }

    /// Get variables by definition file.
    pub fn from_file(&self, file_path: &str) -> Vec<&CrossFileVariable> {
        self.data
            .iter()
            .filter(|var| var.definition_file.to_string_lossy().contains(file_path))
            .collect()
    }

    /// Get total number of cross-file dependencies.
    pub fn total_cross_file_dependencies(&self) -> usize {
        self.data.iter().map(|var| var.usage_files.len()).sum()
    }
}
