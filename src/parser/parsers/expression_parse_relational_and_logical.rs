use crate::ast::{BinaryOperator, ExpressionRef, LogicalOperator};
use crate::lexer::TokenType;
use crate::parser::Parser;
use crate::parser::parsers::{additive_expression, parse_binary_expression, parse_logical_expression};

/**
 * LogicalOrExpression
 *  : LogicalAndExpression LOGICAL_OR_OPERATOR LogicalOrExpression
 *  | LogicalAndExpression
 *  ;
 */
pub fn logical_or_expression(parser: &mut Parser) -> ExpressionRef {
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
pub fn logical_and_expression(parser: &mut Parser) -> ExpressionRef {
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
pub fn equality_expression(parser: &mut Parser) -> ExpressionRef {
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
pub fn relational_expression(parser: &mut Parser) -> ExpressionRef {
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
