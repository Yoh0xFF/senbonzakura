use super::{
    ast_operators::{AssignmentOperator, BinaryOperator, LogicalOperator, UnaryOperator},
    ast_types::Type,
};

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Parameter {
    pub name: String,
    pub type_annotation: Type,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct VariableDeclarator {
    pub name: String,
    pub type_annotation: Type,
    pub initializer: Option<Box<Expression>>,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub enum MemberProperty {
    Named(String),
    Computed(Box<Expression>),
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub enum Statement {
    Program {
        body: Vec<Statement>,
    },

    Block {
        body: Vec<Statement>,
    },

    Empty,

    Expression {
        expression: Box<Expression>,
    },

    VariableDeclaration {
        variables: Vec<VariableDeclarator>,
    },

    If {
        condition: Box<Expression>,
        consequent: Box<Statement>,
        alternative: Option<Box<Statement>>,
    },

    While {
        condition: Box<Expression>,
        body: Box<Statement>,
    },

    DoWhile {
        body: Box<Statement>,
        condition: Box<Expression>,
    },

    For {
        initializer: Option<Box<Statement>>,
        condition: Option<Box<Expression>>,
        increment: Option<Box<Expression>>,
        body: Box<Statement>,
    },

    FunctionDeclaration {
        name: String,
        parameters: Vec<Parameter>,
        return_type: Type,
        body: Vec<Statement>,
    },

    Return {
        argument: Option<Box<Expression>>,
    },

    ClassDeclaration {
        name: String,
        super_class: Option<String>,
        body: Vec<Statement>,
    },
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub enum Expression {
    Assignment {
        operator: AssignmentOperator,
        left: Box<Expression>,
        right: Box<Expression>,
    },

    Binary {
        operator: BinaryOperator,
        left: Box<Expression>,
        right: Box<Expression>,
    },

    Unary {
        operator: UnaryOperator,
        right: Box<Expression>,
    },

    Logical {
        operator: LogicalOperator,
        left: Box<Expression>,
        right: Box<Expression>,
    },

    BooleanLiteral {
        value: bool,
    },

    NilLiteral,

    StringLiteral {
        value: String,
    },

    NumericLiteral {
        value: i32,
    },

    Identifier {
        name: String,
    },

    Member {
        object: Box<Expression>,
        property: MemberProperty,
    },

    Call {
        callee: Box<Expression>,
        arguments: Vec<Expression>,
    },

    This {},

    Super {},

    New {
        callee: Box<Expression>,
        arguments: Vec<Expression>,
    },
}
