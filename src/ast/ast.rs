use std::{fmt, rc::Rc};

pub type Statement = Rc<StatementNode>;
pub type Expression = Rc<ExpressionNode>;
pub type StatementList = Rc<Vec<Rc<StatementNode>>>;

#[derive(Debug, Clone, PartialEq, Eq)]
pub enum StatementNode {
    Program { body: StatementList },
    Block { body: StatementList },
    Empty,
    Expression { expression: Expression },
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub enum ExpressionNode {
    Binary {
        operator: BinaryOperator,
        left: Expression,
        right: Expression,
    },
    StringLiteral(String),
    NumericLiteral(i32),
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum BinaryOperator {
    Add,
    Subtract,
    Multiply,
    Divide,
}

impl fmt::Display for BinaryOperator {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            BinaryOperator::Add => write!(f, "+"),
            BinaryOperator::Subtract => write!(f, "-"),
            BinaryOperator::Multiply => write!(f, "*"),
            BinaryOperator::Divide => write!(f, "/"),
        }
    }
}
