use clap::{Parser, Subcommand};
use std::fs;
use std::process::Command;

mod lexer;
mod parser;
mod ast;
mod interpreter;
mod morse;
mod transpiler;

#[derive(Parser)]
#[command(name="morse", version="1.0", about="The Morse Code Programming Language")]
struct Cli {
    #[command(subcommand)]
    command: Commands,
}

#[derive(Subcommand)]
enum Commands {

    Run { file: String },

    Build { 
        file: String, 
        #[arg(short, long)] 
        out: String 
    },

    Docs {},
}

fn main() {
    let cli = Cli::parse();

    match cli.command {
        Commands::Run { file } => {
            let src = fs::read_to_string(&file).expect("Failed to read source");
            let tokens = lexer::lex(&src);
            let ast = parser::parse(tokens);
            interpreter::run(ast);
        }
        Commands::Build { file, out } => {
            let src = fs::read_to_string(&file).expect("Failed to read source");
            let tokens = lexer::lex(&src);
            let ast = parser::parse(tokens);
            let rust_code = transpiler::to_rust(ast);

            fs::write("out.rs", &rust_code).expect("Failed to write intermediate Rust");

            let status = Command::new("rustc")
                .args(["out.rs", "-O", "-o", &out])
                .status()
                .expect("Failed to invoke rustc. Make sure rustc is installed and in PATH.");

            if status.success() {
                println!("Built successfully → {}", out);
                fs::remove_file("out.rs").ok();
            } else {
                println!("Build failed");
                println!("Generated Rust code saved in out.rs for debugging");
            }
        }

        Commands::Docs {} => {
            let filepath = "docs/MORSE_REFERENCE.md";
            let src = fs::read_to_string(&filepath).expect("Failed to read source");
            println!("{}", src);
        }
    }
}
