use crate::ast::{Parameter, ParameterList, Statement, StatementRef, Type};
use crate::lexer::TokenType;
use crate::parser::parsers::expression_parse_primary::parse_identifier_expression;
use crate::parser::parsers::parse_root_expression;
use crate::parser::parsers::statement_parse_block::parse_block_statement;
use crate::parser::{Parser, ParserResult};

use super::type_parse_annotations::parse_type;

///
/// FunctionDeclaration
///  : def '(' [FormalParameterList] ')' [':' Type] BlockStatement
///
pub(super) fn parse_function_declaration_statement(
    parser: &mut Parser,
) -> ParserResult<StatementRef> {
    parser.eat_token(TokenType::DefKeyword)?;
    let name = parse_identifier_expression(parser)?;

    parser.eat_token(TokenType::OpeningParenthesis)?;
    let parameters = if parser.is_next_token_of_type(TokenType::ClosingParenthesis) {
        vec![]
    } else {
        parse_formal_parameter_list_expression(parser)?
    };
    parser.eat_token(TokenType::ClosingParenthesis)?;

    // Parse return type
    let return_type = if parser.is_next_token_of_type(TokenType::Colon) {
        parser.eat_token(TokenType::Colon)?;
        parse_type(parser)?
    } else {
        Type::Void
    };

    let body = parse_block_statement(parser)?;

    Ok(Box::new(Statement::FunctionDeclaration {
        name,
        parameters,
        return_type,
        body,
    }))
}

///
/// FormalParameterList
///  : FormalParameter
///  | FormalParameterList ',' FormalParameter
///  ;
///
pub(super) fn parse_formal_parameter_list_expression(
    parser: &mut Parser,
) -> ParserResult<ParameterList> {
    let mut parameters = vec![];

    // Parse first parameter
    parameters.push(parse_formal_parameter(parser)?);

    // Parse additional parameters if any
    while parser.is_next_token_of_type(TokenType::Comma) {
        parser.eat_token(TokenType::Comma)?;
        parameters.push(parse_formal_parameter(parser)?);
    }

    Ok(parameters)
}

///
/// FormalParameter
///  : IDENTIFIER ':' Type
///  ;
///
fn parse_formal_parameter(parser: &mut Parser) -> ParserResult<Parameter> {
    let name_token = parser.eat_token(TokenType::Identifier)?;
    let name = String::from(name_token.text(parser.source));
    parser.eat_token(TokenType::Colon)?;
    let type_annotation = parse_type(parser)?;

    Ok(Parameter {
        name,
        type_annotation,
    })
}

///
/// ReturnStatement
///  : return [Expression] 'l'
///
pub(super) fn parse_return_statement(parser: &mut Parser) -> ParserResult<StatementRef> {
    parser.eat_token(TokenType::ReturnKeyword)?;
    let argument = if parser.is_next_token_of_type(TokenType::StatementEnd) {
        None
    } else {
        let expression = parse_root_expression(parser)?;
        Some(expression)
    };
    parser.eat_token(TokenType::StatementEnd)?;

    Ok(Box::new(Statement::Return { argument }))
}
