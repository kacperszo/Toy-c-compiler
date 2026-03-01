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
fn parse_statement(
    tokens_iter: &mut Peekable<impl Iterator<Item = Token>>,
) -> Result<Statement, String> {
    match tokens_iter.next() {
        Some(Token::ReturnKeyword) => {}
        _ => return Err("Return was required".to_string()),
    };
    let exp = parse_expression(tokens_iter)?;
    match tokens_iter.next() {
        Some(Token::Semicolon) => {}
        _ => return Err("; was required".to_string()),
    };
    Ok(Statement::Return(exp))
}
pub fn parse(tokens: Vec<Token>) -> Result<Function, String> {
    let mut tokens_iter = tokens.into_iter().peekable();
    match tokens_iter.next() {
        Some(Token::IntKeyword) => {}
        _ => return Err("int was required as a first token".to_string()),
    };
    let function_name = match tokens_iter.next() {
        Some(Token::Identifier(_function_name)) => _function_name,
        _ => return Err("function name not found".to_string()),
    };
    match tokens_iter.next() {
        Some(Token::OpenParen) => {}
        _ => return Err("( was required".to_string()),
    };
    match tokens_iter.next() {
        Some(Token::CloseParen) => {}
        _ => return Err(") was required".to_string()),
    };
    match tokens_iter.next() {
        Some(Token::OpenBrace) => {}
        _ => return Err("{ was required".to_string()),
    };

    let body = parse_statement(&mut tokens_iter)?;

    match tokens_iter.next() {
        Some(Token::CloseBrace) => {}
        _ => return Err("} was required".to_string()),
    };

    Ok(Function {
        name: function_name,
        body: body,
    })
}

