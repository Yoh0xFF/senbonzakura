use crate::ast::{Expression, ExpressionRef, UnaryOperator};
use crate::lexer::TokenType;
use crate::parser::{Parser, ParserError, ParserResult};

use super::expression_parse_left_hand_side::parse_left_hand_side_expression;

///
/// UnaryExpression
///  : LeftHandSideExpression
///  | ADDITIVE_OPERATOR UnaryExpression
///  | LOGICAL_NOT_OPERATOR UnaryExpression
///  ;
///
pub(super) fn parse_unary_expression(parser: &mut Parser) -> ParserResult<ExpressionRef> {
    if parser.is_next_token_any_of_type(&[
        TokenType::AdditivePlusOperator,
        TokenType::AdditiveMinusOperator,
        TokenType::LogicalNotOperator,
    ]) {
        let token = parser.eat_any_of_token(&[
            TokenType::AdditivePlusOperator,
            TokenType::AdditiveMinusOperator,
            TokenType::LogicalNotOperator,
        ])?;

        let operator = match token.token_type {
            TokenType::AdditivePlusOperator => UnaryOperator::Plus,
            TokenType::AdditiveMinusOperator => UnaryOperator::Minus,
            TokenType::LogicalNotOperator => UnaryOperator::Not,
            _ => {
                return Err(ParserError::ParserError {
                    message: format!("Unknown unary operator {}", token.token_type),
                })
            }
        };

        let right = parse_unary_expression(parser)?;
        return Ok(Box::new(Expression::Unary { operator, right }));
    }

    parse_left_hand_side_expression(parser)
}
