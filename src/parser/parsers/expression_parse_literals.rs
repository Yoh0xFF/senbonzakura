use crate::ast::{Expression, ExpressionRef};
use crate::lexer::TokenType;
use crate::parser::Parser;
use crate::parser::parsers::eat;

/**
 * Literal
 *  : BooleanLiteral
 *  : NilLiteral
 *  : NumericLiteral
 *  | StringLiteral
 *  ;
 */
pub fn literal_expression(parser: &mut Parser) -> ExpressionRef {
    match parser.lookahead.token_type {
        TokenType::Boolean => boolean_literal_expression(parser),
        TokenType::Nil => nil_literal_expression(parser),
        TokenType::Number => numeric_literal_expression(parser),
        TokenType::String => string_literal_expression(parser),
        _ => panic!("Literal: unexpected literal production"),
    }
}

/**
 * BooleanLiteral
 *  : BOOLEAN
 *  ;
 */
pub fn boolean_literal_expression(parser: &mut Parser) -> ExpressionRef {
    let token = eat(parser, TokenType::Boolean);
    let token_value = &parser.source[token.i..token.j];
    let bool_value = token_value == "true";

    Box::new(Expression::BooleanLiteral(bool_value))
}

/**
 * NilLiteral
 *  : NIL
 *  ;
 */
pub fn nil_literal_expression(parser: &mut Parser) -> ExpressionRef {
    eat(parser, TokenType::Nil);

    Box::new(Expression::NilLiteral)
}

/**
 * NumericLiteral
 *  : NUMBER
 *  ;
 */
pub fn numeric_literal_expression(parser: &mut Parser) -> ExpressionRef {
    let token = eat(parser, TokenType::Number);
    let token_value = &parser.source[token.i..token.j];
    let token_value = token_value.trim().parse().unwrap();

    Box::new(Expression::NumericLiteral(token_value))
}

/**
 * StringLiteral
 *  : STRING
 *  ;
 */
pub fn string_literal_expression(parser: &mut Parser) -> ExpressionRef {
    let token = eat(parser, TokenType::String);
    let token_value = &parser.source[token.i + 1..token.j - 1];

    Box::new(Expression::StringLiteral(String::from(token_value)))
}
