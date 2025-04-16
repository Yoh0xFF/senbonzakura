use crate::ast::{
    AssignmentOperator, BinaryOperator, Expression, ExpressionDispatcher, LogicalOperator,
    UnaryOperator,
};

use super::SExpressionVisitor;
use anyhow::Result;
use std::fmt::Write;

pub(super) fn visit_expression(
    visitor: &mut SExpressionVisitor,
    expression: &Expression,
) -> Result<()> {
    let result = match expression {
        Expression::VariableIntialization {
            identifier,
            initializer,
        } => visit_variable_initialization_expression(visitor, identifier, initializer.as_deref()),
        Expression::Assignment {
            operator,
            left,
            right,
        } => visit_assignment_expression(visitor, *operator, left, right),
        Expression::Binary {
            operator,
            left,
            right,
        } => visit_binary_expression(visitor, *operator, left, right),
        Expression::Unary { operator, right } => visit_unary_expression(visitor, *operator, right),
        Expression::Logical {
            operator,
            left,
            right,
        } => visit_logical_expression(visitor, *operator, left, right),
        Expression::BooleanLiteral(value) => visit_boolean_literal_expression(visitor, *value),
        Expression::NilLiteral => visit_nil_literal_expression(visitor),
        Expression::NumericLiteral(value) => visit_numeric_literal_expression(visitor, *value),
        Expression::StringLiteral(value) => visit_string_literal_expression(visitor, value),
        Expression::Identifier(name) => visit_identifier_expression(visitor, name),
        Expression::Member {
            computed,
            object,
            property,
        } => visit_member_expression(visitor, *computed, object, property),
    };

    result
}

fn visit_variable_initialization_expression(
    visitor: &mut SExpressionVisitor,
    identifier: &Expression,
    initializer: Option<&Expression>,
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
    operator: AssignmentOperator,
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
    operator: BinaryOperator,
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

fn visit_unary_expression(
    visitor: &mut SExpressionVisitor,
    operator: UnaryOperator,
    right: &Expression,
) -> Result<()> {
    visitor.begin_expr("unary")?;

    visitor.write_space_or_newline()?;
    visitor.write_indent()?;
    write!(visitor.output, "\"{}\"", operator)?;

    visitor.write_space_or_newline()?;
    right.accept(visitor)?;

    visitor.end_expr()?;

    Ok(())
}

fn visit_logical_expression(
    visitor: &mut SExpressionVisitor,
    operator: LogicalOperator,
    left: &Expression,
    right: &Expression,
) -> Result<()> {
    visitor.begin_expr("logical")?;

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

fn visit_boolean_literal_expression(visitor: &mut SExpressionVisitor, value: bool) -> Result<()> {
    visitor.begin_expr("boolean")?;
    write!(visitor.output, " {}", value)?;
    visitor.end_expr()?;

    Ok(())
}

fn visit_nil_literal_expression(visitor: &mut SExpressionVisitor) -> Result<()> {
    visitor.begin_expr("nil")?;
    visitor.end_expr()?;

    Ok(())
}

fn visit_numeric_literal_expression(visitor: &mut SExpressionVisitor, value: i32) -> Result<()> {
    visitor.begin_expr("number")?;
    write!(visitor.output, " {}", value)?;
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

fn visit_identifier_expression(visitor: &mut SExpressionVisitor, name: &str) -> Result<()> {
    visitor.begin_expr("id")?;

    // Write the identifier name
    write!(visitor.output, " {}", name)?;

    visitor.end_expr()?;

    Ok(())
}

fn visit_member_expression(
    visitor: &mut SExpressionVisitor,
    computed: bool,
    object: &Expression,
    property: &Expression,
) -> Result<()> {
    // TODO Implement visitor for the member expression
    todo!()
}
