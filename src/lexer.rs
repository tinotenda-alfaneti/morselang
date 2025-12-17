use crate::morse::morse_to_keyword;
use crate::ast::Token;

pub fn lex(src: &str) -> Vec<Token> {
    let mut tokens = Vec::new();
    let mut i = 0;
    let chars: Vec<char> = src.chars().collect();

    while i < chars.len() {
        if chars[i].is_whitespace() {
            i += 1;
            continue;
        }

        // Check if this is a Morse keyword (starts with . or - and eventually has /)
        if (chars[i] == '.' || chars[i] == '-') && looks_like_morse(&chars, i) {
            let mut morse = String::new();
            while i < chars.len() && chars[i] != '/' {
                morse.push(chars[i]);
                i += 1;
            }
            if i < chars.len() && chars[i] == '/' {
                i += 1;
                let morse_trimmed = morse.trim();
                if let Some(keyword) = morse_to_keyword(morse_trimmed) {
                    tokens.push(Token::Keyword(keyword.to_string()));
                } else {
                    panic!("unknown Morse keyword: {}", morse_trimmed);
                }
            }
            continue;
        }

        if chars[i] == '"' {
            i += 1;
            let mut string_content = String::new();
            while i < chars.len() && chars[i] != '"' {
                string_content.push(chars[i]);
                i += 1;
            }
            i += 1;
            tokens.push(Token::String(string_content));
            continue;
        }

        if chars[i].is_numeric() || (chars[i] == '-' && i + 1 < chars.len() && chars[i + 1].is_numeric()) {
            let mut num_str = String::new();
            if chars[i] == '-' {
                num_str.push(chars[i]);
                i += 1;
            }
            while i < chars.len() && chars[i].is_numeric() {
                num_str.push(chars[i]);
                i += 1;
            }
            if let Ok(n) = num_str.parse::<i64>() {
                tokens.push(Token::Number(n));
            }
            continue;
        }

        match chars[i] {
            '>' => {
                tokens.push(Token::Gt);
                i += 1;
            }
            '<' => {
                tokens.push(Token::Lt);
                i += 1;
            }
            '=' => {
                if i + 1 < chars.len() && chars[i + 1] == '=' {
                    tokens.push(Token::Eq);
                    i += 2;
                } else {
                    tokens.push(Token::Assign);
                    i += 1;
                }
            }
            '+' => {
                tokens.push(Token::Plus);
                i += 1;
            }
            '-' => {
                tokens.push(Token::Minus);
                i += 1;
            }
            '*' => {
                tokens.push(Token::Star);
                i += 1;
            }
            '/' => {
                tokens.push(Token::Slash);
                i += 1;
            }

            'a'..='z' | 'A'..='Z' | '_' => {
                let mut ident = String::new();
                while i < chars.len() && (chars[i].is_alphanumeric() || chars[i] == '_') {
                    ident.push(chars[i]);
                    i += 1;
                }
                tokens.push(Token::Ident(ident));
            }
            _ => {
                i += 1;
            }
        }
    }

    tokens
}

fn looks_like_morse(chars: &[char], start: usize) -> bool {
    // Look ahead to see if there's a / indicating a Morse keyword
    for i in start..chars.len() {
        if chars[i] == '/' {
            return true;
        }
        // If we hit whitespace followed by something that's not morse-like, probably not morse
        if chars[i].is_whitespace() {
            continue;
        }
        // If we hit an alphanumeric or other operator, it's not morse
        if chars[i] != '.' && chars[i] != '-' && !chars[i].is_whitespace() {
            return false;
        }
    }
    false
}
