#[derive(Debug, Clone)]
pub enum Token {
    Keyword(String),
    Ident(String),
    Number(i64),
    String(String),
    Gt,
    Lt,
    Eq,
    Assign,
    Plus,
    Star,
    Minus,
    Slash,
}

#[derive(Debug, Clone, PartialEq)]
pub enum Operator {
    Gt,
    Lt,
    Eq,
}

#[derive(Debug, Clone)]
pub enum Expr {
    Number(i64),
    Ident(String),
    String(String),
    Call(String, Vec<Expr>),
    Binary(Box<Expr>, BinOp, Box<Expr>),
}

#[derive(Debug, Clone)]
pub enum BinOp {
    Add,
    Sub,
    Mul,
    Div,
}

#[derive(Debug, Clone)]
pub enum Node {
    FuncDef(String, Vec<String>, Vec<Node>), // name, params, body
    Return(Expr),
    If(Expr, Operator, Expr, Vec<Node>, Option<Vec<Node>>), // condition, operator, value, if_body, else_body
    Print(Expr),
    Set(String, Expr),
}
