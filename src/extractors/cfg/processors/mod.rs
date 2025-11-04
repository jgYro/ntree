/// Block and expression processors for CFG construction.

pub mod basic_block_builder;
pub mod basic_block_processor;
pub mod loop_handler;
pub mod process_block;
pub mod process_expression;
pub mod terminator_handler;

pub use basic_block_processor::build_basic_blocks_from_block;
pub use process_block::process_block;
