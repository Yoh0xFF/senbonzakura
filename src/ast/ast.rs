use std::rc::Rc;

pub type StatementRef = Rc<Statement>;
pub type ExpressionRef = Rc<Expression>;
pub type StatementList = Rc<Vec<Rc<Statement>>>;

#[derive(Debug, Clone, PartialEq, Eq)]
pub enum Statement {
    Program { body: StatementList },
    Block { body: StatementList },
    Empty,
    Expression { expression: ExpressionRef },
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub enum Expression {
    Binary {
        operator: String,
        left: ExpressionRef,
        right: ExpressionRef,
    },
    StringLiteral(String),
    NumericLiteral(i32),
}
