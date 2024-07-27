//! Test suite for the Web and headless browsers.

// #![cfg(target_arch = "wasm32")]

extern crate wasm_bindgen_test;
use unocalc::parse_to_tokens;
use unocalc::schema::Token;
use wasm_bindgen_test::*;

extern crate unocalc;

wasm_bindgen_test_configure!();

#[wasm_bindgen_test]
fn test_parse_tokens() {
    use Token::*;

    assert_eq!(
        parse_to_tokens(test_expression_b()),
        Ok(vec![
            Number(-5.0),
            Operator('+'),
            Number(3.14),
            Operator('*'),
            Paren(true),
            Number(2.0),
            Operator('-'),
            Number(1.0),
            Paren(false)
        ])
    );
}

#[cfg(test)]
pub fn test_expression_a() -> String {
    format!("6+(4.5*(8+3/1.5)−2)/4")
}

#[cfg(test)]
pub fn test_expression_b() -> String {
    format!("-5 + 3.14 * ( 2 - 1 )")
}
