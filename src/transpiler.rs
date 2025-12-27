use crate::ast::{Node, Expr, BinOp, Operator};

pub fn to_rust(ast: Vec<Node>) -> String {
    let mut out = String::new();

    // Separate function definitions from main nodes
    let mut funcs = Vec::new();
    let mut main_nodes = Vec::new();
    for node in &ast {
        match node {
            Node::FuncDef(_, _, _) => funcs.push(node.clone()),
            other => main_nodes.push(other.clone()),
        }
    }

    // Emit functions first
    for node in funcs {
        if let Node::FuncDef(name, params, body) = node {
            out.push_str(&format!("fn {}({}) -> i64 {{\n", name, params.iter().map(|p| format!("{}: i64", p)).collect::<Vec<_>>().join(", ")));
            generate_statements(&body, &mut out, 1);
            out.push_str("    panic!(\"function did not return\");\n");
            out.push_str("}\n\n");
        }
    }

    out.push_str("fn main() {\n");
    
    // Collect all variables used in the program (only from main scope)
    let mut vars = std::collections::HashSet::new();
    collect_vars(&main_nodes, &mut vars);
    
    // Declare all variables as mutable i64
    for var in &vars {
        out.push_str(&format!("    let mut {}: i64 = 0;\n", var));
    }
    if !vars.is_empty() {
        out.push_str("\n");
    }

    generate_statements(&main_nodes, &mut out, 1);

    out.push_str("}\n");
    out
}

fn collect_vars(nodes: &[Node], vars: &mut std::collections::HashSet<String>) {
    for node in nodes {
        match node {
            Node::Set(name, expr) => {
                vars.insert(name.clone());
                collect_vars_from_expr(expr, vars);
            }
            Node::If(left, _, right, if_body, else_body) => {
                collect_vars_from_expr(left, vars);
                collect_vars_from_expr(right, vars);
                collect_vars(if_body, vars);
                if let Some(else_stmts) = else_body {
                    collect_vars(else_stmts, vars);
                }
            }
            Node::Print(expr) => {
                collect_vars_from_expr(expr, vars);
            }
            Node::FuncDef(_, _, _) => {
                // skip: function-local variables are not globals
            }
            Node::Return(_) => {}
        }
    }
}

fn collect_vars_from_expr(expr: &Expr, vars: &mut std::collections::HashSet<String>) {
    match expr {
        Expr::Ident(name) => {
            vars.insert(name.clone());
        }
        Expr::Call(_, args) => {
            for a in args {
                collect_vars_from_expr(a, vars);
            }
        }
        Expr::Binary(left, _, right) => {
            collect_vars_from_expr(left, vars);
            collect_vars_from_expr(right, vars);
        }
        _ => {}
    }
}

fn generate_statements(nodes: &[Node], out: &mut String, indent: usize) {
    let indent_str = "    ".repeat(indent);
    
    for node in nodes {
        match node {
            Node::Print(expr) => {
                match expr {
                    Expr::String(s) => {
                        out.push_str(&format!("{}println!(\"{}\");\n", indent_str, s));
                    }
                    _ => {
                        let expr_str = expr_to_rust(expr);
                        out.push_str(&format!("{}println!(\"{{}}\", {});\n", indent_str, expr_str));
                    }
                }
            }
            Node::If(left, op, right, if_body, else_body) => {
                let left_str = expr_to_rust(left);
                let right_str = expr_to_rust(right);
                let op_str = match op {
                    Operator::Gt => ">",
                    Operator::Lt => "<",
                    Operator::Eq => "==",
                };
                
                out.push_str(&format!("{}if {} {} {} {{\n", indent_str, left_str, op_str, right_str));
                generate_statements(if_body, out, indent + 1);
                
                if let Some(else_stmts) = else_body {
                    out.push_str(&format!("{}}} else {{\n", indent_str));
                    generate_statements(else_stmts, out, indent + 1);
                }
                
                out.push_str(&format!("{}}}\n", indent_str));
            }
            Node::Set(var_name, expr) => {
                let expr_str = expr_to_rust(expr);
                out.push_str(&format!("{}{} = {};\n", indent_str, var_name, expr_str));
            }
            Node::Return(expr) => {
                let expr_str = expr_to_rust(expr);
                out.push_str(&format!("{}return {};\n", indent_str, expr_str));
            }
            Node::FuncDef(_, _, _) => {
                // already emitted above
            }
        }
    }
}

fn expr_to_rust(expr: &Expr) -> String {
    match expr {
        Expr::Number(n) => n.to_string(),
        Expr::Ident(name) => name.clone(),
        Expr::String(s) => format!("\"{}\"", s),
        Expr::Call(name, args) => {
            let args_str = args.iter().map(|a| expr_to_rust(a)).collect::<Vec<_>>().join(", ");
            format!("{}({})", name, args_str)
        }
        Expr::Binary(left, op, right) => {
            let left_str = expr_to_rust(left);
            let right_str = expr_to_rust(right);
            let op_str = match op {
                BinOp::Add => "+",
                BinOp::Sub => "-",
                BinOp::Mul => "*",
                BinOp::Div => "/",
            };
            format!("({} {} {})", left_str, op_str, right_str)
        }
    }
}
