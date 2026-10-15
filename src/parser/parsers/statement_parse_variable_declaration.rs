use super::expression_parse_assignment::parse_assignment_expression;
use super::type_parse_annotations::parse_type;
use crate::ast::{Expression, Statement, VariableDeclarator};
use crate::lexer::TokenType;
use crate::parser::{Parser, ParserResult};

///
/// VariableDeclarationStatement
///  : 'let' VariableList ';'
///
/// VariableList
///  : VariableDeclarator
///  | VariableList ',' VariableDeclarator
///  ;
///
pub(super) fn parse_variable_declaration_statement(
    parser: &mut Parser,
    consume_statement_end: bool,
) -> ParserResult<Box<Statement>> {
    let mut variables: Vec<VariableDeclarator> = vec![];

    parser.eat_token(TokenType::LetKeyword)?;
    loop {
        variables.push(parse_variable_declarator(parser)?);

        if !parser.is_next_token_of_type(TokenType::Comma) {
            break;
        }

        parser.eat_token(TokenType::Comma)?;
    }
    if consume_statement_end {
        parser.eat_token(TokenType::StatementEnd)?;
    }

    Ok(Box::new(Statement::VariableDeclaration { variables }))
}

///
/// VariableDeclarator
///  : IDENTIFIER ':' Type ['=' Initializer]
///  ;
///
pub(super) fn parse_variable_declarator(parser: &mut Parser) -> ParserResult<VariableDeclarator> {
    let name_token = parser.eat_token(TokenType::Identifier)?;
    let name = String::from(name_token.text(parser.source));

    // Require type annotation
    parser.eat_token(TokenType::Colon)?;
    let type_annotation = parse_type(parser)?;

    let initializer: Option<Box<Expression>> =
        if parser.is_next_token_any_of_type(&[TokenType::StatementEnd, TokenType::Comma]) {
            None
        } else {
            parser.eat_token(TokenType::SimpleAssignmentOperator)?;
            let initializer = parse_assignment_expression(parser)?;
            Some(initializer)
        };

    Ok(VariableDeclarator {
        name,
        type_annotation,
        initializer,
    })
}
