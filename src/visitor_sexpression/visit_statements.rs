use crate::{
    ast::{Expression, ExpressionDispatcher, ExpressionList, StatementDispatcher, StatementList},
    Statement,
};

use super::SExpressionVisitor;
use anyhow::Result;

pub(super) fn visit_statement(
    visitor: &mut SExpressionVisitor,
    statement: &Statement,
) -> Result<()> {
    let result = match statement {
        Statement::Program { body } => visit_program_statement(visitor, body),
        Statement::Block { body } => visit_block_statement(visitor, body),
        Statement::Empty => visit_empty_statement(visitor),
        Statement::Expression { expression } => visit_expression_statement(visitor, expression),
        Statement::VariableDeclaration { variables } => {
            visit_variable_declaration_statement(visitor, variables)
        }
        Statement::Conditional {
            condition,
            consequent,
            alternative,
        } => visit_conditional_statement(visitor, condition, consequent, alternative.as_deref()),
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

fn visit_conditional_statement(
    visitor: &mut SExpressionVisitor,
    condition: &Expression,
    consequent: &Statement,
    alternative: Option<&Statement>,
) -> Result<()> {
    visitor.begin_expr("if")?;

    // Process condition
    visitor.write_space_or_newline()?;
    condition.accept(visitor)?;

    // Process consequent
    visitor.write_space_or_newline()?;
    consequent.accept(visitor)?;

    // Process alternative if present
    if let Some(alt) = alternative {
        visitor.write_space_or_newline()?;
        alt.accept(visitor)?;
    }

    visitor.end_expr()?;

    Ok(())
}
