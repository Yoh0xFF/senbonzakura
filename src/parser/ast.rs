use std::rc::Rc;

#[derive(Debug, Clone, PartialEq, Eq)]
pub enum Expression {
    Program { body: Rc<Vec<Rc<Expression>>> },
    BlockStatement { body: Rc<Vec<Rc<Expression>>> },
    EmptyStatement,
    ExpressionStatement { expression: Rc<Expression> },
    StringLiteral(String),
    NumericLiteral(i32),
}
