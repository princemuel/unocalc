#[derive(Debug, Clone, PartialEq)]
pub enum Token {
    Number(f64),
    Operator(char),
    Paren(bool),
    // Function(String),  // e.g., "sin", "cos", "sqrt"
    // Variable(String),  // e.g., "x", "y"
    // Constant(String),  // e.g., "pi", "e"
}

impl std::str::FromStr for Token {
    type Err = String;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s {
            "+" | "-" | "*" | "/" => {
                Ok(Token::Operator(s.chars().next().unwrap()))
            },
            "(" => Ok(Token::Paren(true)),
            ")" => Ok(Token::Paren(false)),
            _ => s
                .parse::<f64>()
                .map(Token::Number)
                .map_err(|_| format!("Invalid Token: {s}")),
        }
    }
}
