use serde::{Deserialize, Serialize};

/// Types of early-exit constructs across different languages.
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub enum EarlyExitKind {
    /// Rust: `foo()?` - conditional early return
    #[serde(rename = "try_operator")]
    TryOperator,
    /// Rust: `panic!("msg")` - exceptional termination
    #[serde(rename = "panic_macro")]
    PanicMacro,
    /// Java: `throw new Exception()` - exception throwing
    #[serde(rename = "throw_statement")]
    ThrowStatement,
    /// C/C++: `exit(1)` - program termination
    #[serde(rename = "exit_call")]
    ExitCall,
    /// JavaScript: `throw new Error()` - exception throwing
    #[serde(rename = "throw_expression")]
    ThrowExpression,
    /// Python: `raise Exception()` - exception raising
    #[serde(rename = "raise_statement")]
    RaiseStatement,
}

/// Language-agnostic representation of early-exit constructs.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct EarlyExitIR {
    #[serde(rename = "type")]
    pub exit_type: String,
    pub exit_id: String,
    pub kind: EarlyExitKind,

    /// The expression/call that triggers the early exit
    pub trigger_expr: String,
    /// Optional error message or value
    #[serde(skip_serializing_if = "Option::is_none")]
    pub error_value: Option<String>,
    /// Whether this is conditional (?) or unconditional (panic!)
    pub is_conditional: bool,
}

impl EarlyExitIR {
    /// Create a new try operator (conditional early return).
    pub fn new_try_operator(exit_id: String, trigger_expr: String) -> Self {
        EarlyExitIR {
            exit_type: "EarlyExit".to_string(),
            exit_id,
            kind: EarlyExitKind::TryOperator,
            trigger_expr,
            error_value: None,
            is_conditional: true,
        }
    }

    /// Create a new panic/exception (unconditional exit).
    pub fn new_panic(exit_id: String, trigger_expr: String, error_msg: Option<String>) -> Self {
        EarlyExitIR {
            exit_type: "EarlyExit".to_string(),
            exit_id,
            kind: EarlyExitKind::PanicMacro,
            trigger_expr,
            error_value: error_msg,
            is_conditional: false,
        }
    }

    /// Create a new throw statement (Java/JS style).
    pub fn new_throw(exit_id: String, trigger_expr: String, exception_type: String) -> Self {
        EarlyExitIR {
            exit_type: "EarlyExit".to_string(),
            exit_id,
            kind: EarlyExitKind::ThrowStatement,
            trigger_expr,
            error_value: Some(exception_type),
            is_conditional: false,
        }
    }

    /// Create a new exit call (C/C++ style).
    pub fn new_exit_call(exit_id: String, trigger_expr: String, exit_code: String) -> Self {
        EarlyExitIR {
            exit_type: "EarlyExit".to_string(),
            exit_id,
            kind: EarlyExitKind::ExitCall,
            trigger_expr,
            error_value: Some(exit_code),
            is_conditional: false,
        }
    }

    /// Convert to JSONL format.
    pub fn to_jsonl(&self) -> String {
        match serde_json::to_string(self) {
            Ok(json) => json,
            Err(_) => "{}".to_string(),
        }
    }
}