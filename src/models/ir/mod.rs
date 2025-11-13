/// Intermediate Representation models for language-agnostic analysis.
pub mod basic_block;
pub mod basic_block_graph;
pub mod cfg_ir;
pub mod early_exit;
pub mod loop_ir;

pub use basic_block::{BasicBlock, BasicBlockEdge};
pub use basic_block_graph::BasicBlockGraph;
pub use cfg_ir::{CFGEdgeIR, CFGNodeIR, FunctionCFGIR};
pub use early_exit::{EarlyExitIR, EarlyExitKind};
pub use loop_ir::{ForLoopIR, LoopKind};
