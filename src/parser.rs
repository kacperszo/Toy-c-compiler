use std::iter::Peekable;

use crate::ast::{Expression, Function, Statement};
use crate::lexer::Token;

fn parse_expression(
    tokens_iter: &mut Peekable<impl Iterator<Item = Token>>,
) -> Result<Expression, String> {
    match tokens_iter.next() {
        Some(Token::Number(val)) => Ok(Expression::Constant(val)),
        _ => Err("Expression value was required".to_string()),
    }
}
fn require_token(
    tokens_iter: &mut Peekable<impl Iterator<Item = Token>>,
    expected: Token,
) -> Result<(), String> {
    match tokens_iter.next() {
        Some(t) if t == expected => Ok(()),
        Some(t) => Err(format!("Expected {:?}, got {:?}", expected, t)),
        _ => Err(format!("Expected {:?}, got end of input", expected)),
    }
}
fn parse_statement(
    tokens_iter: &mut Peekable<impl Iterator<Item = Token>>,
) -> Result<Statement, String> {
    let _ = require_token(tokens_iter, Token::ReturnKeyword);
    let exp = parse_expression(tokens_iter)?;
    let _ = require_token(tokens_iter, Token::Semicolon);
    Ok(Statement::Return(exp))
}
pub fn parse(tokens: Vec<Token>) -> Result<Function, String> {
    let mut tokens_iter = tokens.into_iter().peekable();
    let _ = require_token(&mut tokens_iter, Token::IntKeyword);
    let function_name = match tokens_iter.next() {
        Some(Token::Identifier(_function_name)) => _function_name,
        _ => return Err("function name not found".to_string()),
    };
    let _ = require_token(&mut tokens_iter, Token::OpenParen);
    let _ = require_token(&mut tokens_iter, Token::CloseParen);
    let _ = require_token(&mut tokens_iter, Token::OpenBrace);
    let body = parse_statement(&mut tokens_iter)?;
    let _ = require_token(&mut tokens_iter, Token::CloseBrace);
    Ok(Function {
        name: function_name,
        body,
    })
}

