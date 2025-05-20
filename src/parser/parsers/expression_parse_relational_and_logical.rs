use crate::ast::{BinaryOperator, ExpressionRef, LogicalOperator};
use crate::lexer::TokenType;
use crate::parser::parsers::expression_parse_binary::parse_additive_expression;
use crate::parser::parsers::internal_util::{parse_binary_expression, parse_logical_expression};
use crate::parser::Parser;

///
/// LogicalOrExpression
///  : LogicalAndExpression LOGICAL_OR_OPERATOR LogicalOrExpression
///  | LogicalAndExpression
///  ;
///
pub(super) fn parse_logical_or_expression(parser: &mut Parser) -> ExpressionRef {
    parse_logical_expression(
        parser,
        TokenType::LogicalOrOperator,
        |parser| parse_logical_and_expression(parser),
        |op| match op {
            "||" => LogicalOperator::Or,
            _ => panic!("Unknown logical operator {}", op),
        },
    )
}

///
/// LogicalAndExpression
///  : EqualityExpression LOGICAL_AND_OPERATOR LogicalAndExpression
///  | EqualityExpression
///  ;
///
pub(super) fn parse_logical_and_expression(parser: &mut Parser) -> ExpressionRef {
    parse_logical_expression(
        parser,
        TokenType::LogicalAndOperator,
        |parser| parse_equality_expression(parser),
        |op| match op {
            "&&" => LogicalOperator::And,
            _ => panic!("Unknown logical operator {}", op),
        },
    )
}

///
/// EqualityExpression
///  : RelationalExpression EQUALITY_OPERATOR EqualityExpression
///  | RelationalExpression
///  ;
///
pub(super) fn parse_equality_expression(parser: &mut Parser) -> ExpressionRef {
    parse_binary_expression(
        parser,
        TokenType::EqualityOperator,
        |parser| parse_relational_expression(parser),
        |op| match op {
            "==" => BinaryOperator::Equal,
            "!=" => BinaryOperator::NotEqual,
            _ => panic!("Unknown relational operator {}", op),
        },
    )
}

///
/// RelationalExpression
///  : AdditiveExpression
///  | AdditiveExpression RELATIONAL_OPERATOR AdditiveExpression
///  ;
///
pub(super) fn parse_relational_expression(parser: &mut Parser) -> ExpressionRef {
    parse_binary_expression(
        parser,
        TokenType::RelationalOperator,
        |parser| parse_additive_expression(parser),
        |op| match op {
            ">" => BinaryOperator::GreaterThan,
            ">=" => BinaryOperator::GreaterThanOrEqualTo,
            "<" => BinaryOperator::LessThan,
            "<=" => BinaryOperator::LessThanOrEqualTo,
            _ => panic!("Unknown relational operator {}", op),
        },
    )
}
