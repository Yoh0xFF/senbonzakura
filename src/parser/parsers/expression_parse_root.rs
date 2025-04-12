use crate::ast::ExpressionRef;
use crate::parser::Parser;
use crate::parser::parsers::assignment_expression;

/**
 * Expression
 *  : Literal
 *  ;
 */
pub fn expression(parser: &mut Parser) -> ExpressionRef {
    assignment_expression(parser)
}
