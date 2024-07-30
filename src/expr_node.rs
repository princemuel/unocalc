#[derive(Debug, Clone)]
pub enum ExpressionNode {
    Number(f64),
    UnaryOperator {
        operator: char,
        expression: Box<ExpressionNode>,
    },
    BinaryOperator {
        operator: char,
        left: Box<ExpressionNode>,
        right: Box<ExpressionNode>,
    },
}

impl ExpressionNode {
    pub fn evaluate(&self) -> f64 {
        use ExpressionNode::*;

        match self {
            Number(val) => *val,
            UnaryOperator { operator, expression } => {
                let val = expression.evaluate();
                match operator {
                    '-' => -val,
                    _ => val,
                }
            },
            BinaryOperator { operator, left, right } => {
                let left_val = left.evaluate();
                let right_val = right.evaluate();
                match operator {
                    '+' => left_val + right_val,
                    '-' => left_val - right_val,
                    '*' => left_val * right_val,
                    '/' => left_val / right_val,
                    _ => 0.0,
                }
            },
        }
    }
}
