use crate::ast::{Expression, ExpressionRef};
use crate::lexer::TokenType;
use crate::parser::parsers::expression_parse_literals::literal_expression;
use crate::parser::parsers::root::root_expression;
use crate::parser::parsers::utils::{eat, is_literal_token};
use crate::parser::Parser;

/**
 * LeftHandSideExpression
 *  : PrimaryExpression
 *  ;
 */
pub(super) fn left_hand_side_expression(parser: &mut Parser) -> ExpressionRef {
    primary_expression(parser)
}

/**
 * PrimaryExpression
 *  : LiteralExpression
 *  | GroupExpression
 *  | IdentifierExpression
 *  ;
 */
pub(super) fn primary_expression(parser: &mut Parser) -> ExpressionRef {
    if is_literal_token(parser) {
        return literal_expression(parser);
    }

    match parser.lookahead.token_type {
        TokenType::OpeningParenthesis => group_expression(parser),
        TokenType::Identifier => identifier_expression(parser),
        _ => left_hand_side_expression(parser),
    }
}

/**
 * GroupExpression
 *  : '(' Expression ')'
 *  ;
 */
pub(super) fn group_expression(parser: &mut Parser) -> ExpressionRef {
    eat(parser, TokenType::OpeningParenthesis);
    let expression_ref = root_expression(parser);
    eat(parser, TokenType::ClosingParenthesis);

    expression_ref
}

/**
 * IdentifierExpression
 *  : IDENTIFIER
 *  ;
 */
pub(super) fn identifier_expression(parser: &mut Parser) -> ExpressionRef {
    let identifier_token = eat(parser, TokenType::Identifier);
    let identifier_value = &parser.source[identifier_token.i..identifier_token.j];

    Box::new(Expression::Identifier(String::from(identifier_value)))
}
