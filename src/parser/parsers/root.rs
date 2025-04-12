use crate::ast::{ExpressionRef, StatementRef};
use crate::parser::parsers::expression_parse_variable_initialization_and_assignment::assignment_expression;
use crate::parser::parsers::statement_parse_block::program;
use crate::parser::Parser;

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
