use crate::ast::{Statement, StatementRef};
use crate::lexer::TokenType;
use crate::parser::Parser;
use crate::parser::parsers::{eat, expression};

/**
 * EmptyStatement
 *  : ';'
 *  ;
 */
pub fn empty_statement(parser: &mut Parser) -> StatementRef {
    eat(parser, TokenType::StatementEnd);

    Box::new(Statement::Empty)
}

/**
 * ExpressionStatement
 *  : Expression ';'
 *  ;
 */
pub fn expression_statement(parser: &mut Parser, consume_statement_end: bool) -> StatementRef {
    let expression = expression(parser);

    if consume_statement_end {
        eat(parser, TokenType::StatementEnd);
    }

    Box::new(Statement::Expression { expression })
}
