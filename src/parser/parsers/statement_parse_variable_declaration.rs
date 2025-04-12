use crate::ast::{Expression, Statement, StatementRef};
use crate::lexer::TokenType;
use crate::parser::Parser;
use crate::parser::parsers::{eat, is_token, variable_initialization_expression};

/**
 * VariableDeclarationStatement
 *  : 'let' VariableInitializationList ';'
 *
 * VariableInitializationList
 *  : VariableInitialization
 *  | VariableInitializationList ',' VariableInitialization
 *  ;
 */
pub fn variable_declaration_statement(parser: &mut Parser, consume_statement_end: bool) -> StatementRef {
    let mut variables: Vec<Expression> = vec![];

    eat(parser, TokenType::LetKeyword);
    loop {
        variables.push(*variable_initialization_expression(parser));

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