extern crate cfg_if;
extern crate wasm_bindgen;
extern crate web_sys;

pub mod schema;
pub mod utils;

use schema::Token;

use wasm_bindgen::prelude::*;

// A macro to provide `println!(..)`-style syntax for `console.log` logging.
macro_rules! console_log {
    ( $( $t:tt )* ) => {
        web_sys::console::log_1(&format!( $( $t )* ).into());
    }
}

#[wasm_bindgen]
pub fn calculate(input: String) -> String {
    utils::set_panic_hook();
    // parse input to Vec<Token>
    let tokens = match parse_to_tokens(&input) {
        Ok(tokens) => tokens,
        Err(e) => return format!("Error: {}", e),
    };

    console_log!("tokens: {tokens:?}",);

    // validate token sequence (e.g. check for mismatched parentheses)

    // convert to RPN if needed

    // evaluate RPN or infix expression

    // return result as string
    // Basic validation example
    if !validate_tokens(&tokens) {
        return "0".to_string();
    }

    // Convert to RPN (postfix notation) if needed
    let rpn = convert_to_rpn(&tokens);

    // Evaluate RPN expression
    let result = evaluate_rpn(&rpn);

    result.to_string()
}

pub fn parse_to_tokens(input: &str) -> Result<Vec<Token>, String> {
    input.split_whitespace().map(|value| value.parse::<Token>()).collect()
}

pub fn validate_tokens(tokens: &[Token]) -> bool {
    use Token::*;

    let (tokens_are_valid, parens_state, last_value_was_operator) =
        tokens.iter().fold(
            (true, 0u8, true),
            |(is_valid, paren_count, last_val_was_op), token| match token {
                Number(_) => (is_valid, paren_count, false),
                Operator(_) => {
                    (is_valid && !last_val_was_op, paren_count, true)
                },

                Paren(is_open_paren) => {
                    let (count, is_valid_parens) =
                        match (is_open_paren, paren_count) {
                            (true, _) => (paren_count + 1, true), // Increment for opening parenthesis
                            (false, 0) => (paren_count, false), // Invalid: more closing than opening
                            (false, _) => (paren_count - 1, true), // Decrement for closing parenthesis
                        };

                    (is_valid_parens && is_valid, count, false)
                },
            },
        );
    // Valid if there are no unmatched parentheses and the last token is not an operator
    tokens_are_valid && parens_state == 0 && !last_value_was_operator
}

pub fn convert_to_rpn(tokens: &[Token]) -> Vec<Token> {
    // Implement conversion to RPN here
    tokens.to_vec()
}

pub fn evaluate_rpn(tokens: &[Token]) -> f64 {
    // Implement RPN evaluation here
    0.0
}
