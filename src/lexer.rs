use std::iter::Peekable;
use std::str::Chars;

use crate::schema::Token;

#[derive(Debug, Clone)]
pub struct Lexer<'a> {
    input: Peekable<Chars<'a>>,
}

impl<'a> Lexer<'a> {
    pub fn new(input: &'a str) -> Self {
        Self { input: input.chars().peekable() }
    }

    pub fn tokenize(&mut self) -> Result<Vec<Token>, String> {
        let mut tokens = Vec::new();
        while let Some(token) = self.next_token() {
            tokens.push(token?);
        }
        Ok(tokens)
    }

    fn next_token(&mut self) -> Option<Result<Token, String>> {
        while let Some(&char) = self.input.peek() {
            match char {
                '0'..='9' | '.' => return Some(self.get_number_token()),
                '+' | '-' | '*' | '/' => {
                    self.input.next();
                    return Some(Ok(Token::Operator(char)));
                },
                '(' | ')' => {
                    self.input.next();
                    return Some(Ok(Token::Paren(char)));
                },
                ' ' => {
                    self.input.next();
                },
                _ => return Some(Err(format!("Unexpected character: {char}"))),
            }
        }
        None
    }

    fn get_number_token(&mut self) -> Result<Token, String> {
        let number = self
            .input
            .by_ref() // take ownership of self.input for the duration of the loop
            .take_while(|&char| char.is_numeric() || char == '.')
            .collect::<String>();

        number
            .parse::<f64>()
            .map(Token::Number)
            .map_err(|_| format!("Invalid number: {number}"))
    }
}
