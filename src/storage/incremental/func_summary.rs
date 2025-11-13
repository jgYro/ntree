use crate::storage::SymbolId;
use serde::{Deserialize, Serialize};
use std::collections::HashSet;

/// Effect types for function analysis.
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq, Hash)]
pub enum EffectKind {
    /// Function modifies global state
    GlobalMutation,
    /// Function performs I/O operations
    IoOperation,
    /// Function allocates memory
    Allocation,
    /// Function is pure (no side effects)
    Pure,
    /// Function calls external/unknown code
    External,
    /// Function modifies parameters
    ParamMutation,
}

/// Exception types that functions can throw.
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq, Hash)]
pub enum ThrowsKind {
    /// Standard exception/error
    Exception(String),
    /// Panic/abort
    Panic,
    /// Early return with error (Rust ?)
    EarlyReturn,
    /// Timeout/resource exhaustion
    ResourceError,
    /// Type-specific errors (Result<T, E>)
    TypedError(String),
}

/// Parameter summary for function signature analysis.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ParamSummary {
    /// Parameter name
    pub name: String,
    /// Parameter type (if available)
    pub param_type: Option<String>,
    /// Whether parameter is mutated
    pub is_mutated: bool,
    /// Whether parameter can be null/None
    pub nullable: bool,
}

impl ParamSummary {
    /// Create new parameter summary.
    pub fn new(name: String) -> Self {
        ParamSummary {
            name,
            param_type: None,
            is_mutated: false,
            nullable: false,
        }
    }

    /// Set parameter type.
    pub fn with_type(mut self, param_type: String) -> Self {
        self.param_type = Some(param_type);
        self
    }

    /// Mark as mutated.
    pub fn mutated(mut self) -> Self {
        self.is_mutated = true;
        self
    }

    /// Mark as nullable.
    pub fn nullable(mut self) -> Self {
        self.nullable = true;
        self
    }
}

/// Return value summary for function analysis.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ReturnSummary {
    /// Return type (if available)
    pub return_type: Option<String>,
    /// Whether return can be null/None
    pub nullable: bool,
    /// Whether return depends on parameters
    pub depends_on_params: bool,
    /// Whether return depends on global state
    pub depends_on_global: bool,
}

impl ReturnSummary {
    /// Create new return summary.
    pub fn new() -> Self {
        ReturnSummary {
            return_type: None,
            nullable: false,
            depends_on_params: false,
            depends_on_global: false,
        }
    }

    /// Set return type.
    pub fn with_type(mut self, return_type: String) -> Self {
        self.return_type = Some(return_type);
        self
    }

    /// Mark as dependent on parameters.
    pub fn depends_on_params(mut self) -> Self {
        self.depends_on_params = true;
        self
    }

    /// Mark as dependent on global state.
    pub fn depends_on_global(mut self) -> Self {
        self.depends_on_global = true;
        self
    }
}

/// Function summary for incremental analysis.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct FuncSummary {
    /// Symbol ID of the function
    pub sym_id: SymbolId,
    /// Side effects performed by function
    pub effects: HashSet<EffectKind>,
    /// Exception types this function can throw
    pub throws: HashSet<ThrowsKind>,
    /// Parameter summaries
    pub params_summary: Vec<ParamSummary>,
    /// Return value summary
    pub returns_summary: ReturnSummary,
    /// Version/timestamp for cache invalidation
    pub version: u64,
    /// Functions this function calls
    pub calls: HashSet<SymbolId>,
    /// Whether function is pure (no side effects)
    pub is_pure: bool,
}

impl FuncSummary {
    /// Create new function summary.
    pub fn new(sym_id: SymbolId, version: u64) -> Self {
        FuncSummary {
            sym_id,
            effects: HashSet::new(),
            throws: HashSet::new(),
            params_summary: Vec::new(),
            returns_summary: ReturnSummary::new(),
            version,
            calls: HashSet::new(),
            is_pure: false,
        }
    }

    /// Add an effect to the function.
    pub fn add_effect(&mut self, effect: EffectKind) {
        if matches!(effect, EffectKind::Pure) {
            self.is_pure = true;
            self.effects.clear();
        } else {
            self.is_pure = false;
            self.effects.remove(&EffectKind::Pure);
        }
        self.effects.insert(effect);
    }

    /// Add an exception type.
    pub fn add_throw(&mut self, throw_kind: ThrowsKind) {
        self.throws.insert(throw_kind);
    }

    /// Add parameter summary.
    pub fn add_param(&mut self, param: ParamSummary) {
        self.params_summary.push(param);
    }

    /// Set return summary.
    pub fn set_return(&mut self, return_summary: ReturnSummary) {
        self.returns_summary = return_summary;
    }

    /// Add function call dependency.
    pub fn add_call(&mut self, callee: SymbolId) {
        self.calls.insert(callee);
    }

    /// Check if function has side effects.
    pub fn has_side_effects(&self) -> bool {
        !self.is_pure && !self.effects.is_empty()
    }

    /// Check if function can throw exceptions.
    pub fn can_throw(&self) -> bool {
        !self.throws.is_empty()
    }

    /// Get all called functions.
    pub fn get_callees(&self) -> &HashSet<SymbolId> {
        &self.calls
    }

    /// Check if summary is newer than given version.
    pub fn is_newer_than(&self, version: u64) -> bool {
        self.version > version
    }

    /// Update version for cache invalidation.
    pub fn update_version(&mut self, new_version: u64) {
        self.version = new_version;
    }
}
