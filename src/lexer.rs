use std::{iter::Peekable, str::Chars};

#[derive(Debug, PartialEq)]
pub enum Token {
    IntKeyword,
    Identifier(String),
    OpenBrace,
    CloseBrace,
    OpenParen,
    CloseParen,
    ReturnKeyword,
    Number(i32),
    Semicolon,
    Plus,
    Minus,
    Asterisk,
    Slash,
}

fn scan_while_numeric(c: char, chars: &mut Peekable<Chars>) -> String {
    let mut buf = String::from(c);
    while chars.peek().map_or(false, |&c| c.is_ascii_digit()) {
        buf.push(chars.next().unwrap());
    }
    buf
}
fn scan_while_identifier(c: char, chars: &mut Peekable<Chars>) -> String {
    let mut buf = String::from(c);
    while chars
        .peek()
        .map_or(false, |&c| c.is_ascii_alphanumeric() || c == '_')
    {
        buf.push(chars.next().unwrap());
    }
    buf
}

pub fn tokenize(source: &str) -> Result<Vec<Token>, String> {
    let mut chars = source.chars().peekable();
    let mut tokens = Vec::new();
    while let Some(c) = chars.next() {
        let token = match c {
            ' ' | '\n' | '\t' => continue,
            ';' => Token::Semicolon,
            '(' => Token::OpenParen,
            ')' => Token::CloseParen,
            '{' => Token::OpenBrace,
            '}' => Token::CloseBrace,
            '+' => Token::Plus,
            '*' => Token::Asterisk,
            '-' => Token::Minus,
            '/' => Token::Slash,
            '0'..='9' => match scan_while_numeric(c, &mut chars).parse::<i32>() {
                Ok(val) => Token::Number(val),
                Err(error) => {
                    return Err(format!("Faild at tokenizing numeric string {}", error));
                }
            },
            'a'..='z' | 'A'..='Z' | '_' => {
                let buf = scan_while_identifier(c, &mut chars);
                match buf.as_str() {
                    "int" => Token::IntKeyword,
                    "return" => Token::ReturnKeyword,
                    _ => Token::Identifier(buf),
                }
            }
            _ => {
                return Err(format!("Failed at tokenzing character {}", c));
            }
        };
        tokens.push(token);
    }
    Ok(tokens)
}
