//! Test suite for the Web and headless browsers.

// #![cfg(target_arch = "wasm32")]

extern crate wasm_bindgen_test;
use unocalc::parse_to_tokens;
use unocalc::schema::Token;
use unocalc::validate_tokens;
use wasm_bindgen_test::*;

extern crate unocalc;

wasm_bindgen_test_configure!();

#[wasm_bindgen_test]
fn test_parse_to_tokens() {
    use Token::*;

    // on valid expression
    let input = "-5 + 3.12 * ( 2 - 1 )";
    let expected = vec![
        Number(-5.0),
        Operator('+'),
        Number(3.12),
        Operator('*'),
        Paren(true),
        Number(2.0),
        Operator('-'),
        Number(1.0),
        Paren(false),
    ];
    assert_eq!(parse_to_tokens(input), expected);

    // on extra whitespace
    let input = "  -5   +  3.12 *  ( 2 - 1 ) ";
    let expected = vec![
        Token::Number(-5.0),
        Token::Operator('+'),
        Token::Number(3.12),
        Token::Operator('*'),
        Token::Paren(true),
        Token::Number(2.0),
        Token::Operator('-'),
        Token::Number(1.0),
        Token::Paren(false),
    ];
    assert_eq!(parse_to_tokens(input), expected);

    // on single number
    let input = "42";
    let expected = vec![Token::Number(42.0)];
    assert_eq!(parse_to_tokens(input), expected);

    // on invalid token
    let input = "-5 + 3.12 * ( 2 - 1 ) e";
    let expected = vec![
        Token::Number(-5.0),
        Token::Operator('+'),
        Token::Number(3.12),
        Token::Operator('*'),
        Token::Paren(true),
        Token::Number(2.0),
        Token::Operator('-'),
        Token::Number(1.0),
        Token::Paren(false),
    ];
    assert_eq!(parse_to_tokens(input), expected);

    // on empty string
    let input = "";
    let expected = vec![];
    assert_eq!(parse_to_tokens(input), expected);
}

#[wasm_bindgen_test]
fn test_validate_tokens() {
    use Token::*;

    let tokens = vec![
        Number(-5.0),
        Operator('+'),
        Number(3.12),
        Operator('*'),
        Paren(true),
        Number(2.0),
        Operator('-'),
        Number(1.0),
        Paren(false),
    ];
    assert!(validate_tokens(&tokens));

    // Valid expression
    let tokens = vec![
        Token::Paren(true), // Opening parenthesis
        Token::Paren(true), // Nested opening parenthesis
        Token::Number(1.0),
        Token::Operator('+'),
        Token::Paren(true), // Another nested opening parenthesis
        Token::Number(2.0),
        Token::Operator('-'),
        Token::Number(3.0),
        Token::Paren(false), // Closing nested parenthesis
        Token::Paren(false), // Closing parenthesis for the second opening
        Token::Paren(false), // Closing outer parenthesis
    ];
    assert!(validate_tokens(&tokens));

    // Invalid expression with two operators in a row
    let tokens = vec![
        Number(-5.0),
        Operator('+'),
        Operator('*'), // Invalid: Two operators in a row
        Number(2.0),
    ];
    assert!(!validate_tokens(&tokens));

    // Invalid expression with mismatched parentheses
    let tokens = vec![
        Number(-5.0),
        Operator('+'),
        Number(3.12),
        Paren(true), // Open parenthesis
        Number(2.0),
        // Missing closing parenthesis
    ];
    assert!(!validate_tokens(&tokens));
}

#[cfg(test)]
pub fn test_expression_b() -> String {
    "-5 + 3.12 * ( 2 - 1 )".to_string()
}

#[cfg(test)]
pub fn test_expression_a() -> String {
    "-5+3.12*(2-1)".to_string()
}
