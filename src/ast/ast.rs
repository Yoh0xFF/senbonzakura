use serde::{Deserialize, Serialize};

use super::{
    ast_operators::{AssignmentOperator, BinaryOperator, LogicalOperator, UnaryOperator},
    ast_types::Type,
};

pub type StatementRef = Box<Statement>;
pub type ExpressionRef = Box<Expression>;
pub type StatementList = Vec<Statement>;
pub type ExpressionList = Vec<Expression>;
pub type ParameterList = Vec<(Expression, Type)>;

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
#[serde(tag = "type")]
pub enum Statement {
    #[serde(rename = "Program")]
    Program { body: StatementList },

    #[serde(rename = "Block")]
    Block { body: StatementList },

    #[serde(rename = "Empty")]
    Empty,

    #[serde(rename = "Expression")]
    Expression { expression: ExpressionRef },

    #[serde(rename = "VariableDeclaration")]
    VariableDeclaration { variables: ExpressionList },

    #[serde(rename = "If")]
    If {
        condition: ExpressionRef,
        consequent: StatementRef,
        alternative: Option<StatementRef>,
    },

    #[serde(rename = "While")]
    While {
        condition: ExpressionRef,
        body: StatementRef,
    },

    #[serde(rename = "DoWhile")]
    DoWhile {
        body: StatementRef,
        condition: ExpressionRef,
    },

    #[serde(rename = "For")]
    For {
        initializer: Option<StatementRef>,
        condition: Option<ExpressionRef>,
        increment: Option<ExpressionRef>,
        body: StatementRef,
    },

    #[serde(rename = "FunctionDeclaration")]
    FunctionDeclaration {
        name: ExpressionRef,
        parameters: ParameterList,
        return_type: Type,
        body: StatementRef,
    },

    #[serde(rename = "Return")]
    Return { argument: Option<ExpressionRef> },

    #[serde(rename = "ClassDeclaration")]
    ClassDeclaration {
        name: ExpressionRef,
        super_class: Option<ExpressionRef>,
        body: StatementRef,
    },
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
#[serde(tag = "type")]
pub enum Expression {
    #[serde(rename = "Variable")]
    Variable {
        identifier: ExpressionRef,
        type_annotation: Type,
        initializer: Option<ExpressionRef>,
    },

    #[serde(rename = "Assignment")]
    Assignment {
        operator: AssignmentOperator,
        left: ExpressionRef,
        right: ExpressionRef,
    },

    #[serde(rename = "Binary")]
    Binary {
        operator: BinaryOperator,
        left: ExpressionRef,
        right: ExpressionRef,
    },

    #[serde(rename = "Unary")]
    Unary {
        operator: UnaryOperator,
        right: ExpressionRef,
    },

    #[serde(rename = "Logical")]
    Logical {
        operator: LogicalOperator,
        left: ExpressionRef,
        right: ExpressionRef,
    },

    #[serde(rename = "BooleanLiteral")]
    BooleanLiteral { value: bool },

    #[serde(rename = "NilLiteral")]
    NilLiteral,

    #[serde(rename = "StringLiteral")]
    StringLiteral { value: String },

    #[serde(rename = "NumericLiteral")]
    NumericLiteral { value: i32 },

    #[serde(rename = "Identifier")]
    Identifier { name: String },

    #[serde(rename = "Member")]
    Member {
        computed: bool,
        object: ExpressionRef,
        property: ExpressionRef,
    },

    #[serde(rename = "Call")]
    Call {
        callee: ExpressionRef,
        arguments: ExpressionList,
    },

    #[serde(rename = "This")]
    This {},

    #[serde(rename = "Super")]
    Super {},

    #[serde(rename = "New")]
    New {
        callee: ExpressionRef,
        arguments: ExpressionList,
    },
}
