use crate::{
    ast::{
        AstVisitor, Expression, ExpressionDispatcher, ExpressionNode, Statement,
        StatementDispatcher,
    },
    StatementNode,
};
use anyhow::Result;

/**
 * S-Expression Visitor for AST nodes
 */
pub struct SExpressionVisitor;

impl SExpressionVisitor {
    pub fn new() -> Self {
        SExpressionVisitor
    }

    pub fn statement_to_sexpression(&mut self, statement: Statement) -> Result<String> {
        statement.accept(self)
    }
}

impl AstVisitor for SExpressionVisitor {
    type Output = String;

    fn visit_statement(&mut self, statement: Statement) -> Result<Self::Output> {
        match statement.as_ref() {
            StatementNode::Program { body } => {
                let body_sexp: Result<Vec<_>> = body
                    .iter()
                    .map(|statement| self.statement_to_sexpression(statement.clone()))
                    .collect();

                Ok(format!("(program {})", body_sexp?.join(" ")))
            }
            StatementNode::Block { body } => {
                let body_sexp: Result<Vec<_>> = body
                    .iter()
                    .map(|statement| self.statement_to_sexpression(statement.clone()))
                    .collect();

                Ok(format!("(block {})", body_sexp?.join(" ")))
            }
            StatementNode::Empty => Ok("(empty)".to_string()),
            StatementNode::Expression { expression } => {
                let expr_sexp = self.visit_expression(expression.clone())?;
                Ok(format!("(expr {})", expr_sexp))
            }
        }
    }

    fn visit_expression(&mut self, expression: Expression) -> Result<Self::Output> {
        match expression.as_ref() {
            ExpressionNode::Binary {
                operator,
                left,
                right,
            } => {
                let left_sexp = self.visit_expression(left.clone())?;
                let right_sexp = self.visit_expression(right.clone())?;

                Ok(format!(
                    "(binary \"{}\" {} {})",
                    operator, left_sexp, right_sexp
                ))
            }
            ExpressionNode::StringLiteral(value) => {
                // Escape double quotes
                let escaped = value.replace("\"", "\\\"");
                Ok(format!("(string \"{}\")", escaped))
            }
            ExpressionNode::NumericLiteral(value) => Ok(format!("(number {})", value)),
        }
    }
}

/**
 * Add extension trait for convenient s-expression conversion
 */
pub trait ToSExpression {
    fn to_sexpression(&self) -> Result<String>;
}

impl ToSExpression for Statement {
    fn to_sexpression(&self) -> Result<String> {
        let mut visitor = SExpressionVisitor::new();
        self.accept(&mut visitor)
    }
}

impl ToSExpression for Expression {
    fn to_sexpression(&self) -> Result<String> {
        let mut visitor = SExpressionVisitor::new();
        self.accept(&mut visitor)
    }
}
