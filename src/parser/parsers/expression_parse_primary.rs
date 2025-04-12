use crate::ast::{Expression, ExpressionRef};
use crate::lexer::TokenType;
use crate::parser::parsers::expression_parse_literals::parse_literal_expression;
use crate::parser::parsers::root::parse_root_expression;
use crate::parser::parsers::utils::{eat, is_literal_token};
use crate::parser::Parser;

///
/// LeftHandSideExpression
///  : PrimaryExpression
///  ;
///
pub(super) fn parse_left_hand_side_expression(parser: &mut Parser) -> ExpressionRef {
    parse_primary_expression(parser)
}

///
/// PrimaryExpression
///  : LiteralExpression
///  | GroupExpression
///  | IdentifierExpression
///  ;
///
pub(super) fn parse_primary_expression(parser: &mut Parser) -> ExpressionRef {
    if is_literal_token(parser) {
        return parse_literal_expression(parser);
    }

    match parser.lookahead.token_type {
        TokenType::OpeningParenthesis => parse_group_expression(parser),
        TokenType::Identifier => parse_identifier_expression(parser),
        _ => parse_left_hand_side_expression(parser),
    }
}

///
/// GroupExpression
///  : '(' Expression ')'
///  ;
///
pub(super) fn parse_group_expression(parser: &mut Parser) -> ExpressionRef {
    eat(parser, TokenType::OpeningParenthesis);
    let expression_ref = parse_root_expression(parser);
    eat(parser, TokenType::ClosingParenthesis);

    expression_ref
}

///
/// IdentifierExpression
///  : IDENTIFIER
///  ;
///
pub(super) fn parse_identifier_expression(parser: &mut Parser) -> ExpressionRef {
    let identifier_token = eat(parser, TokenType::Identifier);
    let identifier_value = &parser.source[identifier_token.i..identifier_token.j];

    Box::new(Expression::Identifier(String::from(identifier_value)))
}
