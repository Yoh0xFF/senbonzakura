use crate::ast::{Expression, ExpressionRef, UnaryOperator};
use crate::lexer::TokenType;
use crate::parser::Parser;
use crate::parser::parsers::{eat_any_of, is_any_of_token, left_hand_side_expression};

/**
 * UnaryExpression
 *  : LeftHandSideExpression
 *  | ADDITIVE_OPERATOR UnaryExpression
 *  | LOGICAL_NOT_OPERATOR UnaryExpression
 *  ;
 */
pub fn unary_expression(parser: &mut Parser) -> ExpressionRef {
    let mut operator: Option<UnaryOperator> = None;

    if is_any_of_token(
        parser,
        &[TokenType::AdditiveOperator, TokenType::LogicalNotOperator],
    ) {
        let token = eat_any_of(
            parser,
            &[TokenType::AdditiveOperator, TokenType::LogicalNotOperator],
        );
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
            right: unary_expression(parser),
        });
    }

    left_hand_side_expression(parser)
}
