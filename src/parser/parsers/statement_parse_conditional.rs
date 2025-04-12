use crate::ast::{Statement, StatementRef};
use crate::lexer::TokenType;
use crate::parser::Parser;
use crate::parser::parsers::{eat, expression, is_token, statement};

/**
 * ConditionalStatement
 *  : if '(' Expression ')' Statement [else Statement]
 */
pub fn if_statement(parser: &mut Parser) -> StatementRef {
    eat(parser, TokenType::IfKeyword);

    eat(parser, TokenType::OpeningParenthesis);
    let condition = expression(parser);
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
