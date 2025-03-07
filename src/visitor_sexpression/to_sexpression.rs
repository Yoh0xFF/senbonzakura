use crate::ast::{Expression, ExpressionDispatcher, Statement, StatementDispatcher};

use super::{SExpressionConfig, SExpressionVisitor};

use anyhow::Result;

/**
 * Add extension trait for convenient s-expression conversion
 */
pub trait ToSExpression {
    #[allow(dead_code)]
    fn to_sexpression(&self) -> Result<String>;

    #[allow(dead_code)]
    fn to_pretty_sexpression(&self) -> Result<String>;
}

impl ToSExpression for Statement {
    fn to_sexpression(&self) -> Result<String> {
        let mut visitor = SExpressionVisitor::new();
        visitor.output.clear();
        self.accept(&mut visitor)?;
        Ok(visitor.output)
    }

    fn to_pretty_sexpression(&self) -> Result<String> {
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

impl ToSExpression for Expression {
    fn to_sexpression(&self) -> Result<String> {
        let mut visitor = SExpressionVisitor::new();
        visitor.output.clear();
        self.accept(&mut visitor)?;
        Ok(visitor.output)
    }

    fn to_pretty_sexpression(&self) -> Result<String> {
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
