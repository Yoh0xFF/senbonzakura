use crate::{
    ast::{ExpressionDispatcher, Statement, StatementDispatcher},
    StatementNode,
};

use super::SExpressionVisitor;
use anyhow::Result;

pub(super) fn visit_statement(
    visitor: &mut SExpressionVisitor,
    statement: &Statement,
) -> Result<()> {
    match statement.as_ref() {
        StatementNode::Program { body } => {
            visitor.begin_expr("program")?;

            if !body.is_empty() {
                for statement in body.iter() {
                    visitor.write_space_or_newline()?;
                    statement.accept(visitor)?;
                }
            }

            visitor.end_expr()?;
        }
        StatementNode::Block { body } => {
            visitor.begin_expr("block")?;

            if !body.is_empty() {
                for statement in body.iter() {
                    visitor.write_space_or_newline()?;
                    statement.accept(visitor)?;
                }
            }

            visitor.end_expr()?;
        }
        StatementNode::Empty => {
            visitor.begin_expr("empty")?;
            visitor.end_expr()?;
        }
        StatementNode::Expression { expression } => {
            visitor.begin_expr("expr")?;
            visitor.write_space_or_newline()?;
            expression.accept(visitor)?;
            visitor.end_expr()?;
        }
    }

    Ok(())
}
