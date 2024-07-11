pub mod utils;

use utils::operations::Operation;
use wasm_bindgen::prelude::*;

#[wasm_bindgen]
#[derive(Default)]
pub struct Calculator {
    pub current_value: f64,
    pub stored_value: Option<f64>,
    pub current_operation: Option<Operation>,
    pub has_decimal: bool,
    pub decimal_place: u8,
}

impl Calculator {
    pub fn new() -> Self {
        utils::console::set_panic_hook();

        Calculator {
            current_value: 0.0,
            stored_value: None,
            current_operation: None,
            has_decimal: false, // Initialize to false since no decimal has been entered
            decimal_place: 1,
        }
    }

    pub fn calculate(&mut self) -> Option<f64> {
        use Operation::*;

        if let Some(operation) = self.current_operation {
            if let Some(stored_value) = self.stored_value {
                self.current_value = match operation {
                    Add => stored_value + self.current_value,
                    Subtract => stored_value - self.current_value,
                    Multiply => stored_value * self.current_value,
                    Divide => {
                        if self.current_value != 0.0 {
                            stored_value / self.current_value
                        } else {
                            return None; // Cannot divide by zero
                        }
                    },
                };

                // Store the result for the next operation
                self.stored_value = Some(self.current_value);
                self.current_operation = None;
                self.has_decimal = false;
                self.decimal_place = 1;

                return Some(self.current_value);
            }
        }
        None
    }

    pub fn input_operation(&mut self, operation: Operation) {
        // if self.current_operation.is_some() {
        //     self.calculate(); // iterative calc
        // }
        self.current_operation = Some(operation);
        self.stored_value = Some(self.current_value);
        self.current_value = 0.0;
    }

    pub fn input_digit(&mut self, digit: u8) {
        if self.has_decimal {
            self.current_value +=
                digit as f64 / 10f64.powi(self.decimal_place as i32);
            self.decimal_place += 1;
        } else if self.current_value == 0.0 {
            self.current_value = digit as f64;
        } else {
            self.current_value = self.current_value * 10.0 + digit as f64;
        }
    }

    pub fn input_decimal(&mut self) {
        if !self.has_decimal {
            self.has_decimal = true;
            self.decimal_place = 1;
        }
    }

    pub fn reset(&mut self) {
        self.current_value = 0.0;
        self.stored_value = None;
        self.current_operation = None;
        self.has_decimal = false;
        self.decimal_place = 1;
    }

    pub fn delete_last_digit(&mut self) {
        if self.has_decimal {
            // Handle the case where the number has a decimal point
            let mut current_value = self.current_value.to_string();
            if let Some(index_of_dot) = current_value.find('.') {
                current_value = current_value[..index_of_dot].to_string();

                self.current_value =
                    current_value.parse::<f64>().unwrap_or(0.0);

                self.has_decimal = false;
            }
        } else {
            // Handle the case where the number is an integer
            self.current_value = (self.current_value / 10.0).floor();
        }
    }
}
