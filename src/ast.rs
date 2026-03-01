#[derive(Debug)]
pub enum Statement {
    Return(Expression),
}

#[derive(Debug)]
pub enum Expression {
    Constant(i32),
    BinaryOperation(Box<Expression>, BinaryOperator, Box<Expression>),
}
#[derive(Debug)]
pub enum BinaryOperator {
    Add,
    Subtract,
    Multiply,
    Divide,
}
#[derive(Debug)]
pub struct Function {
    pub name: String,
    pub body: Statement,
}
