# MORSELANG - The Morse Code Programming Language

**MORSELANG** is a playful esoteric programming language just weird enough to be delightful. It’s a language where your keywords go `beep-boop`, your code looks like an encrypted transmission, and your compiler tries its absolute best to pretend this is normal.

perfect for tinkering, experimenting, and showing off the fact that you created a language where typing `---` might actually do something important and nobody can stop you.

## Why?

Because my anime is on a cliffhanger and productivity had to go somewhere.

## Features

- **Interpreter mode**: Run `.mo` files directly
- **Compiler mode**: Transpile to Rust and produce native binaries
- **Morse-encoded keywords**: All language keywords are in Morse code with `/` separator
- **Real CLI tool**: Works like `python`, `node`, or `rustc`

## Installation

### Prerequisites

- Rust toolchain (cargo and rustc)
- For building binaries: ensure `rustc` is in your PATH

### Build and Install

```bash
# Clone or download the project
cd morselang

# On Linux/macOS:
chmod +x install.sh
./install.sh

# On Windows (PowerShell as admin):
cargo build --release
copy target\release\morse.exe C:\Windows\
```

After installation, the `morse` command will be available system-wide.

## Morse Keyword Table

Every keyword ends with `/` to avoid ambiguity.

| Keyword | Morse Code | With Separator | Meaning |
|---------|------------|----------------|---------|
| if | `.. ..-.` | `.. ..-./` | conditional |
| else | `. .-.. ... .` | `. .-.. ... ./` | else clause |
| while | `.-- .... .. .-.. .` | `.-- .... .. .-.. ./` | loop |
| for | `..-. --- .-.` | `..-. --- .-./` | loop |
| do | `-.. ---` | `-.. ---/` | block opener |
| end | `.-. -..` | `.-. -../` | terminator |
| print | `.--. .-. .. -. -` | `.--. .-. .. -. -/` | output |
| return | `.-. . - ..- .-. -.` | `.-. . - ..- .-. -./` | return |
| func | `..-. ..- -. -.-.` | `..-. ..- -. -.-./` | function |
| set | `... . -` | `... . -/` | assignment |
| true | `- .-. ..- .` | `- .-. ..- ./` | boolean |
| false | `..-. .- .-.. ... .` | `..-. .- .-.. ... ./` | boolean |
| and | `.- -. -..` | `.- -. -../` | logic |
| or | `--- .-.` | `--- .-./` | logic |
| not | `-. --- -` | `-. --- -/` | logic |

## Example Code

### example.mo

```morse
.. ..-./ 10 > 5
    .--. .-. .. -. -/ "Hello Morse World!"
.-. -../
```

This translates to:

```
if 10 > 5
    print "Hello Morse World!"
end
```

## Usage

### Run (Interpreted)

```bash
morse run example.mo
```

Output:
```
Hello Morse World!
```

### Build (Native Binary)

```bash
morse build example.mo -o hello
./hello
```

Output:
```
Hello Morse World!
```

The build command:
1. Transpiles `.mo` source to Rust code
2. Compiles with `rustc` to produce a native executable
3. Cleans up intermediate files

## Language Design

- **Keywords**: Morse code with `/` separator
- **Variables**: Normal text (e.g., `x`, `count`, `name`)
- **Numbers**: Normal integers (e.g., `10`, `42`, `-5`)
- **Operators**: Normal symbols (`>`, `<`, `==`, `=`, `+`, `-`, `*`, `/`)
- **Strings**: Double-quoted (e.g., `"Hello"`)


## Project Structure

```
morselang/
├── src/
│   ├── main.rs         # CLI entry point
│   ├── lexer.rs        # Tokenizer
│   ├── parser.rs       # AST builder
│   ├── ast.rs          # AST definitions
│   ├── interpreter.rs  # Runtime execution
│   ├── transpiler.rs   # Code generation
│   └── morse.rs        # Morse→keyword table
├── Cargo.toml          # Rust dependencies
├── example.mo         # Sample program
├── install.sh          # Installation script
└── README.md           # This file
```


---

**Built with Rust | Powered by Morse Code**
