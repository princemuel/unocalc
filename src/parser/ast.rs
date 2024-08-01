pub type Program = Vec<Stmt>;

#[derive(Debug, Clone, PartialEq)]
pub enum Stmt {
    ExprStmt(Expression),
}

#[derive(Debug, Clone, PartialEq)]
pub enum Expression {
    LitExpr(Literal),
    PrefixExpr(Prefix, Box<Expression>),
    InfixExpr(Infix, Box<Expression>, Box<Expression>),
}

#[derive(Debug, Clone, PartialEq)]
pub enum Literal {
    Int(i64),
    Float(f64),
}

#[derive(Debug, Clone, PartialEq)]
pub enum Prefix {
    Plus,
    Minus,
}

#[derive(Debug, Clone, PartialEq)]
pub enum Infix {
    Plus,
    Minus,
    Multiply,
    Divide,
}
#[derive(Debug, Clone, PartialEq, PartialOrd)]
pub enum Precedence {
    Lowest,
    Sum,
    Product,
    Call,
}
