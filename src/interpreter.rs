use crate::ast::{Node, Expr, BinOp, Operator};
use std::collections::HashMap;

pub fn run(ast: Vec<Node>) {
    let mut globals: HashMap<String, i64> = HashMap::new();
    let mut functions: HashMap<String, (Vec<String>, Vec<Node>)> = HashMap::new();

    // collect functions and top-level nodes
    let mut main_nodes = Vec::new();
    for node in ast {
        match node {
            Node::FuncDef(name, params, body) => {
                functions.insert(name, (params, body));
            }
            other => main_nodes.push(other),
        }
    }

    // execute main
    execute_block(&main_nodes, &mut globals, &functions);
}

fn execute_block(nodes: &[Node], vars: &mut HashMap<String, i64>, functions: &HashMap<String, (Vec<String>, Vec<Node>)>) -> Option<i64> {
    for node in nodes {
        match node {
            Node::If(left, op, right, if_body, else_body) => {
                let left_val = eval_expr(left, vars, functions);
                let right_val = eval_expr(right, vars, functions);

                let condition = match op {
                    Operator::Gt => left_val > right_val,
                    Operator::Lt => left_val < right_val,
                    Operator::Eq => left_val == right_val,
                };

                if condition {
                    if let Some(ret) = execute_block(if_body, vars, functions) {
                        return Some(ret);
                    }
                } else if let Some(else_stmts) = else_body {
                    if let Some(ret) = execute_block(else_stmts, vars, functions) {
                        return Some(ret);
                    }
                }
            }
            Node::Print(expr) => {
                match expr {
                    Expr::String(s) => println!("{}", s),
                    _ => {
                        let val = eval_expr(expr, vars, functions);
                        println!("{}", val);
                    }
                }
            }
            Node::Set(var_name, expr) => {
                let value = eval_expr(expr, vars, functions);
                vars.insert(var_name.clone(), value);
            }
            Node::Return(expr) => {
                let value = eval_expr(expr, vars, functions);
                return Some(value);
            }
            Node::FuncDef(_, _, _) => {
                // already handled at top-level; ignore nested defs for now
            }
        }
    }
    None
}

fn eval_expr(expr: &Expr, vars: &HashMap<String, i64>, functions: &HashMap<String, (Vec<String>, Vec<Node>)>) -> i64 {
    match expr {
        Expr::Number(n) => *n,
        Expr::Ident(name) => {
            *vars.get(name).unwrap_or_else(|| panic!("undefined variable: {}", name))
        }
        Expr::String(_) => panic!("cannot evaluate string as number"),
        Expr::Call(name, args) => {
            let func = functions.get(name).unwrap_or_else(|| panic!("undefined function: {}", name));
            let (params, body) = func;
            if params.len() != args.len() {
                panic!("function {} expected {} args, got {}", name, params.len(), args.len());
            }
            let mut local_vars: HashMap<String, i64> = HashMap::new();
            for (p, a) in params.iter().zip(args.iter()) {
                let val = eval_expr(a, vars, functions);
                local_vars.insert(p.clone(), val);
            }
            if let Some(ret) = execute_block(body, &mut local_vars, functions) {
                ret
            } else {
                panic!("function {} did not return a value", name);
            }
        }
        Expr::Binary(left, op, right) => {
            let left_val = eval_expr(left, vars, functions);
            let right_val = eval_expr(right, vars, functions);
            match op {
                BinOp::Add => left_val + right_val,
                BinOp::Sub => left_val - right_val,
                BinOp::Mul => left_val * right_val,
                BinOp::Div => left_val / right_val,
            }
        }
    }
}
