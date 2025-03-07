use crate::ast::{BinaryOperator, Expression, ExpressionDispatcher, ExpressionNode};

use super::SExpressionVisitor;
use anyhow::Result;
use std::fmt::Write;

pub(super) fn visit_expression(
    visitor: &mut SExpressionVisitor,
    expression: &Expression,
) -> Result<()> {
    let result = match expression.as_ref() {
        ExpressionNode::Binary {
            operator,
            left,
            right,
        } => visit_binary_expression(visitor, *operator, left, right),
        ExpressionNode::StringLiteral(value) => visit_string_literal_expression(visitor, value),
        ExpressionNode::NumericLiteral(value) => visit_numeric_literal_expression(visitor, *value),
    };

    result
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
