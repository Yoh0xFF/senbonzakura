use crate::ast::{BinaryOperator, ExpressionRef};
use crate::lexer::TokenType;
use crate::parser::parsers::expression_parse_unary::parse_unary_expression;
use crate::parser::parsers::internal_util::parse_binary_expression;
use crate::parser::{Parser, ParserError, ParserResult};

///
/// AdditiveExpression
///  : FactorExpression
///  | AdditiveExpression ADDITIVE_OPERATOR FactorExpression
///  ;
///
pub(super) fn parse_additive_expression(parser: &mut Parser) -> ParserResult<ExpressionRef> {
    parse_binary_expression(
        parser,
        &[
            TokenType::AdditivePlusOperator,
            TokenType::AdditiveMinusOperator,
        ],
        |parser| parse_factor_expression(parser),
        |op| match op {
            TokenType::AdditivePlusOperator => Ok(BinaryOperator::Add),
            TokenType::AdditiveMinusOperator => Ok(BinaryOperator::Subtract),
            _ => Err(ParserError::ParserError {
                message: format!("Unknown additive operator {}", op),
            }),
        },
    )
}

///
/// FactorExpression
///  : PrimaryExpression
///  | FactorExpression FACTOR_OPERATOR PrimaryExpression
///  ;
///
pub(super) fn parse_factor_expression(parser: &mut Parser) -> ParserResult<ExpressionRef> {
    parse_binary_expression(
        parser,
        &[
            TokenType::FactorMultiplicationOperator,
            TokenType::FactorDivisionOperator,
        ],
        |parser| parse_unary_expression(parser),
        |op| match op {
            TokenType::FactorMultiplicationOperator => Ok(BinaryOperator::Multiply),
            TokenType::FactorDivisionOperator => Ok(BinaryOperator::Divide),
            _ => Err(ParserError::ParserError {
                message: format!("Unknown factor operator {}", op),
            }),
        },
    )
}
