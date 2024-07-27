pub mod models;
pub mod utils;

use std::fmt::Debug;

use wasm_bindgen::prelude::*;

extern crate web_sys;

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
