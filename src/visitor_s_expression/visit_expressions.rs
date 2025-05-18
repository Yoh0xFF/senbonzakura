use crate::ast::{
    AssignmentOperator, BinaryOperator, Expression, ExpressionDispatcher, ExpressionList,
    ExpressionRef, LogicalOperator, Type, UnaryOperator,
};

use super::{visit_types::visit_type, SExpressionVisitor};
use anyhow::Result;
use std::fmt::Write;

pub(super) fn visit_expression(
    visitor: &mut SExpressionVisitor,
    expression: &Expression,
) -> Result<()> {
    let result = match expression {
        Expression::Variable {
            identifier,
            type_annotation,
            initializer,
        } => {
            visit_variable_expression(visitor, identifier, type_annotation, initializer.as_deref())
        }
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
        Expression::BooleanLiteral { value } => visit_boolean_literal_expression(visitor, *value),
        Expression::NilLiteral => visit_nil_literal_expression(visitor),
        Expression::NumericLiteral { value } => visit_numeric_literal_expression(visitor, *value),
        Expression::StringLiteral { value } => visit_string_literal_expression(visitor, value),
        Expression::Identifier { name } => visit_identifier_expression(visitor, name),
        Expression::Member {
            computed,
            object,
            property,
        } => visit_member_expression(visitor, *computed, object, property),
        Expression::Call { callee, arguments } => visit_call_expression(visitor, callee, arguments),
        Expression::This {} => visit_this_expression(visitor),
        Expression::Super {} => visit_super_expression(visitor),
        Expression::New { callee, arguments } => visit_new_expression(visitor, callee, arguments),
    };

    result
}

fn visit_variable_expression(
    visitor: &mut SExpressionVisitor,
    identifier: &Expression,
    type_annotation: &Type,
    initializer: Option<&Expression>,
) -> Result<()> {
    visitor.begin_expr("init")?;

    // Process identifier
    visitor.write_space_or_newline()?;
    identifier.accept(visitor)?;

    // Add type annotation to S-expression
    visitor.write_space_or_newline()?;
    visitor.begin_expr("type")?;
    visit_type(visitor, type_annotation)?;
    visitor.end_expr()?;

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
    visitor.begin_expr("member")?;

    // Indicate whether the member access is computed (bracket notation) or not (dot notation)
    visitor.write_space_or_newline()?;
    visitor.write_indent()?;
    write!(
        visitor.output,
        "\"{}\"",
        if computed { "computed" } else { "static" }
    )?;

    // Process object expression (the left part of the member access)
    visitor.write_space_or_newline()?;
    object.accept(visitor)?;

    // Process property expression (the right part of the member access)
    visitor.write_space_or_newline()?;
    property.accept(visitor)?;

    visitor.end_expr()?;

    Ok(())
}

fn visit_call_expression(
    visitor: &mut SExpressionVisitor,
    callee: &ExpressionRef,
    arguments: &ExpressionList,
) -> Result<()> {
    visitor.begin_expr("call")?;

    // Process callee expression
    visitor.write_space_or_newline()?;
    callee.accept(visitor)?;

    if !arguments.is_empty() {
        // Create args expression
        visitor.write_space_or_newline()?;
        visitor.begin_expr("args")?;

        // Process each argument
        for arg in arguments.iter() {
            visitor.write_space_or_newline()?;
            arg.accept(visitor)?;
        }

        visitor.end_expr()?; // Close args expression
    }

    visitor.end_expr()?;

    Ok(())
}

fn visit_this_expression(
    visitor: &mut SExpressionVisitor,
) -> std::result::Result<(), anyhow::Error> {
    visitor.begin_expr("this")?;
    visitor.end_expr()?;

    Ok(())
}

fn visit_super_expression(
    visitor: &mut SExpressionVisitor,
) -> std::result::Result<(), anyhow::Error> {
    visitor.begin_expr("super")?;
    visitor.end_expr()?;

    Ok(())
}

fn visit_new_expression(
    visitor: &mut SExpressionVisitor,
    callee: &Expression,
    arguments: &ExpressionList,
) -> std::result::Result<(), anyhow::Error> {
    visitor.begin_expr("new")?;

    // Process callee expression
    visitor.write_space_or_newline()?;
    callee.accept(visitor)?;

    if !arguments.is_empty() {
        // Create args expression
        visitor.write_space_or_newline()?;
        visitor.begin_expr("args")?;

        // Process each argument
        for arg in arguments.iter() {
            visitor.write_space_or_newline()?;
            arg.accept(visitor)?;
        }

        visitor.end_expr()?; // Close args expression
    }

    visitor.end_expr()?;

    Ok(())
}
