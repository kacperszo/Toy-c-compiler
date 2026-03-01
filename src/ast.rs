#[derive(Debug)]
pub enum Statement {
    Return(Expression),
}
#[derive(Debug)]
pub enum Expression {
    Constant(i32),
}

#[derive(Debug)]
pub struct Function {
    pub name: String,
    pub body: Statement,
}
