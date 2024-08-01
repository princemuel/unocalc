use nom::branch::*;
use nom::bytes::complete::take;
use nom::combinator::{map, opt, verify};
use nom::error::{Error, ErrorKind};
use nom::multi::many0;
use nom::sequence::*;
use nom::Err;
use nom::*;

use std::result::Result::*;

pub mod ast;
use crate::lexer::token::*;
use crate::parser::ast::*;

macro_rules! tag_token (
    ($func_name:ident, $tag: expr) => (
        fn $func_name(tokens: Tokens) -> IResult<Tokens, Tokens> {
            verify(take(1usize), |t: &Tokens| t.token[0] == $tag)(tokens)
        }
    )
  );

fn parse_literal(input: Tokens) -> IResult<Tokens, Literal> {
    let (i1, t1) = take(1usize)(input)?;
    if t1.token.is_empty() {
        Err(Err::Error(Error::new(input, ErrorKind::Tag)))
    } else {
        match t1.token[0].clone() {
            Token::Int(val) => Ok((i1, Literal::Int(val))),
            Token::Float(val) => Ok((i1, Literal::Float(val))),
            _ => Err(Err::Error(Error::new(input, ErrorKind::Tag))),
        }
    }
}

tag_token!(plus_tag, Token::Operator('+'));
tag_token!(minus_tag, Token::Operator('-'));
// tag_token!(multiply, Token::Operator('*'));
// tag_token!(divide, Token::Operator('/'));
tag_token!(l_paren_tag, Token::Paren(true));
tag_token!(r_paren_tag, Token::Paren(false));
tag_token!(eof_tag, Token::EOF);

fn infix_op(t: &Token) -> (Precedence, Option<Infix>) {
    match *t {
        Token::Operator('+') => (Precedence::Sum, Some(Infix::Plus)),
        Token::Operator('-') => (Precedence::Sum, Some(Infix::Minus)),
        Token::Operator('*') => (Precedence::Product, Some(Infix::Multiply)),
        Token::Operator('/') => (Precedence::Product, Some(Infix::Divide)),
        Token::Paren(true) => (Precedence::Call, None),
        _ => (Precedence::Lowest, None),
    }
}

fn parse_program(input: Tokens) -> IResult<Tokens, Program> {
    terminated(many0(parse_expr_stmt), eof_tag)(input)
}

// fn parse_stmt(input: Tokens) -> IResult<Tokens, Stmt> {
//     alt(parse_expr_stmt)(input)
// }

fn parse_expr_stmt(input: Tokens) -> IResult<Tokens, Stmt> {
    map(terminated(parse_expr, opt(eof_tag)), Stmt::ExprStmt)(input)
}

fn parse_atom_expr(input: Tokens) -> IResult<Tokens, Expression> {
    alt((parse_lit_expr, parse_prefix_expr, parse_paren_expr))(input)
}

fn parse_paren_expr(input: Tokens) -> IResult<Tokens, Expression> {
    delimited(l_paren_tag, parse_expr, r_paren_tag)(input)
}

fn parse_lit_expr(input: Tokens) -> IResult<Tokens, Expression> {
    map(parse_literal, Expression::LitExpr)(input)
}

fn parse_prefix_expr(input: Tokens) -> IResult<Tokens, Expression> {
    let (i1, t1) = alt((plus_tag, minus_tag))(input)?;
    if t1.token.is_empty() {
        Err(Err::Error(error_position!(input, ErrorKind::Tag)))
    } else {
        let (i2, e) = parse_atom_expr(i1)?;
        match t1.token[0].clone() {
            Token::Operator('+') => {
                Ok((i2, Expression::PrefixExpr(Prefix::Plus, Box::new(e))))
            },
            Token::Operator('-') => {
                Ok((i2, Expression::PrefixExpr(Prefix::Minus, Box::new(e))))
            },
            _ => Err(Err::Error(error_position!(input, ErrorKind::Tag))),
        }
    }
}

fn parse_expr(input: Tokens) -> IResult<Tokens, Expression> {
    parse_pratt_expr(input, Precedence::Lowest)
}

fn parse_pratt_expr(
    input: Tokens,
    precedence: Precedence,
) -> IResult<Tokens, Expression> {
    let (i1, left) = parse_atom_expr(input)?;
    go_parse_pratt_expr(i1, precedence, left)
}

fn go_parse_pratt_expr(
    input: Tokens,
    precedence: Precedence,
    left: Expression,
) -> IResult<Tokens, Expression> {
    let (i1, t1) = take(1usize)(input)?;

    if t1.token.is_empty() {
        Ok((i1, left))
    } else {
        let preview = &t1.token[0];
        let p = infix_op(preview);
        match p {
            (ref peek_precedence, _) if precedence < *peek_precedence => {
                let (i2, left2) = parse_infix_expr(input, left)?;
                go_parse_pratt_expr(i2, precedence, left2)
            },
            _ => Ok((input, left)),
        }
    }
}

fn parse_infix_expr(
    input: Tokens,
    left: Expression,
) -> IResult<Tokens, Expression> {
    let (i1, t1) = take(1usize)(input)?;
    if t1.token.is_empty() {
        Err(Err::Error(error_position!(input, ErrorKind::Tag)))
    } else {
        let next = &t1.token[0];
        let (precedence, maybe_op) = infix_op(next);
        match maybe_op {
            None => Err(Err::Error(error_position!(input, ErrorKind::Tag))),
            Some(op) => {
                let (i2, right) = parse_pratt_expr(i1, precedence)?;
                Ok((
                    i2,
                    Expression::InfixExpr(op, Box::new(left), Box::new(right)),
                ))
            },
        }
    }
}

pub struct Parser;
impl Parser {
    pub fn parse(tokens: Tokens) -> IResult<Tokens, Program> {
        parse_program(tokens)
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::lexer::*;

    #[test]
    fn test_infix_expr_a() {
        let input = "10 + 20".as_bytes();

        let expected: Program = vec![Stmt::ExprStmt(Expression::InfixExpr(
            Infix::Plus,
            Box::new(Expression::LitExpr(Literal::Int(10))),
            Box::new(Expression::LitExpr(Literal::Int(20))),
        ))];

        let (_, r) = Lexer::tokenize(input).unwrap();
        let tokens = Tokens::new(&r);
        let (_, result) = Parser::parse(tokens).unwrap();
        assert_eq!(result, expected);
    }
}
