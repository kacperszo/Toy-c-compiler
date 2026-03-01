mod ast;
mod codegen;
mod lexer;
mod parser;

use std::fs;
use std::process::Command;

fn main() {
    let source_code = "int main() { return 2 + 2 - 2 - 2 + 2; }";
    let tokens = lexer::tokenize(source_code);
    println!("Tokens {:?}\n\n\n", tokens);
    let _ast = parser::parse(tokens.expect("You cannot parse while there are tokenization errors"));
    println!("AST {:?}\n\n\n", _ast);
    let asm = codegen::generate_function(&_ast.unwrap());
    println!("asm: {:?}\n\n\n", asm);
    let asm_filename = "out.s";
    fs::write(asm_filename, &asm).map_err(|e| format!("{}", e));
}
