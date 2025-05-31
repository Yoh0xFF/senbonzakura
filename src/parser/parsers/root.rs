use crate::ast::{ExpressionRef, StatementRef};
use crate::parser::parsers::expression_parse_assignment::parse_assignment_expression;
use crate::parser::parsers::statement_parse_block::parse_program_statement;
use crate::parser::{Parser, ParserResult};

///
/// Parses a string into an AST
///
pub fn parse_root_statement(parser: &mut Parser) -> ParserResult<StatementRef> {
    parse_program_statement(parser)
}

///
/// Expression
///  : AssignmentExpression
///  ;
///
pub fn parse_root_expression(parser: &mut Parser) -> ParserResult<ExpressionRef> {
    parse_assignment_expression(parser)
}
