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
