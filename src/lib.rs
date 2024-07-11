pub mod utils;

use crate::utils::operations::Operation;
use wasm_bindgen::prelude::*;

#[wasm_bindgen]
#[derive(Default)]
pub struct Calculator {
    pub current_value: f64,
    pub stored_value: Option<f64>,
    pub current_operation: Option<Operation>,
    pub has_decimal: bool,
}

impl Calculator {
    pub fn new() -> Self {
        utils::console::set_panic_hook();
        Calculator {
            current_value: 0.0,
            stored_value: None,
            current_operation: None,
            has_decimal: false, // Initialize to false since no decimal has been entered
        }
    }

    pub fn calculate(&mut self) -> Option<f64> {
        use Operation::*;

        if let (Some(stored_value), Some(operation)) =
            (self.stored_value, self.current_operation)
        {
            match operation {
                Add => Some(stored_value + self.current_value),
                Subtract => Some(stored_value - self.current_value),
                Multiply => Some(stored_value * self.current_value),
                Divide => {
                    if self.current_value != 0.0 {
                        Some(stored_value / self.current_value)
                    } else {
                        None // Handle division by zero
                    }
                },
            }
        } else {
            None
        }
    }
    pub fn input_operation(&mut self, operation: Operation) {
        if self.stored_value.is_some() && self.current_operation.is_some() {
            self.calculate();
        }

        self.stored_value = Some(self.current_value);
        self.current_value = 0.0;
        self.has_decimal = false;

        self.current_operation = Some(operation);
    }

    pub fn input_digit(&mut self, digit: u8) {
        if self.has_decimal {
            self.current_value += digit as f64 * 0.1;
        } else if self.current_value == 0.0 {
            self.current_value = digit as f64;
        } else {
            self.current_value = self.current_value * 10.0 + digit as f64;
        }
    }
    pub fn input_decimal(&mut self) {
        if !self.has_decimal {
            self.has_decimal = true;
        }
    }

    pub fn reset(&mut self) {
        self.current_value = 0.0;
        self.stored_value = None;
        self.current_operation = None;
        self.has_decimal = false;
    }
    // pub fn delete_last_digit(&mut self) {
    //     self.current_value = (self.current_value / 10.0).floor();
    // }
    pub fn delete_last_digit(&mut self) {
        // Convert current_value to string to handle both integer and floating-point numbers
        let current_value = self.current_value.to_string();

        // Determine the new value after deleting the last digit
        if current_value.len() > 1 {
            // If there are multiple characters, remove the last one
            let new_value =
                current_value[..current_value.len() - 1].to_string();
            self.current_value = new_value.parse().unwrap_or(0.0); // Parse back to f64
        } else {
            // If there's only one character left, set current_value to 0.0
            self.current_value = 0.0;
        }

        // Handle case where current_value was zero and input had decimal flag set
        if self.current_value == 0.0 && self.has_decimal {
            self.has_decimal = false; // Reset decimal flag if current_value becomes zero
        }
    }
}
