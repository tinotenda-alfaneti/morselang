use crate::ast::{Token, Node, Expr, BinOp, Operator};

pub fn parse(tokens: Vec<Token>) -> Vec<Node> {
    let mut i = 0;
    let mut ast = Vec::new();

    while i < tokens.len() {
        match &tokens[i] {
            Token::Keyword(k) if k == "IF" => {
                i += 1;
                let (left_expr, new_i) = parse_expr(&tokens, i);
                i = new_i;

                let operator = match &tokens[i] {
                    Token::Gt => Operator::Gt,
                    Token::Lt => Operator::Lt,
                    Token::Eq => Operator::Eq,
                    _ => panic!("expected comparison operator in IF condition"),
                };
                i += 1;

                let (right_expr, new_i) = parse_expr(&tokens, i);
                i = new_i;

                let mut if_body = Vec::new();
                let mut else_body = None;
                
                while i < tokens.len() {
                    if let Token::Keyword(k) = &tokens[i] {
                        if k == "END" {
                            i += 1;
                            break;
                        }
                        if k == "ELSE" {
                            i += 1;
                            let mut else_stmts = Vec::new();
                            while i < tokens.len() {
                                if let Token::Keyword(k) = &tokens[i] {
                                    if k == "END" {
                                        i += 1;
                                        break;
                                    }
                                }
                                let (stmt, new_i) = parse_statement(&tokens, i);
                                else_stmts.push(stmt);
                                i = new_i;
                            }
                            else_body = Some(else_stmts);
                            break;
                        }
                    }
                    let (stmt, new_i) = parse_statement(&tokens, i);
                    if_body.push(stmt);
                    i = new_i;
                }

                ast.push(Node::If(left_expr, operator, right_expr, if_body, else_body));
            }
            Token::Keyword(k) if k == "PRINT" => {
                i += 1;
                let (expr, new_i) = parse_expr(&tokens, i);
                ast.push(Node::Print(expr));
                i = new_i;
            }
            Token::Keyword(k) if k == "SET" => {
                i += 1;
                let var_name = match &tokens[i] {
                    Token::Ident(name) => name.clone(),
                    _ => panic!("expected identifier after SET"),
                };
                i += 1;

                if !matches!(&tokens[i], Token::Assign) {
                    panic!("expected = after variable name in SET");
                }
                i += 1;

                let (expr, new_i) = parse_expr(&tokens, i);
                ast.push(Node::Set(var_name, expr));
                i = new_i;
            }
            _ => i += 1
        }
    }
    ast
}

fn parse_statement(tokens: &[Token], mut i: usize) -> (Node, usize) {
    match &tokens[i] {
        Token::Keyword(k) if k == "PRINT" => {
            i += 1;
            let (expr, new_i) = parse_expr(tokens, i);
            (Node::Print(expr), new_i)
        }
        Token::Keyword(k) if k == "SET" => {
            i += 1;
            let var_name = match &tokens[i] {
                Token::Ident(name) => name.clone(),
                _ => panic!("expected identifier after SET"),
            };
            i += 1;

            if !matches!(&tokens[i], Token::Assign) {
                panic!("expected = after variable name in SET");
            }
            i += 1;

            let (expr, new_i) = parse_expr(tokens, i);
            (Node::Set(var_name, expr), new_i)
        }
        _ => panic!("unexpected token in statement"),
    }
}

fn parse_expr(tokens: &[Token], mut i: usize) -> (Expr, usize) {
    let mut left = match &tokens[i] {
        Token::Number(n) => {
            i += 1;
            Expr::Number(*n)
        }
        Token::Ident(name) => {
            i += 1;
            Expr::Ident(name.clone())
        }
        Token::String(s) => {
            i += 1;
            Expr::String(s.clone())
        }
        _ => panic!("expected number, identifier, or string in expression"),
    };

    // Check for binary operators
    while i < tokens.len() {
        let op = match &tokens[i] {
            Token::Plus => BinOp::Add,
            Token::Minus => BinOp::Sub,
            Token::Star => BinOp::Mul,
            Token::Slash => BinOp::Div,
            _ => break,
        };
        i += 1;

        let right = match &tokens[i] {
            Token::Number(n) => {
                i += 1;
                Expr::Number(*n)
            }
            Token::Ident(name) => {
                i += 1;
                Expr::Ident(name.clone())
            }
            _ => panic!("expected number or identifier after operator"),
        };

        left = Expr::Binary(Box::new(left), op, Box::new(right));
    }

    (left, i)
}
