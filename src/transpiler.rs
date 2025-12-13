use crate::ast::Node;

pub fn to_rust(ast: Vec<Node>) -> String {
    let mut out = String::new();

    out.push_str("fn main() {\n");

    for node in ast {
        match node {
            Node::Print(s) => {
                out.push_str(&format!("    println!(\"{}\");\n", s));
            }
            Node::If(var, num, body) => {
                out.push_str(&format!("    if {} > {} {{\n", var, num));
                for stmt in body {
                    if let Node::Print(s) = stmt {
                        out.push_str(&format!("        println!(\"{}\");\n", s));
                    }
                }
                out.push_str("    }\n");
            }
        }
    }

    out.push_str("}\n");
    out
}
