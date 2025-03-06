use std::rc::Rc;

#[derive(Debug, Clone, PartialEq, Eq)]
pub enum Statement {
    Program { body: Rc<Vec<Rc<Statement>>> },
    BlockStatement { body: Rc<Vec<Rc<Statement>>> },
    EmptyStatement,
    ExpressionStatement { expression: Rc<Expression> },
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub enum Expression {
    StringLiteral(String),
    NumericLiteral(i32),
}
