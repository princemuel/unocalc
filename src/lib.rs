pub mod utils;

use utils::operation::Operation;
use wasm_bindgen::prelude::*;

extern crate web_sys;

// web_sys::console::error_1()
// web_sys::console::log_1()
// A macro to provide `println!(..)`-style syntax for `console.log` logging.
macro_rules! log {
    ( $( $t:tt )* ) => {
        web_sys::console::log_1(&format!( $( $t )* ).into());
    }
}

#[wasm_bindgen]
#[derive(Default)]
pub struct Calculator {
    current_value: String,
    input_buffer: String,
    last_operation: Option<Operation>,
}

#[wasm_bindgen]
impl Calculator {
    #[wasm_bindgen(constructor)]
    pub fn new() -> Self {
        utils::console::set_panic_hook();
        Self {
            current_value: String::new(),
            input_buffer: String::new(),
            last_operation: None,
        }
    }

    pub fn calculate(&mut self) {
        log!("The current expression is {:?}", self.input_buffer);
        // Check if the input buffer is empty or only contains an operator
        if self.input_buffer.is_empty()
            || (self.last_operation.is_some() && self.current_value.is_empty())
        {
            self.current_value.clear();
            return;
        }

        // Append the current value to the input buffer before evaluation
        if !self.current_value.is_empty() {
            self.input_buffer.push_str(&self.current_value);
        }

        // return result on success or zero on error
        let result = exmex::eval_str(&self.input_buffer).unwrap_or(0.0);
        log!("The result is {:?}", result.to_string());

        self.current_value = result.to_string();
        self.input_buffer.clear();
        self.last_operation = None; // Reset last operation after calculation
    }

    pub fn set_digit(&mut self, value: char) {
        if value == '.'
            && (self.current_value.contains('.')
                || self.current_value.is_empty())
        {
            return; // Ignore additional decimal points or leading decimal points
        }
        self.current_value.push(value);
        log!("The current value is {:?}", self.current_value);
    }

    pub fn set_operation(&mut self, operation: Operation) {
        use Operation::*;
        let operator = match operation {
            Add => '+',
            Subtract => '-',
            Multiply => '*',
            Divide => '/',
        };

        if !self.current_value.is_empty() {
            self.input_buffer.push_str(&self.current_value);
            self.input_buffer.push(operator);
            self.current_value.clear();
        } else if self.last_operation.is_some() {
            // Replace last operator if no operand is entered
            self.input_buffer.pop();
            self.input_buffer.push(operator);
        } else {
            // Append operator if there's no current value and no last operation
            self.input_buffer.push(operator);
        }

        self.last_operation = Some(operation);
    }

    pub fn backspace(&mut self) {
        self.current_value.pop();
    }
    pub fn reset(&mut self) {
        self.current_value.clear();
        self.input_buffer.clear();
        self.last_operation = None;
    }

    pub fn get_current_value(&self) -> String {
        self.current_value.clone()
    }

    pub fn get_input_buffer(&self) -> String {
        self.input_buffer.clone()
    }
}
