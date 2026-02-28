mod lexer;

fn main() {
    let source_code = "int main() { return 2; }";
    let tokens = lexer::tokenize(source_code);
    println!("Tokens {:?}", tokens)
}
