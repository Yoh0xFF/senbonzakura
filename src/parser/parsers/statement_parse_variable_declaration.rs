use crate::ast::{Expression, ExpressionRef, Statement, StatementRef};
use crate::lexer::TokenType;
use crate::parser::parsers::internal_util::{eat_token, is_next_token_of_type};
use crate::parser::Parser;

use super::expression_parse_assignment::parse_assignment_expression;
use super::expression_parse_primary::parse_identifier_expression;
use super::internal_util::is_next_token_any_of_type;
use super::type_parse_annotations::parse_type;

///
/// VariableDeclarationStatement
///  : 'let' VariableList ';'
///
/// VariableList
///  : VariableExpression
///  | VariableList ',' VariableExpression
///  ;
///
pub(super) fn parse_variable_declaration_statement(
    parser: &mut Parser,
    consume_statement_end: bool,
) -> StatementRef {
    let mut variables: Vec<Expression> = vec![];

    eat_token(parser, TokenType::LetKeyword);
    loop {
        variables.push(*parse_variable_expression(parser));

        if !is_next_token_of_type(parser, TokenType::Comma) {
            break;
        }

        eat_token(parser, TokenType::Comma);
    }
    if consume_statement_end {
        eat_token(parser, TokenType::StatementEnd);
    }

    Box::new(Statement::VariableDeclaration { variables })
}

///
/// VariableInitializationExpression
///  : Identifier ':' Type ['=' Initializer]
///  ;
///
pub(super) fn parse_variable_expression(parser: &mut Parser) -> ExpressionRef {
    let identifier = parse_identifier_expression(parser);

    // Require type annotation
    eat_token(parser, TokenType::Colon);
    let type_annotation = parse_type(parser);

    let initializer: Option<ExpressionRef> =
        if is_next_token_any_of_type(parser, &[TokenType::StatementEnd, TokenType::Comma]) {
            None
        } else {
            eat_token(parser, TokenType::SimpleAssignmentOperator);
            let initializer = parse_assignment_expression(parser);
            Some(initializer)
        };

    Box::new(Expression::Variable {
        identifier,
        type_annotation,
        initializer,
    })
}
