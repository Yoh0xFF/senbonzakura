use crate::ast::{ExpressionRef, StatementRef};
use crate::parser::Parser;
use crate::parser::parsers::{assignment_expression, program};

/**
 * Parses a string into an AST
 */
pub fn root_statement(parser: &mut Parser) -> StatementRef {
    program(parser)
}

/**
 * Expression
 *  : Literal
 *  ;
 */
pub fn root_expression(parser: &mut Parser) -> ExpressionRef {
    assignment_expression(parser)
}
