use crate::ast::{ParameterList, Statement, StatementRef, Type};
use crate::lexer::TokenType;
use crate::parser::parsers::expression_parse_primary::parse_identifier_expression;
use crate::parser::parsers::parse_root_expression;
use crate::parser::parsers::statement_parse_block::parse_block_statement;
use crate::parser::parsers::internal_util::{eat_token, is_next_token_of_type};
use crate::parser::Parser;

use super::type_parse_annotations::parse_type;

///
/// FunctionDeclaration
///  : def '(' [FormalParameterList] ')' [':' Type] BlockStatement
///
pub(super) fn parse_function_declaration_statement(parser: &mut Parser) -> StatementRef {
    eat_token(parser, TokenType::DefKeyword);
    let name = parse_identifier_expression(parser);

    eat_token(parser, TokenType::OpeningParenthesis);
    let parameters = if is_next_token_of_type(parser, TokenType::ClosingParenthesis) {
        vec![]
    } else {
        parse_formal_parameter_list_expression(parser)
    };
    eat_token(parser, TokenType::ClosingParenthesis);

    // Parse return type
    let return_type = if is_next_token_of_type(parser, TokenType::Colon) {
        eat_token(parser, TokenType::Colon);
        parse_type(parser)
    } else {
        Type::Void
    };

    let body = parse_block_statement(parser);

    Box::new(Statement::FunctionDeclaration {
        name,
        parameters,
        return_type,
        body,
    })
}

///
/// FormalParameterList
///  : IdentifierExpression ':' Type
///  | FormalParameterList ',' IdentifierExpression ':' Type
///  ;
///
pub(super) fn parse_formal_parameter_list_expression(parser: &mut Parser) -> ParameterList {
    let mut parameters = vec![];

    // Parse first parameter
    let param_name = *parse_identifier_expression(parser);
    eat_token(parser, TokenType::Colon);
    let param_type = parse_type(parser);
    parameters.push((param_name, param_type));

    // Parse additional parameters if any
    while is_next_token_of_type(parser, TokenType::Comma) {
        eat_token(parser, TokenType::Comma);
        let param_name = *parse_identifier_expression(parser);
        eat_token(parser, TokenType::Colon);
        let param_type = parse_type(parser);
        parameters.push((param_name, param_type));
    }

    parameters
}

///
/// ReturnStatement
///  : return [Expression] 'l'
///
pub(super) fn parse_return_statement(parser: &mut Parser) -> StatementRef {
    eat_token(parser, TokenType::ReturnKeyword);
    let argument = if is_next_token_of_type(parser, TokenType::StatementEnd) {
        None
    } else {
        Some(parse_root_expression(parser))
    };
    eat_token(parser, TokenType::StatementEnd);

    Box::new(Statement::Return { argument })
}
