pub mod object;
use crate::evaluator::object::*;
use crate::parser::ast::*;

#[derive(Default)]
pub struct Evaluator;

impl Evaluator {
    pub fn new() -> Self {
        Self {}
    }

    pub fn eval_program(&mut self, mut prog: Program) -> Object {
        self.eval_statement(prog.remove(0))
    }

    pub fn eval_statement(&mut self, stmt: Stmt) -> Object {
        match stmt {
            Stmt::ExprStmt(expr) => self.eval_expr(expr),
        }
    }

    pub fn eval_expr(&mut self, expression: Expression) -> Object {
        use Expression::*;
        match expression {
            LitExpr(l) => self.eval_literal(l),
            PrefixExpr(prefix, expr) => self.eval_prefix(&prefix, *expr),
            InfixExpr(infix, a, b) => self.eval_infix(&infix, *a, *b),
        }
    }

    pub fn eval_literal(&mut self, literal: Literal) -> Object {
        match literal {
            Literal::Number(value) => Object::Number(value),
        }
    }

    pub fn eval_prefix(&mut self, prefix: &Prefix, expr: Expression) -> Object {
        let object = self.eval_expr(expr);
        match *prefix {
            Prefix::Plus => match self.oti(object) {
                Ok(value) => Object::Number(value),
                Err(exception) => exception,
            },
            Prefix::Minus => match self.oti(object) {
                Ok(value) => Object::Number(-value),
                Err(exception) => exception,
            },
        }
    }

    pub fn eval_infix(
        &mut self,
        infix: &Infix,
        a: Expression,
        b: Expression,
    ) -> Object {
        let expr_a = self.eval_expr(a);
        let expr_b = self.eval_expr(b);

        match *infix {
            Infix::Plus => {
                let x = self.oti(expr_a);
                let y = self.oti(expr_b);
                match (x, y) {
                    (Ok(x), Ok(y)) => Object::Number(x + y),
                    (Err(exception), _) | (_, Err(exception)) => exception,
                }
            },
            Infix::Minus => {
                let x = self.oti(expr_a);
                let y = self.oti(expr_b);
                match (x, y) {
                    (Ok(x), Ok(y)) => Object::Number(x - y),
                    (Err(exception), _) | (_, Err(exception)) => exception,
                }
            },
            Infix::Multiply => {
                let x = self.oti(expr_a);
                let y = self.oti(expr_b);
                match (x, y) {
                    (Ok(x), Ok(y)) => Object::Number(x * y),
                    (Err(exception), _) | (_, Err(exception)) => exception,
                }
            },
            Infix::Divide => {
                let x = self.oti(expr_a);
                let y = self.oti(expr_b);
                match (x, y) {
                    (Ok(x), Ok(y)) => {
                        Object::Number(if y == 0.0 { y } else { x / y })
                    },
                    (Err(exception), _) | (_, Err(exception)) => exception,
                }
            },
        }
    }

    pub fn oti(&mut self, object: Object) -> Result<f64, Object> {
        match object {
            Object::Number(value) => Ok(value),
            Object::Error(s) => Err(Object::Error(s)),
        }
    }
}
