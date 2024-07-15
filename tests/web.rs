//! Test suite for the Web and headless browsers.

// #![cfg(target_arch = "wasm32")]

extern crate wasm_bindgen_test;
use wasm_bindgen_test::*;

extern crate unocalc;
use unocalc::utils::operation::Operation;
use unocalc::Calculator;

wasm_bindgen_test_configure!(run_in_browser);

#[wasm_bindgen_test]
fn test_addition() {
    let mut calc = Calculator::new();
    calc.set_digit('1');
    calc.set_digit('.');
    calc.set_digit('5');
    calc.set_operation(Operation::Add);
    calc.set_digit('2');
    calc.set_digit('.');
    calc.set_digit('3');
    calc.calculate();
    assert_eq!(calc.get_current_value(), "3.8");
}

#[wasm_bindgen_test]
fn test_subtraction() {
    let mut calc = Calculator::new();
    calc.set_digit('5');
    calc.set_digit('.');
    calc.set_digit('5');
    calc.set_operation(Operation::Subtract);
    calc.set_digit('2');
    calc.calculate();
    assert_eq!(calc.get_current_value(), "3.5");
}

#[wasm_bindgen_test]
fn test_multiplication() {
    let mut calc = Calculator::new();
    calc.set_digit('3');
    calc.set_digit('.');
    calc.set_digit('0');
    calc.set_operation(Operation::Multiply);
    calc.set_digit('2');
    calc.calculate();
    assert_eq!(calc.get_current_value(), "6.0");
}

#[wasm_bindgen_test]
fn test_division() {
    let mut calc = Calculator::new();
    calc.set_digit('6');
    calc.set_digit('.');
    calc.set_digit('0');
    calc.set_operation(Operation::Divide);
    calc.set_digit('2');
    calc.calculate();
    assert_eq!(calc.get_current_value(), "3.0");
}

#[wasm_bindgen_test]
fn test_input_buffer() {
    let mut calc = Calculator::new();
    calc.set_digit('1');
    calc.set_digit('.');
    calc.set_digit('5');
    assert_eq!(calc.get_current_value(), "1.5");
}

#[wasm_bindgen_test]
fn test_reset() {
    let mut calc = Calculator::new();
    calc.set_digit('1');
    calc.set_digit('.');
    calc.set_digit('5');
    calc.reset();
    assert_eq!(calc.get_current_value(), 0.0);
    assert_eq!(calc.get(), "");
    assert!(calc.current_operation().is_none());
}

#[wasm_bindgen_test]
fn test_history() {
    let mut calc = Calculator::new();
    calc.set_digit('2');
    calc.set_operation(Operation::Add);
    calc.set_digit('3');
    calc.calculate();
    let history = calc.history();
    assert_eq!(history.len(), 1);
    let entry = &history[0];
    assert_eq!(entry.operands(), (2.0, Some(3.0)));
    assert_eq!(entry.result(), 5.0);
}

#[wasm_bindgen_test]
fn test_ignore_multiple_decimal_points() {
    let mut calc = Calculator::new();
    calc.set_digit('1');
    calc.set_digit('.');
    calc.set_digit('2');
    calc.set_digit('.');
    calc.set_digit('3');
    assert_eq!(calc.input_buffer(), "1.23");
}

#[wasm_bindgen_test]
fn test_backspace() {
    let mut calc = Calculator::new();
    calc.set_digit('1');
    calc.set_digit('2');
    calc.set_digit('3');
    assert_eq!(calc.input_buffer(), "123");
    calc.backspace();
    assert_eq!(calc.input_buffer(), "12");
    calc.backspace();
    assert_eq!(calc.input_buffer(), "1");
    calc.backspace();
    assert_eq!(calc.input_buffer(), "");
    calc.backspace(); // Should handle gracefully
    assert_eq!(calc.input_buffer(), "");
}

#[wasm_bindgen_test]
fn test_chained_operations() {
    let mut calc = Calculator::new();
    calc.set_digit('2');
    calc.set_operation(Operation::Add);
    calc.set_digit('3');
    calc.set_operation(Operation::Multiply);
    calc.set_digit('4');
    calc.calculate();
    assert_eq!(calc.get_current_value(), 20.0); // (2 + 3) * 4 = 20
}

#[wasm_bindgen_test]
fn test_chained_operations_with_equals() {
    let mut calc = Calculator::new();
    calc.set_digit('2');
    calc.set_operation(Operation::Add);
    calc.set_digit('3');
    calc.calculate(); // 2 + 3 = 5
    assert_eq!(calc.get_current_value(), 5.0);
    calc.set_operation(Operation::Multiply);
    calc.set_digit('4');
    calc.calculate(); // 5 * 4 = 20
    assert_eq!(calc.get_current_value(), "20.0");
}

#[wasm_bindgen_test]
fn test_division_by_zero() {
    let mut calc = Calculator::new();
    calc.set_digit('6');
    calc.set_operation(Operation::Divide);
    calc.set_digit('0');
    calc.calculate();
    assert_eq!(calc.get_current_value(), "0.0"); // Should return 0
}
