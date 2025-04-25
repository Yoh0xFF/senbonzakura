use super::{
    ast_operators::{AssignmentOperator, BinaryOperator, LogicalOperator, UnaryOperator},
    ast_types::Type,
};

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
    If {
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
    },
    FunctionDeclaration {
        name: ExpressionRef,
        parameters: ExpressionList,
        body: StatementRef,
    },
    Return {
        argument: Option<ExpressionRef>,
    },
    ClassDeclaration {
        name: ExpressionRef,
        super_class: Option<ExpressionRef>,
        body: StatementRef,
    },
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub enum Expression {
    VariableInitialization {
        identifier: ExpressionRef,
        type_annotation: Type,
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
    Member {
        computed: bool,
        object: ExpressionRef,
        property: ExpressionRef,
    },
    Call {
        callee: ExpressionRef,
        arguments: ExpressionList,
    },
    This {},
    Super {},
    New {
        callee: ExpressionRef,
        arguments: ExpressionList,
    },
}
