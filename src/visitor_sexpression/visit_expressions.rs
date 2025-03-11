use crate::ast::{
    AssignmentOperator, BinaryOperator, Expression, ExpressionDispatcher, ExpressionNode,
};

use super::SExpressionVisitor;
use anyhow::Result;
use std::fmt::Write;

pub(super) fn visit_expression(
    visitor: &mut SExpressionVisitor,
    expression: &Expression,
) -> Result<()> {
    let result = match expression.as_ref() {
        ExpressionNode::VariableIntialization {
            identifier,
            initializer,
        } => visit_variable_initialization_expression(visitor, identifier, initializer),
        ExpressionNode::Assignment {
            operator,
            left,
            right,
        } => visit_assignment_expression(visitor, operator, left, right),
        ExpressionNode::Binary {
            operator,
            left,
            right,
        } => visit_binary_expression(visitor, operator, left, right),
        ExpressionNode::StringLiteral(value) => visit_string_literal_expression(visitor, value),
        ExpressionNode::NumericLiteral(value) => visit_numeric_literal_expression(visitor, *value),
        ExpressionNode::Identifier(name) => visit_identifier_expression(visitor, name),
    };

    result
}

fn visit_variable_initialization_expression(
    visitor: &mut SExpressionVisitor,
    identifier: &Expression,
    initializer: &Option<Expression>,
) -> Result<()> {
    visitor.begin_expr("init")?;

    // Process identifier
    visitor.write_space_or_newline()?;
    identifier.accept(visitor)?;

    // Process initializer if present
    if let Some(init) = initializer {
        visitor.write_space_or_newline()?;
        init.accept(visitor)?;
    }

    visitor.end_expr()?;

    Ok(())
}

fn visit_assignment_expression(
    visitor: &mut SExpressionVisitor,
    operator: &AssignmentOperator,
    left: &Expression,
    right: &Expression,
) -> Result<()> {
    visitor.begin_expr("assign")?;

    // Write the operator
    visitor.write_space_or_newline()?;
    visitor.write_indent()?;

    // Convert operator to appropriate string representation
    write!(visitor.output, "\"{}\"", operator)?;

    // Process left operand
    visitor.write_space_or_newline()?;
    left.accept(visitor)?;

    // Process right operand
    visitor.write_space_or_newline()?;
    right.accept(visitor)?;

    visitor.end_expr()?;

    Ok(())
}

fn visit_binary_expression(
    visitor: &mut SExpressionVisitor,
    operator: &BinaryOperator,
    left: &Expression,
    right: &Expression,
) -> Result<()> {
    visitor.begin_expr("binary")?;

    visitor.write_space_or_newline()?;
    visitor.write_indent()?;
    write!(visitor.output, "\"{}\"", operator)?;

    visitor.write_space_or_newline()?;
    left.accept(visitor)?;

    visitor.write_space_or_newline()?;
    right.accept(visitor)?;

    visitor.end_expr()?;

    Ok(())
}

fn visit_string_literal_expression(visitor: &mut SExpressionVisitor, value: &str) -> Result<()> {
    visitor.begin_expr("string")?;

    let escaped = value.replace('\"', "\\\"");
    write!(visitor.output, " \"{}\"", escaped)?;

    visitor.end_expr()?;

    Ok(())
}

fn visit_numeric_literal_expression(visitor: &mut SExpressionVisitor, value: i32) -> Result<()> {
    visitor.begin_expr("number")?;
    write!(visitor.output, " {}", value)?;
    visitor.end_expr()?;

    Ok(())
}

fn visit_identifier_expression(visitor: &mut SExpressionVisitor, name: &str) -> Result<()> {
    visitor.begin_expr("id")?;

    // Write the identifier name
    write!(visitor.output, " {}", name)?;

    visitor.end_expr()?;

    Ok(())
}
