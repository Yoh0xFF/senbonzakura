use crate::ast::{BinaryOperator, ExpressionRef, LogicalOperator};
use crate::lexer::TokenType;
use crate::parser::parsers::expression_parse_binary::parse_additive_expression;
use crate::parser::parsers::internal_util::{parse_binary_expression, parse_logical_expression};
use crate::parser::{Parser, ParserError, ParserResult};

///
/// LogicalOrExpression
///  : LogicalAndExpression LOGICAL_OR_OPERATOR LogicalOrExpression
///  | LogicalAndExpression
///  ;
///
pub(super) fn parse_logical_or_expression(parser: &mut Parser) -> ParserResult<ExpressionRef> {
    parse_logical_expression(
        parser,
        &[TokenType::LogicalOrOperator],
        |parser| parse_logical_and_expression(parser),
        |op| match op {
            TokenType::LogicalOrOperator => Ok(LogicalOperator::Or),
            _ => Err(ParserError::ParserError {
                message: format!("Unknown logical operator {}", op),
            }),
        },
    )
}

///
/// LogicalAndExpression
///  : EqualityExpression LOGICAL_AND_OPERATOR LogicalAndExpression
///  | EqualityExpression
///  ;
///
pub(super) fn parse_logical_and_expression(parser: &mut Parser) -> ParserResult<ExpressionRef> {
    parse_logical_expression(
        parser,
        &[TokenType::LogicalAndOperator],
        |parser| parse_equality_expression(parser),
        |op| match op {
            TokenType::LogicalAndOperator => Ok(LogicalOperator::And),
            _ => Err(ParserError::ParserError {
                message: format!("Unknown logical operator {}", op),
            }),
        },
    )
}

///
/// EqualityExpression
///  : RelationalExpression EQUALITY_OPERATOR EqualityExpression
///  | RelationalExpression
///  ;
///
pub(super) fn parse_equality_expression(parser: &mut Parser) -> ParserResult<ExpressionRef> {
    parse_binary_expression(
        parser,
        &[TokenType::EqualOperator, TokenType::NotEqualOperator],
        |parser| parse_relational_expression(parser),
        |op| match op {
            TokenType::EqualOperator => Ok(BinaryOperator::Equal),
            TokenType::NotEqualOperator => Ok(BinaryOperator::NotEqual),
            _ => Err(ParserError::ParserError {
                message: format!("Unknown relational operator {}", op),
            }),
        },
    )
}

///
/// RelationalExpression
///  : AdditiveExpression
///  | AdditiveExpression RELATIONAL_OPERATOR AdditiveExpression
///  ;
///
pub(super) fn parse_relational_expression(parser: &mut Parser) -> ParserResult<ExpressionRef> {
    parse_binary_expression(
        parser,
        &[
            TokenType::RelationalGreaterThanOperator,
            TokenType::RelationalGreaterThanOrEqualToOperator,
            TokenType::RelationalLessThanOperator,
            TokenType::RelationalLessThanOrEqualToOperator,
        ],
        |parser| parse_additive_expression(parser),
        |op| match op {
            TokenType::RelationalGreaterThanOperator => Ok(BinaryOperator::GreaterThan),
            TokenType::RelationalGreaterThanOrEqualToOperator => {
                Ok(BinaryOperator::GreaterThanOrEqualTo)
            }
            TokenType::RelationalLessThanOperator => Ok(BinaryOperator::LessThan),
            TokenType::RelationalLessThanOrEqualToOperator => Ok(BinaryOperator::LessThanOrEqualTo),
            _ => Err(ParserError::ParserError {
                message: format!("Unknown relational operator {}", op),
            }),
        },
    )
}
