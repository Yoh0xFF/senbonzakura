use crate::ast::{Expression, ExpressionRef, UnaryOperator};
use crate::lexer::TokenType;
use crate::parser::Parser;

use super::expression_parse_left_hand_side::parse_left_hand_side_expression;

///
/// UnaryExpression
///  : LeftHandSideExpression
///  | ADDITIVE_OPERATOR UnaryExpression
///  | LOGICAL_NOT_OPERATOR UnaryExpression
///  ;
///
pub(super) fn parse_unary_expression(parser: &mut Parser) -> ExpressionRef {
    let mut operator: Option<UnaryOperator> = None;

    if parser
        .is_next_token_any_of_type(&[TokenType::AdditiveOperator, TokenType::LogicalNotOperator])
    {
        let token =
            parser.eat_any_of_token(&[TokenType::AdditiveOperator, TokenType::LogicalNotOperator]);
        let token_value = &parser.source[token.i..token.j];

        operator = Some(match token_value {
            "+" => UnaryOperator::Plus,
            "-" => UnaryOperator::Minus,
            "!" => UnaryOperator::Not,
            _ => panic!("Unknown unary operator {}", token_value),
        });
    }

    if operator.is_some() {
        return Box::new(Expression::Unary {
            operator: operator.unwrap(),
            right: parse_unary_expression(parser),
        });
    }

    parse_left_hand_side_expression(parser)
}
