use crate::ast::{BinaryOperator, ExpressionRef};
use crate::lexer::TokenType;
use crate::parser::parsers::expression_parse_unary::unary_expression;
use crate::parser::parsers::utils::parse_binary_expression;
use crate::parser::Parser;

/**
 * AdditiveExpression
 *  : FactorExpression
 *  | AdditiveExpression ADDITIVE_OPERATOR FactorExpression
 *  ;
 */
pub(super) fn additive_expression(parser: &mut Parser) -> ExpressionRef {
    parse_binary_expression(
        parser,
        TokenType::AdditiveOperator,
        |parser| factor_expression(parser),
        |op| match op {
            "+" => BinaryOperator::Add,
            "-" => BinaryOperator::Subtract,
            _ => panic!("Unknown additive operator {}", op),
        },
    )
}

/**
 * FactorExpression
 *  : PrimaryExpression
 *  | FactorExpression FACTOR_OPERATOR PrimaryExpression
 *  ;
 */
pub(super) fn factor_expression(parser: &mut Parser) -> ExpressionRef {
    parse_binary_expression(
        parser,
        TokenType::FactorOperator,
        |parser| unary_expression(parser),
        |op| match op {
            "*" => BinaryOperator::Multiply,
            "/" => BinaryOperator::Divide,
            _ => panic!("Unknown factor operator {}", op),
        },
    )
}
