extern crate cfg_if;
extern crate wasm_bindgen;
extern crate web_sys;

pub mod schema;
pub mod utils;

use schema::Token;
use std::fmt::Debug;
use wasm_bindgen::prelude::*;

// A macro to provide `println!(..)`-style syntax for `console.log` logging.
macro_rules! console_log {
    ( $( $t:tt )* ) => {
        web_sys::console::log_1(&format!( $( $t )* ).into());
    }
}
// A macro to provide `println!(..)`-style syntax for `console.error` logging.
macro_rules! console_err {
    ( $( $t:tt )* ) => {
        web_sys::console::error_1(&format!( $( $t )* ).into());
    }
}

#[wasm_bindgen]
#[derive(Default, Debug)]
pub struct Calculator {
    input_buffer: String,
}

#[wasm_bindgen]
impl Calculator {
    #[wasm_bindgen(constructor)]
    pub fn new() -> Self {
        utils::set_panic_hook();
        Self { input_buffer: String::from("0") }
    }
}

pub fn parse_to_tokens(input: String) -> Result<Vec<Token>, String> {
    input.split_whitespace().map(|value| value.parse::<Token>()).collect()
}
