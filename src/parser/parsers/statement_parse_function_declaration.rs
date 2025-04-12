use crate::ast::{ExpressionList, Statement, StatementRef};
use crate::lexer::TokenType;
use crate::parser::parsers::expression_parse_primary::identifier_expression;
use crate::parser::parsers::root_expression;
use crate::parser::parsers::statement_parse_block::block_statement;
use crate::parser::parsers::utils::{eat, is_token};
use crate::parser::Parser;

/**
 * FunctionDeclaration
 *  : def '(' [FormalParameterList] ')' BlockStatement
 */
pub(super) fn parse_function_declaration_statement(parser: &mut Parser) -> StatementRef {
    eat(parser, TokenType::DefKeyword);
    let name = identifier_expression(parser);

    eat(parser, TokenType::OpeningParenthesis);
    let parameters = if is_token(parser, TokenType::ClosingParenthesis) {
        vec![]
    } else {
        parse_formal_parameter_list(parser)
    };
    eat(parser, TokenType::ClosingParenthesis);

    let body = block_statement(parser);

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
pub(super) fn parse_formal_parameter_list(parser: &mut Parser) -> ExpressionList {
    let mut params = vec![];

    loop {
        params.push(*identifier_expression(parser));

        if is_token(parser, TokenType::Comma) {
            eat(parser, TokenType::Comma);
            break;
        } else {
            break;
        }
    }

    params
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
        Some(root_expression(parser))
    };
    eat(parser, TokenType::StatementEnd);
    
    Box::new(Statement::Return { argument })
}
