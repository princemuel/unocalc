use wasm_bindgen::prelude::wasm_bindgen;

#[wasm_bindgen]
#[derive(Debug, Clone, Copy, PartialEq)]
pub enum Operation {
    Add,
    Subtract,
    Multiply,
    Divide,
}

impl Operation {
    pub fn from_char(value: char) -> Option<Self> {
        use Operation::*;
        match value {
            '+' => Some(Add),
            '-' => Some(Subtract),
            '*' => Some(Multiply),
            '/' => Some(Divide),
            _ => None,
        }
    }
}
