use std::str::FromStr;

use wasm_bindgen::prelude::wasm_bindgen;

#[wasm_bindgen]
#[repr(u8)]
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum Operation {
    Add = 1,
    Subtract = 2,
    Multiply = 3,
    Divide = 4,
}

#[derive(Debug, Clone, PartialEq)]
pub enum Token {
    Number(f64),
    Operator(char),
    Paren(bool),
}

type ParseTokenError = String;

impl FromStr for Token {
    type Err = ParseTokenError;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s {
            "+" | "-" | "*" | "/" => {
                Ok(Token::Operator(s.chars().next().unwrap()))
            },
            "(" => Ok(Token::Paren(true)),
            ")" => Ok(Token::Paren(false)),
            _ => {
                if let Ok(number) = s.parse::<f64>() {
                    Ok(Token::Number(number))
                } else {
                    Err(format!("Invalid Token: {s}"))
                }
            },
        }
    }
}
