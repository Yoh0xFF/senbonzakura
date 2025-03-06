use std::rc::Rc;

pub type StatementRef = Rc<Statement>;
pub type ExpressionRef = Rc<Expression>;
pub type StatementList = Rc<Vec<Rc<Statement>>>;

#[derive(Debug, Clone, PartialEq, Eq)]
pub enum Statement {
    Program { body: StatementList },
    BlockStatement { body: StatementList },
    EmptyStatement,
    ExpressionStatement { expression: ExpressionRef },
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub enum Expression {
    StringLiteral(String),
    NumericLiteral(i32),
}
