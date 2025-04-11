use crate::{
    ast::{AstVisitor, Expression},
    Statement,
};
use anyhow::Result;
use std::fmt::Write;

use super::{visit_expressions::visit_expression, visit_statements::visit_statement};

/**
 * S-Expression Visitor Configuration
 */
#[derive(Debug, Clone, Copy)]
pub struct SExpressionConfig {
    pub pretty: bool,
    pub indent_size: usize,
}

impl Default for SExpressionConfig {
    fn default() -> Self {
        Self {
            pretty: false,
            indent_size: 2,
        }
    }
}

/**
 * S-Expression Visitor for AST nodes
 */
pub struct SExpressionVisitor {
    pub(super) config: SExpressionConfig,
    pub(super) current_indent: usize,
    pub(super) output: String,
}

impl SExpressionVisitor {
    pub fn new() -> Self {
        Self::with_config(SExpressionConfig::default())
    }

    pub fn with_config(config: SExpressionConfig) -> Self {
        SExpressionVisitor {
            config,
            output: String::with_capacity(1024),
            current_indent: 0,
        }
    }

    pub(super) fn write_indent(&mut self) -> Result<()> {
        if self.config.pretty && self.current_indent > 0 {
            write!(
                self.output,
                "{}",
                " ".repeat(self.current_indent * self.config.indent_size)
            )?;
        }
        Ok(())
    }

    pub(super) fn begin_expr(&mut self, name: &str) -> Result<()> {
        self.write_indent()?;
        write!(self.output, "({}", name)?;

        if self.config.pretty {
            self.current_indent += 1;
        }

        Ok(())
    }

    pub(super) fn end_expr(&mut self) -> Result<()> {
        if self.config.pretty {
            self.current_indent -= 1;
        }

        write!(self.output, ")")?;
        Ok(())
    }

    pub(super) fn write_space_or_newline(&mut self) -> Result<()> {
        if self.config.pretty {
            writeln!(self.output)?;
        } else {
            write!(self.output, " ")?;
        }
        Ok(())
    }
}

impl AstVisitor for SExpressionVisitor {
    type Output = ();

    fn visit_statement(&mut self, statement: &Statement) -> Result<Self::Output> {
        visit_statement(self, statement)
    }

    fn visit_expression(&mut self, expression: &Expression) -> Result<Self::Output> {
        visit_expression(self, expression)
    }
}
