/// Context for CFG building that tracks IDs.
pub struct CfgContext {
    pub next_id: usize,
    pub exit_id: usize,
}

impl CfgContext {
    pub fn new() -> Self {
        CfgContext {
            next_id: 0,
            exit_id: 9999, // High number to avoid conflicts
        }
    }

    pub fn alloc_id(&mut self) -> usize {
        let id = self.next_id;
        self.next_id += 1;
        id
    }
}