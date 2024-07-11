//! Test suite for the Web and headless browsers.

#![cfg(target_arch = "wasm32")]

extern crate wasm_bindgen_test;
use wasm_bindgen_test::*;

extern crate unocalc;
use unocalc::utils::operations::Operation;
use unocalc::Calculator;

wasm_bindgen_test_configure!(run_in_browser);

#[cfg(test)]
#[wasm_bindgen_test]
fn test_input_digit() {
    let mut calc = Calculator::new();

    calc.input_digit(5);
    assert_eq!(calc.current_value, 5.0);

    calc.input_digit(3);
    assert_eq!(calc.current_value, 53.0);

    calc.input_digit(7);
    assert_eq!(calc.current_value, 537.0);

    calc.input_decimal();
    calc.input_digit(2);
    assert_eq!(calc.current_value, 537.2);
}

#[cfg(test)]
#[wasm_bindgen_test]
fn test_input_decimal() {
    let mut calc = Calculator::new();

    // Input decimal point
    calc.input_digit(5);
    calc.input_decimal();
    calc.input_digit(3);
    assert_eq!(calc.current_value, 5.3);

    // Should not add decimal point multiple times
    calc.input_decimal();
    assert_eq!(calc.current_value, 5.3);

    calc.input_decimal();
    calc.input_digit(7);
    assert_eq!(calc.current_value, 5.37);

    calc.input_decimal();
    calc.input_digit(4);
    assert_eq!(calc.current_value, 5.374);
}

// /*****************  Math Operation Tests *****************/
// #[wasm_bindgen_test]
// fn test_calculator_addition() {
//     let mut calc = Calculator::new();

//     // Simple addition
//     calc.input_digit(5);
//     calc.input_operation(Operation::Add);
//     calc.input_digit(3);
//     assert_eq!(calc.calculate(), Some(8.0));
// }
// #[wasm_bindgen_test]
// fn test_calculator_subtraction() {
//     let mut calc = Calculator::new();

//     // Simple subtraction
//     calc.input_digit(5);
//     calc.input_operation(Operation::Subtract);
//     calc.input_digit(3);
//     assert_eq!(calc.calculate(), Some(2.0));
// }
// #[wasm_bindgen_test]
// fn test_calculator_multiplication() {
//     let mut calc = Calculator::new();

//     // Simple multiplication
//     calc.input_digit(5);
//     calc.input_operation(Operation::Multiply);
//     calc.input_digit(3);
//     assert_eq!(calc.calculate(), Some(15.0));
// }
// #[wasm_bindgen_test]
// fn test_calculator_division() {
//     let mut calc = Calculator::new();

//     // Simple division
//     calc.input_digit(6);
//     calc.input_operation(Operation::Divide);
//     calc.input_digit(3);
//     assert_eq!(calc.calculate(), Some(2.0));

//     // Division by zero
//     calc.input_digit(0);
//     assert_eq!(calc.calculate(), None);
// }
// /*****************  Math Operation Tests *****************/
// #[wasm_bindgen_test]
// fn test_input_operation() {
//     let mut calc = Calculator::new();

//     calc.input_digit(5);
//     calc.input_operation(Operation::Add);
//     calc.input_digit(3);
//     assert_eq!(calc.calculate(), Some(8.0));

//     calc.input_operation(Operation::Multiply);
//     calc.input_digit(2);
//     assert_eq!(calc.calculate(), Some(16.0));
// }

// #[wasm_bindgen_test]
// fn test_calculate() {
//     let mut calc = Calculator::new();

//     calc.input_digit(5);
//     calc.input_operation(Operation::Add);
//     calc.input_digit(3);
//     assert_eq!(calc.calculate(), Some(8.0));

//     calc.input_operation(Operation::Subtract);
//     calc.input_digit(2);
//     assert_eq!(calc.calculate(), Some(6.0));

//     calc.input_operation(Operation::Multiply);
//     calc.input_digit(2);
//     assert_eq!(calc.calculate(), Some(12.0));

//     calc.input_operation(Operation::Divide);
//     calc.input_digit(4);
//     assert_eq!(calc.calculate(), Some(3.0));
// }

// #[wasm_bindgen_test]
// fn test_calculator_reset() {
//     let mut calc = Calculator::new();

//     // Reset after input
//     calc.input_digit(5);
//     calc.reset();
//     assert_eq!(calc.current_value, 0.0);

//     // Reset after multiple inputs
//     calc.input_digit(5);
//     calc.input_operation(Operation::Add);
//     calc.input_digit(3);
//     assert_eq!(calc.current_value, 0.0);

//     // Reset after operation
//     calc.input_digit(5);
//     calc.input_operation(Operation::Add);
//     calc.input_digit(3);
//     calc.calculate();
//     calc.reset();
//     assert_eq!(calc.current_value, 0.0);

//     assert_eq!(calc.current_value, 0.0);
//     assert_eq!(calc.stored_value, None);
//     assert_eq!(calc.current_operation, None);
// }

// #[test]
// fn test_delete_last_digit() {
//     let mut calc = Calculator::new();

//     // Delete last digit from a single-digit number
//     calc.input_digit(5);
//     calc.delete_last_digit();
//     assert_eq!(calc.current_value, 0.0);

//     // Delete last digit from a multi-digit number
//     calc.input_digit(1);
//     calc.input_digit(2);
//     calc.input_digit(3);
//     calc.delete_last_digit();
//     assert_eq!(calc.current_value, 12.0);

//     // Delete last digit after decimal point
//     calc.input_decimal();
//     calc.input_digit(5);
//     calc.delete_last_digit();
//     assert_eq!(calc.current_value, 12.0); // Should maintain the integer part

//     // Delete when current value is already zero
//     calc.delete_last_digit();
//     assert_eq!(calc.current_value, 0.0); // Should remain zero
// }
