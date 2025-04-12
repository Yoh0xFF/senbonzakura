use crate::ast::{Statement, StatementRef};
use crate::lexer::TokenType;
use crate::parser::parsers::root_expression;
use crate::parser::parsers::utils::eat;
use crate::parser::Parser;

/**
 * EmptyStatement
 *  : ';'
 *  ;
 */
pub(super) fn empty_statement(parser: &mut Parser) -> StatementRef {
    eat(parser, TokenType::StatementEnd);

    Box::new(Statement::Empty)
}

/**
 * ExpressionStatement
 *  : Expression ';'
 *  ;
 */
pub(super) fn expression_statement(
    parser: &mut Parser,
    consume_statement_end: bool,
) -> StatementRef {
    let expression = root_expression(parser);

    if consume_statement_end {
        eat(parser, TokenType::StatementEnd);
    }

    Box::new(Statement::Expression { expression })
}
