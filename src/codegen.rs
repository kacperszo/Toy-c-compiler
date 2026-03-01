use std::fmt::format;

use crate::ast::{BinaryOperator, Expression, Function, Statement};

pub fn generate_expression(expr: &Expression) -> String {
    match expr {
        Expression::Constant(v) => format!("mov rax, {}\n", v),
        Expression::BinaryOperation(left, op, right) => {
            let mut asm = generate_expression(right.as_ref());
            asm.push_str("  push rax\n");
            asm.push_str(&generate_expression(left.as_ref()));
            asm.push_str("  pop rcx\n");
            let op_asm = match op {
                BinaryOperator::Add => "   add rax, rcx\n",
                BinaryOperator::Subtract => "   sub rax, rcx\n",
                BinaryOperator::Multiply => "   imul rax, rcx\n",
                BinaryOperator::Divide => "   cqo\n   idiv rcx\n",
            };
            asm.push_str(op_asm);
            asm
        }
    }
}
pub fn generate_statment(stmnt: &Statement) -> String {
    match stmnt {
        Statement::Return(e) => {
            let mut asm = generate_expression(e);
            asm.push_str("    ret\n");
            asm
        }
        _ => "ERRO!".to_string(),
    }
}
pub fn generate_function(func: &Function) -> String {
    format!(
        "   .intel_syntax noprefix\n    .global {}\n{}:\n{}",
        func.name,
        func.name,
        generate_statment(&func.body)
    )
}
