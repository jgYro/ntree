/// Core utilities for CFG processing.

pub mod cfg_context;
pub mod cfg_utils;

pub use cfg_context::CfgContext;
pub use cfg_utils::{get_if_condition, get_if_parts, get_statement_text, is_statement_node};