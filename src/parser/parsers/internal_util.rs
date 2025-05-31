use crate::ast::{BinaryOperator, Expression, ExpressionRef, LogicalOperator};
use crate::lexer::TokenType;
use crate::parser::{Parser, ParserResult};

///
/// Parse generic binary expression
///
pub(super) fn parse_binary_expression<OperandParserFnType, OperatorMapperFnType>(
    parser: &mut Parser,
    token_types: &[TokenType],
    operand_parser: OperandParserFnType,
    operator_mapper: OperatorMapperFnType,
) -> ParserResult<ExpressionRef>
where
    OperandParserFnType: Fn(&mut Parser) -> ParserResult<ExpressionRef>,
    OperatorMapperFnType: Fn(TokenType) -> ParserResult<BinaryOperator>,
{
    let mut left = operand_parser(parser)?;

    while parser.is_next_token_any_of_type(token_types) {
        let operator_token = parser.eat_any_of_token(token_types)?;
        let operator = operator_mapper(operator_token.token_type)?;

        let right = operand_parser(parser)?;

        left = Box::new(Expression::Binary {
            operator,
            left,
            right,
        });
    }

    Ok(left)
}

///
/// Parse generic logical expression
///
pub(super) fn parse_logical_expression<OperandParserFnType, OperatorMapperFnType>(
    parser: &mut Parser,
    token_types: &[TokenType],
    operand_parser: OperandParserFnType,
    operator_mapper: OperatorMapperFnType,
) -> ParserResult<ExpressionRef>
where
    OperandParserFnType: Fn(&mut Parser) -> ParserResult<ExpressionRef>,
    OperatorMapperFnType: Fn(TokenType) -> ParserResult<LogicalOperator>,
{
    let mut left = operand_parser(parser)?;

    while parser.is_next_token_any_of_type(token_types) {
        let operator_token = parser.eat_any_of_token(token_types)?;
        let operator = operator_mapper(operator_token.token_type)?;

        let right = operand_parser(parser)?;

        left = Box::new(Expression::Logical {
            operator,
            left,
            right,
        });
    }

    Ok(left)
}
