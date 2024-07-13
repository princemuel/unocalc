pub mod utils;

use utils::types::{HistoryEntry, Operation};
use wasm_bindgen::prelude::*;

#[wasm_bindgen]
#[derive(Default)]
pub struct Calculator {
    current_value: f64,
    current_operation: Option<Operation>,
    input_buffer: String,
    history: Vec<HistoryEntry>,
}

#[wasm_bindgen]
impl Calculator {
    pub fn new() -> Calculator {
        utils::console::set_panic_hook();

        Calculator {
            current_value: 0.0,
            current_operation: None,
            input_buffer: String::new(),
            history: Vec::new(),
        }
    }

    pub fn reset(&mut self) {
        self.current_value = 0.0;
        self.current_operation = None;
        self.input_buffer.clear();
    }
    pub fn backspace(&mut self) {
        self.input_buffer.pop();
    }

    pub fn input_digit(&mut self, value: char) {
        if value == '.' && self.input_buffer.contains('.') {
            return; // Ignore additional decimal points if one is already present
        }
        self.input_buffer.push(value);
    }
    pub fn set_operation(&mut self, operation: Operation) {
        if let Ok(value) = self.input_buffer.parse::<f64>() {
            self.calculate_with_operand(value);
        }
        self.current_operation = Some(operation);
        self.input_buffer.clear();
    }

    pub fn calculate(&mut self) {
        if let Ok(value) = self.input_buffer.parse::<f64>() {
            self.calculate_with_operand(value);
            self.input_buffer.clear();
        }
        self.current_operation = None;
    }

    fn calculate_with_operand(&mut self, operand: f64) {
        if let Some(operation) = self.current_operation {
            use Operation::*;
            let result = match operation {
                Add => self.current_value + operand,
                Subtract => self.current_value - operand,
                Multiply => self.current_value * operand,
                Divide => {
                    if operand == 0.0 {
                        0.0 // Return 0 if division by zero
                    } else {
                        self.current_value / operand
                    }
                },
            };

            self.history.push(HistoryEntry::new(
                operation,
                self.current_value,
                Some(operand),
                result,
            ));
            self.current_value = result;
        } else {
            self.current_value = operand;
        }
    }

    // Getters
    pub fn current_value(&self) -> f64 {
        self.current_value
    }
    pub fn current_operation(&self) -> Option<Operation> {
        self.current_operation
    }
    pub fn input_buffer(&self) -> String {
        self.input_buffer.clone()
    }
    pub fn history(&self) -> Vec<HistoryEntry> {
        self.history.clone()
    }
}
