use crate::ast::{Expression, ExpressionRef};
use crate::lexer::TokenType;
use crate::parser::Parser;

///
/// Literal
///  : BooleanLiteral
///  : NilLiteral
///  : NumericLiteral
///  | StringLiteral
///  ;
///
pub(super) fn parse_literal_expression(parser: &mut Parser) -> ExpressionRef {
    match parser.lookahead.token_type {
        TokenType::Boolean => parse_boolean_literal_expression(parser),
        TokenType::Nil => parse_nil_literal_expression(parser),
        TokenType::Number => parse_numeric_literal_expression(parser),
        TokenType::String => parse_string_literal_expression(parser),
        _ => panic!("Literal: unexpected literal production"),
    }
}

///
/// BooleanLiteral
///  : BOOLEAN
///  ;
///
pub(super) fn parse_boolean_literal_expression(parser: &mut Parser) -> ExpressionRef {
    let token = parser.eat_token(TokenType::Boolean);
    let token_value = &parser.source[token.i..token.j];
    let bool_value = token_value == "true";

    Box::new(Expression::BooleanLiteral { value: bool_value })
}

///
/// NilLiteral
///  : NIL
///  ;
///
pub(super) fn parse_nil_literal_expression(parser: &mut Parser) -> ExpressionRef {
    parser.eat_token(TokenType::Nil);

    Box::new(Expression::NilLiteral)
}

///
/// NumericLiteral
///  : NUMBER
///  ;
///
pub(super) fn parse_numeric_literal_expression(parser: &mut Parser) -> ExpressionRef {
    let token = parser.eat_token(TokenType::Number);
    let token_value = &parser.source[token.i..token.j];
    let token_value = token_value.trim().parse().unwrap();

    Box::new(Expression::NumericLiteral { value: token_value })
}

///
/// StringLiteral
///  : STRING
///  ;
///
pub(super) fn parse_string_literal_expression(parser: &mut Parser) -> ExpressionRef {
    let token = parser.eat_token(TokenType::String);
    let token_value = &parser.source[token.i + 1..token.j - 1];

    Box::new(Expression::StringLiteral {
        value: String::from(token_value),
    })
}
