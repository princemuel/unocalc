use unocalc::utils::operations::Operation;
use unocalc::Calculator;

#[test]
fn test_input_digit() {
    let mut calc = Calculator::new();

    // Input single digit
    calc.input_digit(5);
    assert_eq!(calc.current_value, 5.0);

    // Input multiple digits
    calc.input_digit(3);
    assert_eq!(calc.current_value, 53.0);
}

#[test]
fn test_input_decimal() {
    let mut calc = Calculator::new();

    // Input decimal point
    calc.input_digit(5);
    calc.input_decimal();
    calc.input_digit(3);
    assert_eq!(calc.current_value, 5.3);

    // Test decimal point handling
    calc.input_decimal();
    calc.input_digit(7);
    assert_eq!(calc.current_value, 5.37); // Should not add decimal point multiple times
}

#[test]
fn test_input_operation() {
    let mut calc = Calculator::new();

    // Addition test
    calc.input_digit(5);
    calc.input_operation(Operation::Add);
    calc.input_digit(3);
    assert_eq!(calc.calculate(), Some(8.0));

    // Subtraction test
    calc.input_operation(Operation::Subtract);
    calc.input_digit(2);
    assert_eq!(calc.calculate(), Some(6.0));

    // Multiplication test
    calc.input_operation(Operation::Multiply);
    calc.input_digit(4);
    assert_eq!(calc.calculate(), Some(24.0));

    // Division test
    calc.input_operation(Operation::Divide);
    calc.input_digit(3);
    assert_eq!(calc.calculate(), Some(8.0));

    // Division by zero test
    calc.input_digit(0);
    assert_eq!(calc.calculate(), None);
}

#[test]
fn test_reset() {
    let mut calc = Calculator::new();

    // Reset after input
    calc.input_digit(5);
    calc.reset();
    assert_eq!(calc.current_value, 0.0);

    // Reset after operation
    calc.input_digit(5);
    calc.input_operation(Operation::Add);
    calc.reset();
    assert_eq!(calc.current_value, 0.0);
    assert_eq!(calc.stored_value, None);
    assert_eq!(calc.current_operation, None);
}

#[test]
fn test_delete_last_digit() {
    let mut calc = Calculator::new();

    // Delete last digit from a single-digit number
    calc.input_digit(5);
    calc.delete_last_digit();
    println!("Result: {:?}", calc.current_value);
    assert_eq!(calc.current_value, 0.0);

    // Delete last digit from a multi-digit number
    calc.input_digit(1);
    calc.input_digit(2);
    calc.input_digit(3);
    calc.delete_last_digit();
    assert_eq!(calc.current_value, 12.0);

    // Delete last digit after decimal point
    calc.input_decimal();
    calc.input_digit(5);
    calc.delete_last_digit();
    assert_eq!(calc.current_value, 12.0); // Should maintain the integer part

    // Delete when current value is already zero
    calc.delete_last_digit();
    assert_eq!(calc.current_value, 0.0); // Should remain zero
}
