use serde::{Deserialize, Serialize};
use super::super::incremental::func_summary::EffectKind;
use std::collections::HashSet;

/// Taint analysis kinds for security analysis.
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq, Hash)]
pub enum TaintKind {
    /// Source of untrusted data (user input, network)
    Source,
    /// Sink that should not receive untrusted data (SQL, eval)
    Sink,
    /// Sanitizer that cleans data
    Sanitizer,
    /// Propagates taint without modification
    Propagator,
}

/// Contract specification for external functions.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ContractSpec {
    /// Preconditions (parameter constraints)
    pub preconditions: Vec<String>,
    /// Postconditions (return value guarantees)
    pub postconditions: Vec<String>,
    /// Side effect descriptions
    pub side_effects: Vec<String>,
    /// Exception specifications
    pub exceptions: Vec<String>,
}

impl ContractSpec {
    /// Create new contract specification.
    pub fn new() -> Self {
        ContractSpec {
            preconditions: Vec::new(),
            postconditions: Vec::new(),
            side_effects: Vec::new(),
            exceptions: Vec::new(),
        }
    }

    /// Add precondition.
    pub fn with_precondition(mut self, condition: String) -> Self {
        self.preconditions.push(condition);
        self
    }

    /// Add postcondition.
    pub fn with_postcondition(mut self, condition: String) -> Self {
        self.postconditions.push(condition);
        self
    }

    /// Add side effect description.
    pub fn with_side_effect(mut self, effect: String) -> Self {
        self.side_effects.push(effect);
        self
    }

    /// Add exception specification.
    pub fn with_exception(mut self, exception: String) -> Self {
        self.exceptions.push(exception);
        self
    }
}

/// External function/library summary for black-box analysis.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ExternalSummary {
    /// Qualified name of external function/method
    pub qualname: String,
    /// Library/package this function belongs to
    pub library: String,
    /// Version of the library
    pub version: Option<String>,
    /// Side effects this function performs
    pub effects: HashSet<EffectKind>,
    /// Taint analysis information
    pub taint_info: HashSet<TaintKind>,
    /// Contract specification (if available)
    pub contract: Option<ContractSpec>,
    /// Whether function is thread-safe
    pub thread_safe: bool,
    /// Whether function terminates
    pub terminates: bool,
    /// Parameter count (if known)
    pub param_count: Option<usize>,
    /// Return type (if known)
    pub return_type: Option<String>,
}

impl ExternalSummary {
    /// Create new external summary.
    pub fn new(qualname: String, library: String) -> Self {
        ExternalSummary {
            qualname,
            library,
            version: None,
            effects: HashSet::new(),
            taint_info: HashSet::new(),
            contract: None,
            thread_safe: true,
            terminates: true,
            param_count: None,
            return_type: None,
        }
    }

    /// Add effect.
    pub fn with_effect(mut self, effect: EffectKind) -> Self {
        self.effects.insert(effect);
        self
    }

    /// Add taint information.
    pub fn with_taint(mut self, taint: TaintKind) -> Self {
        self.taint_info.insert(taint);
        self
    }

    /// Set contract specification.
    pub fn with_contract(mut self, contract: ContractSpec) -> Self {
        self.contract = Some(contract);
        self
    }

    /// Mark as not thread-safe.
    pub fn not_thread_safe(mut self) -> Self {
        self.thread_safe = false;
        self
    }

    /// Mark as potentially non-terminating.
    pub fn may_not_terminate(mut self) -> Self {
        self.terminates = false;
        self
    }

    /// Set parameter count.
    pub fn with_param_count(mut self, count: usize) -> Self {
        self.param_count = Some(count);
        self
    }

    /// Set return type.
    pub fn with_return_type(mut self, return_type: String) -> Self {
        self.return_type = Some(return_type);
        self
    }

    /// Check if function is a taint source.
    pub fn is_taint_source(&self) -> bool {
        self.taint_info.contains(&TaintKind::Source)
    }

    /// Check if function is a taint sink.
    pub fn is_taint_sink(&self) -> bool {
        self.taint_info.contains(&TaintKind::Sink)
    }

    /// Check if function has side effects.
    pub fn has_side_effects(&self) -> bool {
        !self.effects.is_empty() && !self.effects.contains(&EffectKind::Pure)
    }

    /// Get security risk level.
    pub fn security_risk_level(&self) -> SecurityRiskLevel {
        if self.is_taint_sink() {
            SecurityRiskLevel::High
        } else if self.is_taint_source() {
            SecurityRiskLevel::Medium
        } else if self.has_side_effects() {
            SecurityRiskLevel::Low
        } else {
            SecurityRiskLevel::None
        }
    }
}

/// Security risk assessment for external functions.
#[derive(Debug, Clone, PartialEq)]
pub enum SecurityRiskLevel {
    None,
    Low,
    Medium,
    High,
}