use crate::ast::{BinaryOperator, Expression, ExpressionRef, LogicalOperator};
use crate::lexer::TokenType;
use crate::parser::Parser;

///
/// Parse generic binary expression
///
pub(super) fn parse_binary_expression<OperandParserFnType, OperatorMapperFnType>(
    parser: &mut Parser,
    token_types: &[TokenType],
    operand_parser: OperandParserFnType,
    operator_mapper: OperatorMapperFnType,
) -> ExpressionRef
where
    OperandParserFnType: Fn(&mut Parser) -> ExpressionRef,
    OperatorMapperFnType: Fn(TokenType) -> BinaryOperator,
{
    let mut left = operand_parser(parser);

    while parser.is_next_token_any_of_type(token_types) {
        let operator_token = parser.eat_any_of_token(token_types);
        let operator = operator_mapper(operator_token.token_type);

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
    token_types: &[TokenType],
    operand_parser: OperandParserFnType,
    operator_mapper: OperatorMapperFnType,
) -> ExpressionRef
where
    OperandParserFnType: Fn(&mut Parser) -> ExpressionRef,
    OperatorMapperFnType: Fn(TokenType) -> LogicalOperator,
{
    let mut left = operand_parser(parser);

    while parser.is_next_token_any_of_type(token_types) {
        let operator_token = parser.eat_any_of_token(token_types);
        let operator = operator_mapper(operator_token.token_type);

        let right = operand_parser(parser);

        left = Box::new(Expression::Logical {
            operator,
            left,
            right,
        });
    }

    left
}
