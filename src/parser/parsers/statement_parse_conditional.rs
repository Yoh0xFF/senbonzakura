use crate::ast::{Statement, StatementRef};
use crate::lexer::TokenType;
use crate::parser::parsers::parse_root_expression;
use crate::parser::parsers::statement_parse_block::parse_statement;
use crate::parser::{Parser, ParserResult};

///
/// ConditionalStatement
///  : if '(' Expression ')' Statement [else Statement]
///
pub(super) fn parse_if_statement(parser: &mut Parser) -> ParserResult<StatementRef> {
    parser.eat_token(TokenType::IfKeyword)?;

    parser.eat_token(TokenType::OpeningParenthesis)?;
    let condition = parse_root_expression(parser)?;
    parser.eat_token(TokenType::ClosingParenthesis)?;

    let consequent = parse_statement(parser)?;

    let alternative = if parser.is_next_token_of_type(TokenType::ElseKeyword) {
        parser.eat_token(TokenType::ElseKeyword)?;
        let statement = parse_statement(parser)?;
        Some(statement)
    } else {
        None
    };

    Ok(Box::new(Statement::If {
        condition,
        consequent,
        alternative,
    }))
}
