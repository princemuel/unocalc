use crate::expr_node::ExpressionNode;
use crate::schema::Token;

#[derive(Debug, Clone, Copy)]
pub struct Parser<'a> {
    tokens: &'a [Token],
    current_token: Option<Token>,
    position: usize,
}

impl<'a> Parser<'a> {
    pub fn new(tokens: &'a [Token]) -> Self {
        Self { tokens, current_token: None, position: 0 }
    }

    fn parse_primary(&mut self) -> Result<ExpressionNode, String> {
        // let t = *self.current_token.ok_or_else(|| );
        if let Some(token) = self.current_token {
            self.advance();

            match token {
                Token::Number(val) => Ok(ExpressionNode::Number(val)),
                Token::Operator(op) => {
                    let expr =
                        self.parse_expression(self.get_precedence(op))?;
                    Ok(ExpressionNode::UnaryOperator {
                        operator: op,
                        expression: Box::new(expr),
                    })
                },
                Token::Paren('(') => {
                    let expr = self.parse_expression(0)?;
                    if let Some(Token::Paren(')')) = self.current_token {
                        self.advance(); // consume ')'
                        Ok(expr)
                    } else {
                        Err("Mismatched parentheses".to_string())
                    }
                },
                _ => Err("Unexpected token".to_string()),
            }
        } else {
            Err("Unexpected end of input".to_string())
        }
    }

    pub fn parse_expression(
        &mut self,
        precedence: u8,
    ) -> Result<ExpressionNode, String> {
        self.advance();

        let mut left = self.parse_primary()?;
        while let Some(Token::Operator(op)) = self.current_token {
            let token_precedence = self.get_precedence(op);
            if token_precedence <= precedence {
                break;
            }

            self.advance();

            let right = self.parse_expression(token_precedence)?;
            left = ExpressionNode::BinaryOperator {
                operator: op,
                left: Box::new(left),
                right: Box::new(right),
            };
        }

        Ok(left)
    }

    fn advance(&mut self) {
        if self.position < self.tokens.len() {
            self.current_token = self.tokens.get(self.position).copied();
            self.position += 1;
        } else {
            self.current_token = None;
        }
    }

    fn get_precedence(&self, op: char) -> u8 {
        match op {
            '+' | '-' => 1,
            '*' | '/' => 2,
            _ => 0,
        }
    }
}
