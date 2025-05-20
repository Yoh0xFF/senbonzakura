use crate::ast::{Expression, ExpressionRef, Statement, StatementRef};
use crate::lexer::TokenType;
use crate::parser::parsers::internal_util::{eat, is_token};
use crate::parser::Parser;

use super::expression_parse_assignment::parse_assignment_expression;
use super::expression_parse_primary::parse_identifier_expression;
use super::internal_util::is_any_of_token;
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

    eat(parser, TokenType::LetKeyword);
    loop {
        variables.push(*parse_variable_expression(parser));

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

///
/// VariableInitializationExpression
///  : Identifier ':' Type ['=' Initializer]
///  ;
///
pub(super) fn parse_variable_expression(parser: &mut Parser) -> ExpressionRef {
    let identifier = parse_identifier_expression(parser);

    // Require type annotation
    eat(parser, TokenType::Colon);
    let type_annotation = parse_type(parser);

    let initializer: Option<ExpressionRef> =
        if is_any_of_token(parser, &[TokenType::StatementEnd, TokenType::Comma]) {
            None
        } else {
            eat(parser, TokenType::SimpleAssignmentOperator);
            let initializer = parse_assignment_expression(parser);
            Some(initializer)
        };

    Box::new(Expression::Variable {
        identifier,
        type_annotation,
        initializer,
    })
}
