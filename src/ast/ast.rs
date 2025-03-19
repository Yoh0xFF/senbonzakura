use std::fmt;

pub type StatementRef = Box<StatementNode>;
pub type ExpressionRef = Box<ExpressionNode>;
pub type StatementRefList = Vec<StatementRef>;
pub type ExpressionRefList = Vec<ExpressionRef>;

#[derive(Debug, Clone, PartialEq, Eq)]
pub enum StatementNode {
    Program {
        body: StatementRefList,
    },
    Block {
        body: StatementRefList,
    },
    Empty,
    Expression {
        expression: ExpressionRef,
    },
    VariableDeclaration {
        variables: ExpressionRefList,
    },
    Conditional {
        condition: ExpressionRef,
        consequent: StatementRef,
        alternative: Option<StatementRef>,
    },
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub enum ExpressionNode {
    VariableIntialization {
        identifier: ExpressionRef,
        initializer: Option<ExpressionRef>,
    },
    Assignment {
        operator: AssignmentOperator,
        left: ExpressionRef,
        right: ExpressionRef,
    },
    Binary {
        operator: BinaryOperator,
        left: ExpressionRef,
        right: ExpressionRef,
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
