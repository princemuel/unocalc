mod calculator_builder;
mod number;
mod operation;
mod utils;

use crate::calculator_builder::CalculatorBuilder;
use crate::number::Number;
use wasm_bindgen::prelude::*;

#[wasm_bindgen]
pub struct Calculator {
    result: number::Number,
}

impl Calculator {
    pub fn builder() -> CalculatorBuilder {
        CalculatorBuilder::default()
    }

    pub fn get_result(&self) -> Number {
        self.result.clone()
    }
}
