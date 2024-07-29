extern crate cfg_if;
extern crate wasm_bindgen;
extern crate web_sys;

pub mod schema;
pub mod utils;

use once_cell::sync::Lazy;
use regex::Regex;
use wasm_bindgen::prelude::*;

use schema::Token;

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
    let tokens = parse_to_tokens(&input);
    console_log!("tokens: {tokens:?}",);

    // validate token sequence (e.g. check for mismatched parentheses)
    if !validate_tokens(&tokens) {
        return "0".to_string();
    }

    // Convert to RPN (postfix notation) if needed
    let rpn = convert_to_rpn(&tokens);

    // Evaluate RPN expression or infix expression
    let result = evaluate_rpn(&rpn);

    // return result as string
    result.to_string()
}

pub fn parse_to_tokens(input: &str) -> Vec<Token> {
    static RE: Lazy<Regex> = Lazy::new(|| {
        Regex::new(
            r"(?x)
        (?P<number>-?\d*\.?\d+([eE][+-]?\d+)?) |  # numbers, including floats
        (?P<operator>[+\-*/]) |                   # operators
        (?P<paren>[()])                           # parentheses
    ",
        )
        .unwrap()
    });

    RE.captures_iter(input)
        .filter_map(|captures| {
            captures
                .get(0)
                .and_then(|value| value.as_str().parse::<Token>().ok())
            // .and_then(|value| Token::from_str(value.as_str()).ok())
        })
        .collect()
}
// pub fn parse_to_tokens(input: &str) -> Vec<Token> {
//     input
//         .split_whitespace()
//         .filter_map(|value| value.parse::<Token>().ok())
//         .collect()
// }

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
    use Token::*;

    let mut output_queue: Vec<Token> = Vec::new();
    let mut operator_stack: Vec<Token> = Vec::new();

    tokens.iter().for_each(|token| match token {
        Number(_) => output_queue.push(*token),
        Token::Operator(_) => {
            while operator_stack.last().map_or(false, |top| {
                top.precedence() >= token.precedence()
                    && token.is_left_associative()
            }) {
                output_queue.push(operator_stack.pop().unwrap());
            }
            operator_stack.push(*token);
        },
        Paren(true) => operator_stack.push(*token),
        Paren(false) => {
            while operator_stack.last() != Some(&Paren(true)) {
                output_queue.push(operator_stack.pop().unwrap());
            }
            operator_stack.pop(); // Pop the '('
        },
    });

    while let Some(op) = operator_stack.pop() {
        output_queue.push(op);
    }

    output_queue
}

pub fn evaluate_rpn(tokens: &[Token]) -> f64 {
    // Implement RPN evaluation here
    0.0
}
