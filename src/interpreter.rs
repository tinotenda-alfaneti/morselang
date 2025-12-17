use crate::ast::{Node, Expr, BinOp, Operator};
use std::collections::HashMap;

pub fn run(ast: Vec<Node>) {
    let mut vars: HashMap<String, i64> = HashMap::new();
    execute_block(&ast, &mut vars);
}

fn execute_block(nodes: &[Node], vars: &mut HashMap<String, i64>) {
    for node in nodes {
        match node {
            Node::If(left, op, right, if_body, else_body) => {
                let left_val = eval_expr(left, vars);
                let right_val = eval_expr(right, vars);
                
                let condition = match op {
                    Operator::Gt => left_val > right_val,
                    Operator::Lt => left_val < right_val,
                    Operator::Eq => left_val == right_val,
                };

                if condition {
                    execute_block(if_body, vars);
                } else if let Some(else_stmts) = else_body {
                    execute_block(else_stmts, vars);
                }
            }
            Node::Print(expr) => {
                match expr {
                    Expr::String(s) => println!("{}", s),
                    _ => {
                        let val = eval_expr(expr, vars);
                        println!("{}", val);
                    }
                }
            }
            Node::Set(var_name, expr) => {
                let value = eval_expr(expr, vars);
                vars.insert(var_name.clone(), value);
            }
        }
    }
}

fn eval_expr(expr: &Expr, vars: &HashMap<String, i64>) -> i64 {
    match expr {
        Expr::Number(n) => *n,
        Expr::Ident(name) => {
            *vars.get(name).unwrap_or_else(|| panic!("undefined variable: {}", name))
        }
        Expr::String(_) => panic!("cannot evaluate string as number"),
        Expr::Binary(left, op, right) => {
            let left_val = eval_expr(left, vars);
            let right_val = eval_expr(right, vars);
            match op {
                BinOp::Add => left_val + right_val,
                BinOp::Sub => left_val - right_val,
                BinOp::Mul => left_val * right_val,
                BinOp::Div => left_val / right_val,
            }
        }
    }
}
