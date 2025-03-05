use std::rc::Rc;

#[derive(Debug, Clone, PartialEq, Eq)]
pub enum Expression {
    Program { body: Rc<Vec<Rc<Expression>>> },
    ExpressionStatement { expression: Rc<Expression> },
    StringLiteral(String),
    NumericLiteral(i32),
}
