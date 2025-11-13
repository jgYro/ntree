use serde::{Deserialize, Serialize};
use std::collections::HashMap;

/// Represents the lifecycle of a variable through its scope.
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct VariableLifecycle {
    /// Variable name
    pub name: String,
    /// Type of the variable (if known)
    pub variable_type: Option<String>,
    /// Definition location
    pub definition: VariableEvent,
    /// All usage locations
    pub uses: Vec<VariableEvent>,
    /// Mutation locations (assignments after definition)
    pub mutations: Vec<VariableEvent>,
    /// Scope information
    pub scope: VariableScope,
    /// Whether variable is live at function exit
    pub live_at_exit: bool,
}

/// Represents a variable definition, use, or mutation event.
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct VariableEvent {
    /// Location in source code
    pub span: String,
    /// Type of event
    pub event_type: VariableEventType,
    /// Context where event occurs
    pub context: String,
    /// Line number
    pub line: u32,
    /// Column number
    pub column: u32,
}

/// Types of variable events.
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub enum VariableEventType {
    /// Variable declaration/definition
    Definition,
    /// Variable read/use
    Use,
    /// Variable assignment/mutation
    Mutation,
    /// Variable passed as parameter
    ParameterPass,
    /// Variable returned from function
    Return,
}

/// Scope information for a variable.
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct VariableScope {
    /// Function containing the variable
    pub function_name: String,
    /// Nested scope level (0 = function level)
    pub scope_level: u32,
    /// Scope start location
    pub scope_start: String,
    /// Scope end location
    pub scope_end: String,
    /// Whether variable is captured by closure
    pub captured: bool,
}

impl VariableLifecycle {
    /// Create a new variable lifecycle.
    pub fn new(name: String, definition: VariableEvent, scope: VariableScope) -> Self {
        VariableLifecycle {
            name,
            variable_type: None,
            definition,
            uses: Vec::new(),
            mutations: Vec::new(),
            scope,
            live_at_exit: false,
        }
    }

    /// Add a usage event.
    pub fn add_use(&mut self, use_event: VariableEvent) {
        self.uses.push(use_event);
    }

    /// Add a mutation event.
    pub fn add_mutation(&mut self, mutation_event: VariableEvent) {
        self.mutations.push(mutation_event);
    }

    /// Check if variable is mutated after definition.
    pub fn is_mutated(&self) -> bool {
        !self.mutations.is_empty()
    }

    /// Check if variable is used after definition.
    pub fn is_used(&self) -> bool {
        !self.uses.is_empty()
    }

    /// Set variable type.
    pub fn with_type(mut self, variable_type: String) -> Self {
        self.variable_type = Some(variable_type);
        self
    }
}

/// Collection of variable lifecycles for analysis results.
#[derive(Debug, Default, Serialize, Deserialize)]
pub struct VariableLifecycleSet {
    lifecycles: HashMap<String, VariableLifecycle>,
}

impl VariableLifecycleSet {
    /// Create new empty set.
    pub fn new() -> Self {
        Self {
            lifecycles: HashMap::new(),
        }
    }

    /// Add a variable lifecycle.
    pub fn add_lifecycle(&mut self, lifecycle: VariableLifecycle) {
        self.lifecycles.insert(lifecycle.name.clone(), lifecycle);
    }

    /// Get all variable lifecycles.
    pub fn all(&self) -> Vec<&VariableLifecycle> {
        self.lifecycles.values().collect()
    }

    /// Get lifecycle by variable name.
    pub fn get(&self, name: &str) -> Option<&VariableLifecycle> {
        self.lifecycles.get(name)
    }
}
