use wasm_bindgen::prelude::wasm_bindgen;

#[wasm_bindgen]
#[derive(Debug, Clone, Copy, PartialEq)]
pub enum Operation {
    Add = 1,
    Subtract = 2,
    Multiply = 3,
    Divide = 4,
}

impl Operation {
    pub fn from_char(value: char) -> Option<Self> {
        use Operation::*;
        match value {
            '+' => Some(Add),
            '-' => Some(Subtract),
            'x' => Some(Multiply),
            '/' => Some(Divide),
            _ => None,
        }
    }
}
