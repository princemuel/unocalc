#[derive(Debug, Clone, PartialEq)]
pub enum Number {
    Int(i64),
    Float(f64),
}

impl From<i64> for Number {
    fn from(value: i64) -> Self {
        Number::Int(value)
    }
}

impl From<f64> for Number {
    fn from(value: f64) -> Self {
        Number::Float(value)
    }
}

impl Number {
    pub fn to_float(&self) -> f64 {
        match self {
            Number::Int(value) => *value as f64,
            Number::Float(value) => *value,
        }
    }
}
