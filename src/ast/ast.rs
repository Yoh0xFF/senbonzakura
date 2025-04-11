use std::fmt;

pub type StatementRef = Box<Statement>;
pub type ExpressionRef = Box<Expression>;
pub type StatementList = Vec<Statement>;
pub type ExpressionList = Vec<Expression>;

#[derive(Debug, Clone, PartialEq, Eq)]
pub enum Statement {
    Program {
        body: StatementList,
    },
    Block {
        body: StatementList,
    },
    Empty,
    Expression {
        expression: ExpressionRef,
    },
    VariableDeclaration {
        variables: ExpressionList,
    },
    Conditional {
        condition: ExpressionRef,
        consequent: StatementRef,
        alternative: Option<StatementRef>,
    },
    While {
        condition: ExpressionRef,
        body: StatementRef,
    },
    DoWhile {
        body: StatementRef,
        condition: ExpressionRef,
    },
    For {
        initializer: Option<StatementRef>,
        condition: Option<ExpressionRef>,
        increment: Option<ExpressionRef>,
        body: StatementRef,
    }
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub enum Expression {
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
    Unary {
        operator: UnaryOperator,
        right: ExpressionRef,
    },
    Logical {
        operator: LogicalOperator,
        left: ExpressionRef,
        right: ExpressionRef,
    },
    BooleanLiteral(bool),
    NilLiteral,
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
    Equal,
    NotEqual,
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
            BinaryOperator::Equal => write!(f, "=="),
            BinaryOperator::NotEqual => write!(f, "!="),
            BinaryOperator::GreaterThan => write!(f, ">"),
            BinaryOperator::GreaterThanOrEqualTo => write!(f, ">="),
            BinaryOperator::LessThan => write!(f, "<"),
            BinaryOperator::LessThanOrEqualTo => write!(f, "<="),
        }
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum UnaryOperator {
    Plus,
    Minus,
    Not,
}

impl fmt::Display for UnaryOperator {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            UnaryOperator::Plus => write!(f, "+"),
            UnaryOperator::Minus => write!(f, "-"),
            UnaryOperator::Not => write!(f, "!"),
        }
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum LogicalOperator {
    And,
    Or,
}

impl fmt::Display for LogicalOperator {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            LogicalOperator::And => write!(f, "&&"),
            LogicalOperator::Or => write!(f, "||"),
        }
    }
}
