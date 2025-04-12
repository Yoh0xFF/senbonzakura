use crate::ast::{ExpressionRef, StatementRef};
use crate::parser::parsers::expression_parse_variable_initialization_and_assignment::parse_assignment_expression;
use crate::parser::parsers::statement_parse_block::parse_program_statement;
use crate::parser::Parser;

/**
 * Parses a string into an AST
 */
pub fn parse_root_statement(parser: &mut Parser) -> StatementRef {
    parse_program_statement(parser)
}

/**
 * Expression
 *  : Literal
 *  ;
 */
pub fn parse_root_expression(parser: &mut Parser) -> ExpressionRef {
    parse_assignment_expression(parser)
}
