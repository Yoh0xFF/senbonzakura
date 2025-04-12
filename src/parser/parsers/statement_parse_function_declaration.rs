use crate::ast::{ExpressionList, Statement, StatementRef};
use crate::lexer::TokenType;
use crate::parser::parsers::expression_parse_primary::parse_identifier_expression;
use crate::parser::parsers::parse_root_expression;
use crate::parser::parsers::statement_parse_block::parse_block_statement;
use crate::parser::parsers::utils::{eat, is_token};
use crate::parser::Parser;

/**
 * FunctionDeclaration
 *  : def '(' [FormalParameterList] ')' BlockStatement
 */
pub(super) fn parse_function_declaration_statement(parser: &mut Parser) -> StatementRef {
    eat(parser, TokenType::DefKeyword);
    let name = parse_identifier_expression(parser);

    eat(parser, TokenType::OpeningParenthesis);
    let parameters = if is_token(parser, TokenType::ClosingParenthesis) {
        vec![]
    } else {
        parse_formal_parameter_list_expression(parser)
    };
    eat(parser, TokenType::ClosingParenthesis);

    let body = parse_block_statement(parser);

    Box::new(Statement::FunctionDeclaration {
        name,
        parameters,
        body,
    })
}

/**
 * FormalParameterList
 *  : IdentifierExpression
 *  | FormalParameterList ',' IdentifierExpression
 *  ;
 */
pub(super) fn parse_formal_parameter_list_expression(parser: &mut Parser) -> ExpressionList {
    let mut parameters = vec![];

    // Parse first parameter
    parameters.push(*parse_identifier_expression(parser));

    // Parse additional parameters if any
    while is_token(parser, TokenType::Comma) {
        eat(parser, TokenType::Comma);
        parameters.push(*parse_identifier_expression(parser));
    }

    parameters
}

/**
 * ReturnStatement
 *  : return [Expression] 'l'
 */
pub(super) fn parse_return_statement(parser: &mut Parser) -> StatementRef {
    eat(parser, TokenType::ReturnKeyword);
    let argument = if is_token(parser, TokenType::StatementEnd) {
        None
    } else {
        Some(parse_root_expression(parser))
    };
    eat(parser, TokenType::StatementEnd);

    Box::new(Statement::Return { argument })
}
