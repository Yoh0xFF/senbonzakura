use crate::ast::{BinaryOperator, Expression, ExpressionRef, LogicalOperator};
use crate::lexer::TokenType;
use crate::parser::Parser;

///
/// Parse generic binary expression
///
pub(super) fn parse_binary_expression<OperandParserFnType, OperatorMapperFnType>(
    parser: &mut Parser,
    token_type: TokenType,
    operand_parser: OperandParserFnType,
    operator_mapper: OperatorMapperFnType,
) -> ExpressionRef
where
    OperandParserFnType: Fn(&mut Parser) -> ExpressionRef,
    OperatorMapperFnType: Fn(&str) -> BinaryOperator,
{
    let mut left = operand_parser(parser);

    while parser.lookahead.token_type == token_type {
        let operator_token = parser.eat_token(token_type);
        let operator_value = &parser.source[operator_token.i..operator_token.j];
        let operator = operator_mapper(operator_value);

        let right = operand_parser(parser);

        left = Box::new(Expression::Binary {
            operator,
            left,
            right,
        });
    }

    left
}

///
/// Parse generic logical expression
///
pub(super) fn parse_logical_expression<OperandParserFnType, OperatorMapperFnType>(
    parser: &mut Parser,
    token_type: TokenType,
    operand_parser: OperandParserFnType,
    operator_mapper: OperatorMapperFnType,
) -> ExpressionRef
where
    OperandParserFnType: Fn(&mut Parser) -> ExpressionRef,
    OperatorMapperFnType: Fn(&str) -> LogicalOperator,
{
    let mut left = operand_parser(parser);

    while parser.lookahead.token_type == token_type {
        let operator_token = parser.eat_token(token_type);
        let operator_value = &parser.source[operator_token.i..operator_token.j];
        let operator = operator_mapper(operator_value);

        let right = operand_parser(parser);

        left = Box::new(Expression::Logical {
            operator,
            left,
            right,
        });
    }

    left
}
