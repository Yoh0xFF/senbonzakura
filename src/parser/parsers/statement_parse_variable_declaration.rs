use crate::ast::{Expression, Statement, StatementRef};
use crate::lexer::TokenType;
use crate::parser::parsers::expression_parse_variable_initialization_and_assignment::parse_variable_initialization_expression;
use crate::parser::parsers::utils::{eat, is_token};
use crate::parser::Parser;

///
/// VariableDeclarationStatement
///  : 'let' VariableInitializationList ';'
/// 
/// VariableInitializationList
///  : VariableInitialization
///  | VariableInitializationList ',' VariableInitialization
///  ;
///
pub(super) fn parse_variable_declaration_statement(
    parser: &mut Parser,
    consume_statement_end: bool,
) -> StatementRef {
    let mut variables: Vec<Expression> = vec![];

    eat(parser, TokenType::LetKeyword);
    loop {
        variables.push(*parse_variable_initialization_expression(parser));

        if !is_token(parser, TokenType::Comma) {
            break;
        }

        eat(parser, TokenType::Comma);
    }
    if consume_statement_end {
        eat(parser, TokenType::StatementEnd);
    }

    Box::new(Statement::VariableDeclaration { variables })
}
