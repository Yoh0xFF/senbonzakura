use std::rc::Rc;

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
        operator: String,
        left: Expression,
        right: Expression,
    },
    StringLiteral(String),
    NumericLiteral(i32),
}
