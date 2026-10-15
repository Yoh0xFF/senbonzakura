use serde::{Deserialize, Serialize};

use super::{
    ast_operators::{AssignmentOperator, BinaryOperator, LogicalOperator, UnaryOperator},
    ast_types::Type,
};

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct Parameter {
    pub name: String,
    pub type_annotation: Type,
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
#[serde(tag = "type", rename_all = "PascalCase")]
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
        variables: Vec<Expression>,
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
        body: Box<Statement>,
    },

    Return {
        argument: Option<Box<Expression>>,
    },

    ClassDeclaration {
        name: String,
        super_class: Option<String>,
        body: Box<Statement>,
    },
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
#[serde(tag = "type", rename_all = "PascalCase")]
pub enum Expression {
    Variable {
        identifier: String,
        type_annotation: Type,
        initializer: Option<Box<Expression>>,
    },

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
        computed: bool,
        object: Box<Expression>,
        property: Box<Expression>,
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
