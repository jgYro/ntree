use crate::storage::SymbolId;
use serde::{Deserialize, Serialize};

/// Unique identifier for call sites.
pub type CallSiteId = usize;

/// Type instantiation record for RTA analysis.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct TypeInstantiated {
    /// Type/class that was instantiated
    pub type_id: SymbolId,
    /// Location where instantiation occurred
    pub site: String,
    /// File where instantiation occurred
    pub file_path: String,
    /// Whether instantiation is reachable
    pub reachable: bool,
}

impl TypeInstantiated {
    /// Create new type instantiation record.
    pub fn new(type_id: SymbolId, site: String, file_path: String) -> Self {
        TypeInstantiated {
            type_id,
            site,
            file_path,
            reachable: false,
        }
    }

    /// Mark as reachable.
    pub fn mark_reachable(&mut self) {
        self.reachable = true;
    }
}

/// Resolution algorithm used for call site.
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub enum ResolutionAlgorithm {
    /// Direct call - single known target
    Direct,
    /// Class Hierarchy Analysis - all possible overrides
    CHA,
    /// Rapid Type Analysis - filtered by instantiated types
    RTA,
    /// Heuristic-based resolution
    Heuristic,
    /// Dynamic - cannot be resolved statically
    Dynamic,
}

/// Call site resolution information.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Resolution {
    /// Unique call site identifier
    pub callsite_id: CallSiteId,
    /// Possible target functions
    pub targets: Vec<SymbolId>,
    /// Resolution algorithm used
    pub algorithm: ResolutionAlgorithm,
    /// Confidence level (0.0 - 1.0)
    pub confidence: f64,
    /// Caller function
    pub caller: SymbolId,
    /// Call expression text
    pub call_expr: String,
    /// Receiver type (for OO calls)
    pub receiver_type: Option<String>,
}

impl Resolution {
    /// Create new resolution with direct target.
    pub fn direct(
        callsite_id: CallSiteId,
        target: SymbolId,
        caller: SymbolId,
        call_expr: String,
    ) -> Self {
        Resolution {
            callsite_id,
            targets: vec![target],
            algorithm: ResolutionAlgorithm::Direct,
            confidence: 1.0,
            caller,
            call_expr,
            receiver_type: None,
        }
    }

    /// Create CHA resolution with multiple candidates.
    pub fn cha(
        callsite_id: CallSiteId,
        targets: Vec<SymbolId>,
        caller: SymbolId,
        call_expr: String,
    ) -> Self {
        let confidence = if targets.len() == 1 { 0.9 } else { 0.7 };
        Resolution {
            callsite_id,
            targets,
            algorithm: ResolutionAlgorithm::CHA,
            confidence,
            caller,
            call_expr,
            receiver_type: None,
        }
    }

    /// Create RTA resolution with instantiation filtering.
    pub fn rta(
        callsite_id: CallSiteId,
        targets: Vec<SymbolId>,
        caller: SymbolId,
        call_expr: String,
    ) -> Self {
        let confidence = match targets.len() {
            1 => 0.95,
            2..=3 => 0.8,
            _ => 0.6,
        };
        Resolution {
            callsite_id,
            targets,
            algorithm: ResolutionAlgorithm::RTA,
            confidence,
            caller,
            call_expr,
            receiver_type: None,
        }
    }

    /// Create heuristic resolution.
    pub fn heuristic(
        callsite_id: CallSiteId,
        targets: Vec<SymbolId>,
        caller: SymbolId,
        call_expr: String,
    ) -> Self {
        Resolution {
            callsite_id,
            targets,
            algorithm: ResolutionAlgorithm::Heuristic,
            confidence: 0.5,
            caller,
            call_expr,
            receiver_type: None,
        }
    }

    /// Create dynamic/unresolvable call.
    pub fn dynamic(callsite_id: CallSiteId, caller: SymbolId, call_expr: String) -> Self {
        Resolution {
            callsite_id,
            targets: Vec::new(),
            algorithm: ResolutionAlgorithm::Dynamic,
            confidence: 0.0,
            caller,
            call_expr,
            receiver_type: None,
        }
    }

    /// Set receiver type for OO calls.
    pub fn with_receiver_type(mut self, receiver_type: String) -> Self {
        self.receiver_type = Some(receiver_type);
        self
    }

    /// Check if resolution is definitive.
    pub fn is_definitive(&self) -> bool {
        self.targets.len() == 1 && self.confidence > 0.8
    }

    /// Check if resolution is ambiguous.
    pub fn is_ambiguous(&self) -> bool {
        self.targets.len() > 1
    }

    /// Get best target (highest confidence).
    pub fn get_best_target(&self) -> Option<&SymbolId> {
        self.targets.first()
    }
}
