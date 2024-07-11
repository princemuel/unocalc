//! Test suite for the Web and headless browsers.

#![cfg(target_arch = "wasm32")]

extern crate unocalc;
extern crate wasm_bindgen_test;

use unocalc::operation::Operation;
use unocalc::Calculator;
use wasm_bindgen_test::*;

wasm_bindgen_test_configure!(run_in_browser);

#[wasm_bindgen_test]
fn test_input_digit() {
    let mut calc = Calculator::new();

    calc.input_digit(5);
    calc.input_digit(3);
    assert_eq!(calc.current_value, 53.0);

    calc.input_digit(7);
    assert_eq!(calc.current_value, 537.0);

    calc.input_decimal();
    calc.input_digit(2);
    assert_eq!(calc.current_value, 537.2);
}

#[wasm_bindgen_test]
fn test_input_decimal() {
    let mut calc = Calculator::new();

    calc.input_digit(5);
    calc.input_decimal();
    calc.input_digit(3);
    assert_eq!(calc.current_value, 5.3);
}

#[wasm_bindgen_test]
fn test_input_operation() {
    let mut calc = Calculator::new();

    calc.input_digit(5);
    calc.input_operation(Operation::Add);
    calc.input_digit(3);
    assert_eq!(calc.calculate(), Some(8.0));

    calc.input_operation(Operation::Multiply);
    calc.input_digit(2);
    assert_eq!(calc.calculate(), Some(16.0));
}

#[wasm_bindgen_test]
fn test_calculate() {
    let mut calc = Calculator::new();

    calc.input_digit(5);
    calc.input_operation(Operation::Add);
    calc.input_digit(3);
    assert_eq!(calc.calculate(), Some(8.0));

    calc.input_operation(Operation::Subtract);
    calc.input_digit(2);
    assert_eq!(calc.calculate(), Some(6.0));

    calc.input_operation(Operation::Multiply);
    calc.input_digit(2);
    assert_eq!(calc.calculate(), Some(12.0));

    calc.input_operation(Operation::Divide);
    calc.input_digit(4);
    assert_eq!(calc.calculate(), Some(3.0));
}

#[wasm_bindgen_test]
fn test_reset() {
    let mut calc = Calculator::new();

    calc.input_digit(5);
    calc.input_operation(Operation::Add);
    calc.input_digit(3);
    calc.reset();

    assert_eq!(calc.current_value, 0.0);
    assert_eq!(calc.stored_value, None);
    assert_eq!(calc.current_operation, None);
    assert_eq!(calc.has_decimal, false);
}

#[wasm_bindgen_test]
fn test_delete_last_digit() {
    let mut calc = Calculator::new();

    calc.input_digit(5);
    calc.input_digit(3);
    calc.delete_last_digit();
    assert_eq!(calc.current_value, 5.0);
}
