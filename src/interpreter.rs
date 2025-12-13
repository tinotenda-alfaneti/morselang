use crate::ast::Node;

pub fn run(ast: Vec<Node>) {
    for node in ast {
        match node {
            Node::If(var, num, body) => {
                if eval_condition(&var, num) {
                    execute_block(body);
                }
            }
            Node::Print(s) => {
                println!("{}", s);
            }
        }
    }
}

fn eval_condition(var: &str, num: i64) -> bool {
    if let Ok(v) = var.parse::<i64>() {
        return v > num;
    }
    false
}

fn execute_block(body: Vec<Node>) {
    for stmt in body {
        match stmt {
            Node::Print(s) => println!("{}", s),
            _ => {}
        }
    }
}
