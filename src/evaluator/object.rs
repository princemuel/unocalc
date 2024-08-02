use std::fmt;

#[derive(Debug, Clone, PartialEq)]
pub enum Object {
    Number(f64),
    Error(String),
}

impl Object {
    pub fn as_f64(&self) -> Option<f64> {
        match self {
            Object::Number(value) => Some(*value),
            _ => None,
        }
    }
}

impl fmt::Display for Object {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match *self {
            Object::Number(ref i) => write!(f, "{}", i),
            Object::Error(ref s) => write!(f, "Error: {}", s),
        }
    }
}
