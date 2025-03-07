use anyhow::Result;

use super::{ExpressionRef, StatementRef};

/**
 * A trait for any compiler pass that processes the AST
 */
pub trait AstVisitor {
    /**
     * The type returned by this visitor
     */
    type Output;

    /**
     * Process a Statement node
     */
    fn visit_statement(&mut self, statement: StatementRef) -> Result<Self::Output>;

    /**
     * Process an Expression node
     */
    fn visit_expression(&mut self, expression: ExpressionRef) -> Result<Self::Output>;
}

/**
 * A trait to extend Statement with visitor methods
 */
pub trait StatementDispatcher {
    /**
     * Apply a visitor to this statement
     */
    fn accept<V: AstVisitor>(&self, visitor: &mut V) -> Result<V::Output>;
}

/**
 * A trait to extend Expression with visitor methods
 */
pub trait ExpressionDispatcher {
    /**
     * Apply a visitor to this expression
     */
    fn accept<V: AstVisitor>(&self, visitor: &mut V) -> Result<V::Output>;
}

/**
 * Implementation for Statement
 */
impl StatementDispatcher for StatementRef {
    fn accept<V: AstVisitor>(&self, visitor: &mut V) -> Result<V::Output> {
        visitor.visit_statement(self.clone())
    }
}

/**
 * Implementation for Expression
 */
impl ExpressionDispatcher for ExpressionRef {
    fn accept<V: AstVisitor>(&self, visitor: &mut V) -> Result<V::Output> {
        visitor.visit_expression(self.clone())
    }
}
