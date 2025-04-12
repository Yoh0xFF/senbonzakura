use crate::ast::{Expression, ExpressionRef};
use crate::lexer::TokenType;
use crate::parser::parsers::root::root_expression;
use crate::parser::parsers::{eat, is_literal_token, literal_expression};
use crate::parser::Parser;

/**
 * LeftHandSideExpression
 *  : PrimaryExpression
 *  ;
 */
pub fn left_hand_side_expression(parser: &mut Parser) -> ExpressionRef {
    primary_expression(parser)
}

/**
 * PrimaryExpression
 *  : LiteralExpression
 *  | GroupExpression
 *  | IdentifierExpression
 *  ;
 */
pub fn primary_expression(parser: &mut Parser) -> ExpressionRef {
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
pub fn group_expression(parser: &mut Parser) -> ExpressionRef {
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
pub fn identifier_expression(parser: &mut Parser) -> ExpressionRef {
    let identifier_token = eat(parser, TokenType::Identifier);
    let identifier_value = &parser.source[identifier_token.i..identifier_token.j];

    Box::new(Expression::Identifier(String::from(identifier_value)))
}
