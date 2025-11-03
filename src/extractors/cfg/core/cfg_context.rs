/// Represents a loop context for break/continue handling.
#[derive(Debug, Clone)]
pub struct LoopContext {
    /// Node ID of the loop condition (for continue)
    pub condition_id: usize,
    /// Node ID where the loop exits (for break)
    pub after_id: usize,
}

/// Context for CFG building that tracks IDs and loop control.
pub struct CfgContext {
    pub next_id: usize,
    pub exit_id: usize,
    /// Stack of active loop contexts for break/continue handling
    pub loop_stack: Vec<LoopContext>,
}

impl CfgContext {
    pub fn new() -> Self {
        CfgContext {
            next_id: 0,
            exit_id: 9999, // High number to avoid conflicts
            loop_stack: Vec::new(),
        }
    }

    pub fn alloc_id(&mut self) -> usize {
        let id = self.next_id;
        self.next_id += 1;
        id
    }

    /// Push a new loop context onto the stack.
    pub fn push_loop(&mut self, condition_id: usize, after_id: usize) {
        self.loop_stack.push(LoopContext {
            condition_id,
            after_id,
        });
    }

    /// Pop the most recent loop context from the stack.
    pub fn pop_loop(&mut self) -> Option<LoopContext> {
        self.loop_stack.pop()
    }

    /// Get the current loop context for break/continue.
    pub fn current_loop(&self) -> Option<&LoopContext> {
        self.loop_stack.last()
    }
}