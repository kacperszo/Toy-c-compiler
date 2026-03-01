mod ast;
mod lexer;
mod parser;

fn main() {
    let source_code = "int main() { return 2; }";
    let tokens = lexer::tokenize(source_code);
    println!("Tokens {:?}", tokens);
    let _ast = parser::parse(tokens.expect("You cannot parse while there are tokenization errors"));
    println!("AST {:?}", _ast);
}
