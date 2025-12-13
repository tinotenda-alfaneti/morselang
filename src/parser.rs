use crate::ast::{Token, Node};

pub fn parse(tokens: Vec<Token>) -> Vec<Node> {
    let mut i = 0;
    let mut ast = Vec::new();

    while i < tokens.len() {
        match &tokens[i] {
            Token::Keyword(k) if k == "IF" => {
                let left_val = match &tokens[i+1] {
                    Token::Ident(v) => v.clone(),
                    Token::Number(n) => n.to_string(),
                    _ => panic!("expected identifier or number after IF"),
                };
                
                let _op = &tokens[i+2];
                
                let number = match &tokens[i+3] {
                    Token::Number(n) => *n,
                    _ => panic!("expected number in IF condition"),
                };

                i += 4;

                let mut body = Vec::new();
                while i < tokens.len() {
                    if let Token::Keyword(k) = &tokens[i] {
                        if k == "END" { 
                            break; 
                        }
                        if k == "PRINT" {
                            match &tokens[i+1] {
                                Token::String(s) => {
                                    body.push(Node::Print(s.clone()));
                                    i += 2;
                                },
                                _ => panic!("expected string after PRINT"),
                            }
                        } else {
                            i += 1;
                        }
                    } else {
                        i += 1;
                    }
                }

                i += 1;
                ast.push(Node::If(left_val, number, body));
            }
            _ => i += 1
        }
    }
    ast
}
