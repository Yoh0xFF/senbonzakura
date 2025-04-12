use crate::ast::{BinaryOperator, ExpressionRef, LogicalOperator};
use crate::lexer::TokenType;
use crate::parser::parsers::expression_parse_binary::additive_expression;
use crate::parser::parsers::utils::{parse_binary_expression, parse_logical_expression};
use crate::parser::Parser;

/**
 * LogicalOrExpression
 *  : LogicalAndExpression LOGICAL_OR_OPERATOR LogicalOrExpression
 *  | LogicalAndExpression
 *  ;
 */
pub(super) fn logical_or_expression(parser: &mut Parser) -> ExpressionRef {
    parse_logical_expression(
        parser,
        TokenType::LogicalOrOperator,
        |parser| logical_and_expression(parser),
        |op| match op {
            "||" => LogicalOperator::Or,
            _ => panic!("Unknown logical operator {}", op),
        },
    )
}

/**
 * LogicalAndExpression
 *  : EqualityExpression LOGICAL_AND_OPERATOR LogicalAndExpression
 *  | EqualityExpression
 *  ;
 */
pub(super) fn logical_and_expression(parser: &mut Parser) -> ExpressionRef {
    parse_logical_expression(
        parser,
        TokenType::LogicalAndOperator,
        |parser| equality_expression(parser),
        |op| match op {
            "&&" => LogicalOperator::And,
            _ => panic!("Unknown logical operator {}", op),
        },
    )
}

/**
 * EqualityExpression
 *  : RelationalExpression EQUALITY_OPERATOR EqualityExpression
 *  | RelationalExpression
 *  ;
 */
pub(super) fn equality_expression(parser: &mut Parser) -> ExpressionRef {
    parse_binary_expression(
        parser,
        TokenType::EqualityOperator,
        |parser| relational_expression(parser),
        |op| match op {
            "==" => BinaryOperator::Equal,
            "!=" => BinaryOperator::NotEqual,
            _ => panic!("Unknown relational operator {}", op),
        },
    )
}

/**
 * RelationalExpression
 *  : AdditiveExpression
 *  | AdditiveExpression RELATIONAL_OPERATOR AdditiveExpression
 *  ;
 */
pub(super) fn relational_expression(parser: &mut Parser) -> ExpressionRef {
    parse_binary_expression(
        parser,
        TokenType::RelationalOperator,
        |parser| additive_expression(parser),
        |op| match op {
            ">" => BinaryOperator::GreaterThan,
            ">=" => BinaryOperator::GreaterThanOrEqualTo,
            "<" => BinaryOperator::LessThan,
            "<=" => BinaryOperator::LessThanOrEqualTo,
            _ => panic!("Unknown relational operator {}", op),
        },
    )
}
