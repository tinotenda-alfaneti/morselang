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
}

#[derive(Debug, Clone)]
pub enum Node {
    If(String, i64, Vec<Node>),
    Print(String),
}
