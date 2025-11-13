//! Variable lifecycle analysis module.
//!
//! This module provides functionality for tracking variable lifecycles
//! through their definition, use, mutation, and eventual disposal.

mod data_flow_integration;
mod utils;
mod variable_extractor;

pub use data_flow_integration::DataFlowIntegrator;
pub use utils::LifecycleUtils;
pub use variable_extractor::VariableExtractor;

// Re-export the main analyzer
pub use super::variable_lifecycle_analyzer::VariableLifecycleAnalyzer;