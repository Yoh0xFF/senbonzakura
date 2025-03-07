use crate::ast::{Expression, ExpressionDispatcher, ExpressionNode};

use super::SExpressionVisitor;
use anyhow::Result;
use std::fmt::Write;

pub(super) fn visit_expression(
    visitor: &mut SExpressionVisitor,
    expression: &Expression,
) -> Result<()> {
    match expression.as_ref() {
        ExpressionNode::Binary {
            operator,
            left,
            right,
        } => {
            visitor.begin_expr("binary")?;

            visitor.write_space_or_newline()?;
            visitor.write_indent()?;
            write!(visitor.output, "\"{}\"", operator)?;

            visitor.write_space_or_newline()?;
            left.accept(visitor)?;

            visitor.write_space_or_newline()?;
            right.accept(visitor)?;

            visitor.end_expr()?;
        }
        ExpressionNode::StringLiteral(value) => {
            visitor.begin_expr("string")?;

            let escaped = value.replace('\"', "\\\"");
            write!(visitor.output, " \"{}\"", escaped)?;

            visitor.end_expr()?;
        }
        ExpressionNode::NumericLiteral(value) => {
            visitor.begin_expr("number")?;
            write!(visitor.output, " {}", value)?;
            visitor.end_expr()?;
        }
    }

    Ok(())
}
