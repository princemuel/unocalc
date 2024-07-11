use crate::number::Number;
use crate::operation::Operation;
use crate::Calculator;

#[derive(Default)]
pub struct CalculatorBuilder {
    a: Option<Number>,
    b: Option<Number>,
    operation: Option<Operation>,
}

impl CalculatorBuilder {
    pub fn build(self) -> Result<Calculator, String> {
        use Operation::*;

        let a = self
            .a
            .ok_or_else(|| "Operand 'a' is missing".to_string())?
            .to_float();
        let b = self
            .b
            .ok_or_else(|| "Operand 'b' is missing".to_string())?
            .to_float();
        let operation =
            self.operation.ok_or_else(|| "Operator is missing".to_string())?;

        let result = match operation {
            Add => Number::Float(a + b),
            Subtract => Number::Float(a - b),
            _ => todo!(),
        };

        Ok(Calculator { result })
    }

    pub fn a<T: Into<Number>>(mut self, value: T) -> Self {
        self.a = Some(value.into());
        self
    }
    pub fn b<T: Into<Number>>(mut self, value: T) -> Self {
        self.b = Some(value.into());
        self
    }
    pub fn operation(mut self, operation: Operation) -> Self {
        self.operation = Some(operation);
        self
    }
}
