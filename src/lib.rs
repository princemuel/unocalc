extern crate cfg_if;
extern crate wasm_bindgen;
extern crate web_sys;

pub mod expr_node;
pub mod lexer;
pub mod parser;
pub mod schema;
pub mod utils;

use wasm_bindgen::prelude::*;

// A macro to provide `println!(..)`-style syntax for `console.log` logging.
macro_rules! console_log {
    ( $( $t:tt )* ) => {
        web_sys::console::log_1(&format!( $( $t )* ).into());
    }
}

#[wasm_bindgen]

pub fn evaluate_expression(input: String) -> String {
    utils::set_panic_hook();

    console_log!("{}", input);

    input
}
