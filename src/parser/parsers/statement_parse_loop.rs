use crate::ast::{Statement, StatementRef};
use crate::lexer::TokenType;
use crate::parser::parsers::parse_root_expression;
use crate::parser::parsers::statement_parse_block::parse_statement;
use crate::parser::parsers::statement_parse_empty_and_expression::parse_expression_statement;
use crate::parser::parsers::statement_parse_variable_declaration::parse_variable_declaration_statement;
use crate::parser::Parser;

///
/// WhileStatement
///  : while '(' Expression ')' Statement ';'
///  ;
///
pub(super) fn parse_while_statement(parser: &mut Parser) -> StatementRef {
    parser.eat_token(TokenType::WhileKeyword);

    parser.eat_token(TokenType::OpeningParenthesis);
    let condition = parse_root_expression(parser);
    parser.eat_token(TokenType::ClosingParenthesis);

    let body = parse_statement(parser);

    Box::new(Statement::While { condition, body })
}

///
/// DoWhileStatement
///  : do Statement while '(' Expression ')' ';'
///  ;
///
pub(super) fn parse_do_while_statement(parser: &mut Parser) -> StatementRef {
    parser.eat_token(TokenType::DoKeyword);

    let body = parse_statement(parser);

    parser.eat_token(TokenType::WhileKeyword);

    parser.eat_token(TokenType::OpeningParenthesis);
    let condition = parse_root_expression(parser);
    parser.eat_token(TokenType::ClosingParenthesis);

    parser.eat_token(TokenType::StatementEnd);

    Box::new(Statement::DoWhile { body, condition })
}

///
/// ForStatement
///  : for '(' [InitExpression] ';' [Expression] ';' [Expression] ')' Statement
///  ;
///
pub(super) fn parse_for_statement(parser: &mut Parser) -> StatementRef {
    parser.eat_token(TokenType::ForKeyword);
    parser.eat_token(TokenType::OpeningParenthesis);

    let initializer = if parser.is_next_token_of_type(TokenType::StatementEnd) {
        None
    } else {
        Some(parse_for_statement_init_statement(parser))
    };
    parser.eat_token(TokenType::StatementEnd);

    let condition = if parser.is_next_token_of_type(TokenType::StatementEnd) {
        None
    } else {
        Some(parse_root_expression(parser))
    };
    parser.eat_token(TokenType::StatementEnd);

    let increment = if parser.is_next_token_of_type(TokenType::ClosingParenthesis) {
        None
    } else {
        Some(parse_root_expression(parser))
    };
    parser.eat_token(TokenType::ClosingParenthesis);

    let body = parse_statement(parser);

    Box::new(Statement::For {
        initializer,
        condition,
        increment,
        body,
    })
}

///
/// ForStatementInit
///  : VariableDeclarationStatement
///  | Expression
///  ;
///
pub(super) fn parse_for_statement_init_statement(parser: &mut Parser) -> StatementRef {
    if parser.is_next_token_of_type(TokenType::LetKeyword) {
        return parse_variable_declaration_statement(parser, false);
    }
    parse_expression_statement(parser, false)
}
