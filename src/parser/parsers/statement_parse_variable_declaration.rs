use super::expression_parse_assignment::parse_assignment_expression;
use super::expression_parse_primary::parse_identifier_expression;
use super::type_parse_annotations::parse_type;
use crate::ast::{Expression, ExpressionRef, Statement, StatementRef};
use crate::lexer::TokenType;
use crate::parser::{Parser, ParserResult};

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
) -> ParserResult<StatementRef> {
    let mut variables: Vec<Expression> = vec![];

    parser.eat_token(TokenType::LetKeyword)?;
    loop {
        let variable_expression = parse_variable_expression(parser)?;
        variables.push(*variable_expression);

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
/// VariableInitializationExpression
///  : Identifier ':' Type ['=' Initializer]
///  ;
///
pub(super) fn parse_variable_expression(parser: &mut Parser) -> ParserResult<ExpressionRef> {
    let identifier = parse_identifier_expression(parser)?;

    // Require type annotation
    parser.eat_token(TokenType::Colon)?;
    let type_annotation = parse_type(parser)?;

    let initializer: Option<ExpressionRef> =
        if parser.is_next_token_any_of_type(&[TokenType::StatementEnd, TokenType::Comma]) {
            None
        } else {
            parser.eat_token(TokenType::SimpleAssignmentOperator)?;
            let initializer = parse_assignment_expression(parser)?;
            Some(initializer)
        };

    Ok(Box::new(Expression::Variable {
        identifier,
        type_annotation,
        initializer,
    }))
}
