use super::expression_parse_left_hand_side::{
    parse_arguments, parse_left_hand_side_expression, parse_member_expression,
};
use crate::ast::{Expression, ExpressionRef};
use crate::lexer::TokenType;
use crate::parser::parsers::expression_parse_literals::parse_literal_expression;
use crate::parser::parsers::root::parse_root_expression;
use crate::parser::parsers::utils::{eat, is_literal_token};
use crate::parser::Parser;

///
/// PrimaryExpression
///  : LiteralExpression
///  | GroupExpression
///  | IdentifierExpression
///  | ThisExpression
///  ;
///
pub(super) fn parse_primary_expression(parser: &mut Parser) -> ExpressionRef {
    if is_literal_token(parser) {
        return parse_literal_expression(parser);
    }

    match parser.lookahead.token_type {
        TokenType::OpeningParenthesis => parse_group_expression(parser),
        TokenType::Identifier => parse_identifier_expression(parser),
        TokenType::ThisKeyword => parse_this_expression(parser),
        TokenType::NewKeyword => parse_new_expression(parser),
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

///
/// ThisExpression
///  : this
///  ;
///
pub(super) fn parse_this_expression(parser: &mut Parser) -> ExpressionRef {
    eat(parser, TokenType::ThisKeyword);
    Box::new(Expression::This {})
}

///
/// NewExpression
///  : new MemberExpression Arguments
///  ;
///
pub(super) fn parse_new_expression(parser: &mut Parser) -> ExpressionRef {
    eat(parser, TokenType::NewKeyword);

    let callee = parse_member_expression(parser);

    let arguments = parse_arguments(parser);

    Box::new(Expression::New { callee, arguments })
}
