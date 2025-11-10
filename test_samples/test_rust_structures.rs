use std::collections::HashMap;

pub struct Calculator {
    name: String,
    history: Vec<String>,
}

impl Calculator {
    pub fn new(name: String) -> Self {
        Calculator {
            name,
            history: Vec::new(),
        }
    }

    pub fn add(&mut self, a: i32, b: i32) -> i32 {
        let result = a + b;
        self.log_operation("add", a, b, result);
        result
    }

    fn log_operation(&mut self, op: &str, a: i32, b: i32, result: i32) {
        let entry = format!("{}: {}({}, {}) = {}", self.name, op, a, b, result);
        self.history.push(entry);
    }
}

pub enum Operation {
    Add(i32, i32),
    Multiply(i32, i32),
    Divide(i32, i32),
}

impl Operation {
    pub fn execute(&self) -> i32 {
        match self {
            Operation::Add(a, b) => a + b,
            Operation::Multiply(a, b) => a * b,
            Operation::Divide(a, b) => a / b,
        }
    }
}

pub fn process_operations(ops: Vec<Operation>) -> Vec<i32> {
    ops.iter().map(|op| op.execute()).collect()
}