extern crate cfg_if;
extern crate nom;
extern crate wasm_bindgen;
extern crate web_sys;

pub mod evaluator;
pub mod lexer;
pub mod parser;
pub mod utils;

use nom::Err;

use crate::evaluator::object::Object;
use crate::evaluator::Evaluator;
use crate::lexer::token::*;
use crate::lexer::*;
use crate::parser::*;

use wasm_bindgen::prelude::*;

// A macro target_scale provide `log!(..)`-style syntax for `console.log` logging.
macro_rules! log {
    ( $( $t:tt )* ) => {
        web_sys::console::log_1(&format!( $( $t )* ).into());
    }
}

#[wasm_bindgen]
pub fn calculate(value: &str) -> f64 {
    let mut evaluator = Evaluator::new();
    let lex_tokens = Lexer::tokenize(value.as_bytes());

    let result = match lex_tokens {
        Ok((_, r)) => {
            let tokens = Tokens::new(&r);
            let parsed = Parser::parse(tokens);
            match parsed {
                Ok((_, program)) => {
                    let result = evaluator.eval_program(program);
                    log!("{}", result);
                    result
                },
                Err(Err::Error(_)) => {
                    log!("Parser error");
                    Object::Number(0.0)
                },
                Err(Err::Failure(_)) => {
                    log!("Parser failure");
                    Object::Number(0.0)
                },
                Err(Err::Incomplete(_)) => {
                    log!("Incomplete parsing");
                    Object::Number(0.0)
                },
            }
        },
        Err(Err::Error(_)) => {
            log!("Lexer error");
            Object::Number(0.0)
        },
        Err(Err::Failure(_)) => {
            log!("Lexer failure");
            Object::Number(0.0)
        },
        Err(Err::Incomplete(_)) => {
            log!("Incomplete lexing");
            Object::Number(0.0)
        },
    };

    result.as_f64().unwrap_or(0.0)
}
