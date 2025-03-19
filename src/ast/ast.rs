use std::fmt;

pub type Statement = Box<StatementNode>;
pub type Expression = Box<ExpressionNode>;
pub type StatementList = Vec<Statement>;
pub type ExpressionList = Vec<Expression>;

#[derive(Debug, Clone, PartialEq, Eq)]
pub enum StatementNode {
    Program {
        body: StatementList,
    },
    Block {
        body: StatementList,
    },
    Empty,
    Expression {
        expression: Expression,
    },
    VariableDeclaration {
        variables: ExpressionList,
    },
    Conditional {
        condition: Expression,
        consequent: Statement,
        alternative: Option<Statement>,
    },
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub enum ExpressionNode {
    VariableIntialization {
        identifier: Expression,
        initializer: Option<Expression>,
    },
    Assignment {
        operator: AssignmentOperator,
        left: Expression,
        right: Expression,
    },
    Binary {
        operator: BinaryOperator,
        left: Expression,
        right: Expression,
    },
    StringLiteral(String),
    NumericLiteral(i32),
    Identifier(String),
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum AssignmentOperator {
    Assign,
    AssignAdd,
    AssignSubtract,
    AssignMultiply,
    AssignDivide,
}

impl fmt::Display for AssignmentOperator {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            AssignmentOperator::Assign => write!(f, "="),
            AssignmentOperator::AssignAdd => write!(f, "+="),
            AssignmentOperator::AssignSubtract => write!(f, "-="),
            AssignmentOperator::AssignMultiply => write!(f, "*="),
            AssignmentOperator::AssignDivide => write!(f, "/="),
        }
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum BinaryOperator {
    Add,
    Subtract,
    Multiply,
    Divide,
    GreaterThan,
    GreaterThanOrEqualTo,
    LessThan,
    LessThanOrEqualTo,
}

impl fmt::Display for BinaryOperator {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            BinaryOperator::Add => write!(f, "+"),
            BinaryOperator::Subtract => write!(f, "-"),
            BinaryOperator::Multiply => write!(f, "*"),
            BinaryOperator::Divide => write!(f, "/"),
            BinaryOperator::GreaterThan => write!(f, ">"),
            BinaryOperator::GreaterThanOrEqualTo => write!(f, ">="),
            BinaryOperator::LessThan => write!(f, "<"),
            BinaryOperator::LessThanOrEqualTo => write!(f, "<="),
        }
    }
}
