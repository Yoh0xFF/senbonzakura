use crate::ast::{BinaryOperator, ExpressionRef};
use crate::lexer::TokenType;
use crate::parser::Parser;
use crate::parser::parsers::{parse_binary_expression, unary_expression};

/**
 * AdditiveExpression
 *  : FactorExpression
 *  | AdditiveExpression ADDITIVE_OPERATOR FactorExpression
 *  ;
 */
pub fn additive_expression(parser: &mut Parser) -> ExpressionRef {
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
pub fn factor_expression(parser: &mut Parser) -> ExpressionRef {
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
