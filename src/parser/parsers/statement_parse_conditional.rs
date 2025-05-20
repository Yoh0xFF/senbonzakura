use crate::ast::{Statement, StatementRef};
use crate::lexer::TokenType;
use crate::parser::parsers::internal_util::{eat_token, is_next_token_of_type};
use crate::parser::parsers::parse_root_expression;
use crate::parser::parsers::statement_parse_block::parse_statement;
use crate::parser::Parser;

///
/// ConditionalStatement
///  : if '(' Expression ')' Statement [else Statement]
///
pub(super) fn parse_if_statement(parser: &mut Parser) -> StatementRef {
    eat_token(parser, TokenType::IfKeyword);

    eat_token(parser, TokenType::OpeningParenthesis);
    let condition = parse_root_expression(parser);
    eat_token(parser, TokenType::ClosingParenthesis);

    let consequent = parse_statement(parser);

    let alternative = if is_next_token_of_type(parser, TokenType::ElseKeyword) {
        eat_token(parser, TokenType::ElseKeyword);
        Some(parse_statement(parser))
    } else {
        None
    };

    Box::new(Statement::If {
        condition,
        consequent,
        alternative,
    })
}
