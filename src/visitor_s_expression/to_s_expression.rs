use crate::ast::{ExpressionDispatcher, ExpressionRef, StatementDispatcher, StatementRef};

use super::{SExpressionConfig, SExpressionVisitor};

use anyhow::Result;

/**
 * Add extension trait for convenient s-expression conversion
 */
pub trait ToSExpression {
    #[allow(dead_code)]
    fn to_s_expression(&self) -> Result<String>;

    #[allow(dead_code)]
    fn to_pretty_s_expression(&self) -> Result<String>;
}

impl ToSExpression for StatementRef {
    fn to_s_expression(&self) -> Result<String> {
        let mut visitor = SExpressionVisitor::new();
        visitor.output.clear();
        self.accept(&mut visitor)?;
        Ok(visitor.output)
    }

    fn to_pretty_s_expression(&self) -> Result<String> {
        let config = SExpressionConfig {
            pretty: true,
            indent_size: 2,
        };

        let mut visitor = SExpressionVisitor::with_config(config);
        visitor.output.clear();
        self.accept(&mut visitor)?;
        Ok(visitor.output)
    }
}

impl ToSExpression for ExpressionRef {
    fn to_s_expression(&self) -> Result<String> {
        let mut visitor = SExpressionVisitor::new();
        visitor.output.clear();
        self.accept(&mut visitor)?;
        Ok(visitor.output)
    }

    fn to_pretty_s_expression(&self) -> Result<String> {
        let config = SExpressionConfig {
            pretty: true,
            indent_size: 2,
        };

        let mut visitor = SExpressionVisitor::with_config(config);
        visitor.output.clear();
        self.accept(&mut visitor)?;
        Ok(visitor.output)
    }
}
