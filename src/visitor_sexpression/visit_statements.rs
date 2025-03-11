use crate::{
    ast::{
        Expression, ExpressionDispatcher, ExpressionList, Statement, StatementDispatcher,
        StatementList,
    },
    StatementNode,
};

use super::SExpressionVisitor;
use anyhow::Result;

pub(super) fn visit_statement(
    visitor: &mut SExpressionVisitor,
    statement: &Statement,
) -> Result<()> {
    let result = match statement.as_ref() {
        StatementNode::Program { body } => visit_program_statement(visitor, body),
        StatementNode::Block { body } => visit_block_statement(visitor, body),
        StatementNode::Empty => visit_empty_statement(visitor),
        StatementNode::Expression { expression } => visit_expression_statement(visitor, expression),
        StatementNode::VariableDeclaration { variables } => {
            visit_variable_declaration_statement(visitor, variables)
        }
    };

    result
}

fn visit_program_statement(visitor: &mut SExpressionVisitor, body: &StatementList) -> Result<()> {
    visitor.begin_expr("program")?;

    if !body.is_empty() {
        for statement in body.iter() {
            visitor.write_space_or_newline()?;
            statement.accept(visitor)?;
        }
    }

    visitor.end_expr()?;

    Ok(())
}

fn visit_block_statement(visitor: &mut SExpressionVisitor, body: &StatementList) -> Result<()> {
    visitor.begin_expr("block")?;

    if !body.is_empty() {
        for statement in body.iter() {
            visitor.write_space_or_newline()?;
            statement.accept(visitor)?;
        }
    }

    visitor.end_expr()?;

    Ok(())
}

fn visit_empty_statement(visitor: &mut SExpressionVisitor) -> Result<()> {
    visitor.begin_expr("empty")?;
    visitor.end_expr()?;

    Ok(())
}

fn visit_expression_statement(
    visitor: &mut SExpressionVisitor,
    expression: &Expression,
) -> Result<()> {
    visitor.begin_expr("expr")?;
    visitor.write_space_or_newline()?;
    expression.accept(visitor)?;
    visitor.end_expr()?;

    Ok(())
}

fn visit_variable_declaration_statement(
    visitor: &mut SExpressionVisitor,
    variables: &ExpressionList,
) -> Result<()> {
    visitor.begin_expr("let")?;

    for variable in variables.iter() {
        visitor.write_space_or_newline()?;
        variable.accept(visitor)?;
    }

    visitor.end_expr()?;

    Ok(())
}
