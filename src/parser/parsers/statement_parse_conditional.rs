use crate::ast::{Statement, StatementRef};
use crate::lexer::TokenType;
use crate::parser::parsers::root_expression;
use crate::parser::parsers::statement_parse_block::statement;
use crate::parser::parsers::utils::{eat, is_token};
use crate::parser::Parser;

/**
 * ConditionalStatement
 *  : if '(' Expression ')' Statement [else Statement]
 */
pub(super) fn if_statement(parser: &mut Parser) -> StatementRef {
    eat(parser, TokenType::IfKeyword);

    eat(parser, TokenType::OpeningParenthesis);
    let condition = root_expression(parser);
    eat(parser, TokenType::ClosingParenthesis);

    let consequent = statement(parser);

    let alternative = if is_token(parser, TokenType::ElseKeyword) {
        eat(parser, TokenType::ElseKeyword);
        Some(statement(parser))
    } else {
        None
    };

    Box::new(Statement::Conditional {
        condition,
        consequent,
        alternative,
    })
}
