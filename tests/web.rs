//! Test suite for the Web and headless browsers.

// #![cfg(target_arch = "wasm32")]

extern crate wasm_bindgen_test;
use wasm_bindgen_test::*;

extern crate unocalc;

wasm_bindgen_test_configure!()


#[wasm_bindgen_test]
fn tester(){
    test_fn();
}

#[cfg(test)]
pub fn test_fn()  {}
