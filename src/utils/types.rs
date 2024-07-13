use wasm_bindgen::prelude::wasm_bindgen;

#[wasm_bindgen]
#[derive(Debug, Clone, Copy, PartialEq)]
pub enum Operation {
    Add = 1,
    Subtract = 2,
    Multiply = 3,
    Divide = 4,
}

#[wasm_bindgen]
#[derive(Debug, Clone)]
pub struct HistoryEntry {
    operation: Operation,
    operands: (f64, Option<f64>),
    result: f64,
}

impl HistoryEntry {
    pub fn new(
        operation: Operation,
        a: f64,
        b: Option<f64>,
        result: f64,
    ) -> HistoryEntry {
        HistoryEntry { operation, operands: (a, b), result }
    }

    pub fn operation(&self) -> &Operation {
        &self.operation
    }

    pub fn operands(&self) -> (f64, Option<f64>) {
        self.operands
    }

    pub fn result(&self) -> f64 {
        self.result
    }
}
