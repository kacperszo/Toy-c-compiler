use std::{fmt::format, iter::Peekable, str::Chars};

#[derive(Debug)]
pub enum Token {
    IntKeyworld,
    Identifier(String),
    OpenBrace,
    CloseBrace,
    OpenBraket,
    CloseBraket,
    ReturnKeyword,
    Number(i32),
    Semicoln,
}

fn is_token_ending(chars: &mut Peekable<Chars<'_>>) -> bool {
    chars.peek() == Some(&'(')
        || chars.peek() == Some(&')')
        || chars.peek() == Some(&'{')
        || chars.peek() == Some(&';')
        || chars.peek() == Some(&'}')
        || chars.peek() == Some(&' ')
        || chars.peek() == Some(&'\n')
        || chars.peek() == None
}

pub fn tokenize(source: &str) -> Vec<Token> {
    let mut chars = source.chars().peekable();
    let mut current_scanned = String::from("");
    let mut tokens = Vec::new();
    let mut is_numeric = false;
    while let Some(c) = chars.next() {
        let one_character_tokens = match c {
            ';' => Some(Token::Semicoln),
            '(' => Some(Token::OpenBraket),
            ')' => Some(Token::CloseBraket),
            '{' => Some(Token::OpenBrace),
            '}' => Some(Token::CloseBrace),
            _ => None,
        };
        if let Some(token) = one_character_tokens {
            tokens.push(token);
            current_scanned = String::from("");
            continue;
        }
        if c == ' ' || c == '\n' {
            current_scanned = String::from("");
            continue;
        }

        if c.is_numeric() && (is_numeric || current_scanned.len() == 0) {
            current_scanned.push(c);
            is_numeric = true;
            if is_token_ending(&mut chars) {
                tokens.push(Token::Number(
                    current_scanned.parse::<i32>().expect("NOT a number!"),
                ));
                current_scanned = String::from("");
                is_numeric = false;
            }
            continue;
        }
        if c.is_ascii_alphanumeric() {
            current_scanned.push(c);
            if is_token_ending(&mut chars) {
                tokens.push(match current_scanned.as_str() {
                    "int" => Token::IntKeyworld,
                    "return" => Token::ReturnKeyword,
                    _ => Token::Identifier(current_scanned),
                });
                current_scanned = String::from("");
            }
            continue;
        }
        println!("{}", c);
        println!("{:?}", tokens);
        panic!("Unkown token");
    }
    return tokens;
}
