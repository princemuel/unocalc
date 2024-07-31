use nom::branch::alt;
use nom::bytes::complete::{tag, take};
use nom::character::complete::{digit1, multispace0, one_of};
use nom::combinator::{map, map_res, opt, recognize};
use nom::multi::many0;
use nom::sequence::{delimited, pair, tuple};
use nom::*;

use std::str;
use std::str::FromStr;

pub mod token;
use crate::lexer::token::*;

macro_rules! syntax {
    ($func_name: ident, $tag_string: literal, $output_token: expr) => {
        fn $func_name(s: &[u8]) -> IResult<&[u8], Token> {
            map(tag($tag_string), |_| $output_token)(s)
        }
    };
}

// Operator Tokens
syntax! {plus_op, "+", Token::Operator('+')}
syntax! {minus_op, "-", Token::Operator('-')}
syntax! {multiply_op, "*", Token::Operator('*')}
syntax! {divide_op, "/", Token::Operator('/')}
pub fn lex_operator(input: &[u8]) -> IResult<&[u8], Token> {
    alt((plus_op, minus_op, multiply_op, divide_op))(input)
}

// Punctuation Tokens
syntax! {l_paren, "(", Token::Paren(true)}
syntax! {r_paren, ")", Token::Paren(false)}
pub fn lex_punctuation(input: &[u8]) -> IResult<&[u8], Token> {
    alt((l_paren, r_paren))(input)
}

// Number Tokens
fn lex_number(input: &[u8]) -> IResult<&[u8], Token> {
    let parser = recognize(tuple((
        opt(one_of("+-")),
        digit1,
        opt(pair(tag("."), digit1)),
        opt(pair(one_of("eE"), tuple((opt(one_of("+-")), digit1)))),
    )));

    map(
        map_res(map_res(parser, |value| str::from_utf8(value)), |value| {
            f64::from_str(value)
        }),
        Token::Number,
    )(input)
}

// Illegal Tokens
fn lex_illegal(input: &[u8]) -> IResult<&[u8], Token> {
    map(take(1usize), |_| Token::Illegal)(input)
}

fn lex_token(input: &[u8]) -> IResult<&[u8], Token> {
    alt((lex_operator, lex_punctuation, lex_number, lex_illegal))(input)
}

fn lex_tokens(input: &[u8]) -> IResult<&[u8], Vec<Token>> {
    many0(delimited(multispace0, lex_token, multispace0))(input)
}

pub struct Lexer;
impl Lexer {
    pub fn tokenize(bytes: &[u8]) -> IResult<&[u8], Vec<Token>> {
        lex_tokens(bytes).map(|(slice, result)| {
            (slice, [&result[..], &vec![Token::EOF][..]].concat())
        })
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_lexer_1() {
        let input = &b"-+*/()"[..];
        let (_, result) = Lexer::tokenize(input).unwrap();

        let expected = vec![
            Token::Operator('-'),
            Token::Operator('+'),
            Token::Operator('*'),
            Token::Operator('/'),
            Token::Paren(true),
            Token::Paren(false),
            Token::EOF,
        ];
        assert_eq!(result, expected);
    }

    #[test]
    fn test_lexer_2() {
        let input = &b"- + * / ( ) "[..];
        let (_, result) = Lexer::tokenize(input).unwrap();

        let expected = vec![
            Token::Operator('-'),
            Token::Operator('+'),
            Token::Operator('*'),
            Token::Operator('/'),
            Token::Paren(true),
            Token::Paren(false),
            Token::EOF,
        ];
        assert_eq!(result, expected);
    }

    #[test]
    fn test_lexer_3() {
        let input = &b"  -5   +  3.12 *  ( 2 - 1 ) "[..];
        let (_, result) = Lexer::tokenize(input).unwrap();
        let expected = vec![
            Token::Operator('-'),
            Token::Number(5.0),
            Token::Operator('+'),
            Token::Number(3.12),
            Token::Operator('*'),
            Token::Paren(true),
            Token::Number(2.0),
            Token::Operator('-'),
            Token::Number(1.0),
            Token::Paren(false),
            Token::EOF,
        ];
        assert_eq!(result, expected);
    }
}
