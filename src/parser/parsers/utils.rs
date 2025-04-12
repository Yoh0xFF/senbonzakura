use crate::ast::{BinaryOperator, Expression, ExpressionRef, LogicalOperator};
use crate::lexer::{Token, TokenType};
use crate::parser::Parser;

///
/// Expects a token of a given type
///
pub(super) fn eat(parser: &mut Parser, token_type: TokenType) -> Token {
    if parser.lookahead.token_type != token_type {
        panic!(
            "Unexpected token: {}, expected token: '{}'",
            parser.lookahead.token_type, token_type
        );
    }

    let pre_token = parser.lookahead;
    parser.lookahead = parser.lexer.next_token();
    pre_token
}

///
/// Expects a token of a given types
///
pub(super) fn eat_any_of(parser: &mut Parser, token_types: &[TokenType]) -> Token {
    for token_type in token_types {
        if parser.lookahead.token_type == *token_type {
            let pre_token = parser.lookahead;
            parser.lookahead = parser.lexer.next_token();
            return pre_token;
        }
    }

    panic!(
        "Unexpected token: {}, expected tokens: '{:?}'",
        parser.lookahead.token_type, token_types
    );
}

///
/// Check the current token type
///
#[allow(dead_code)]
pub(super) fn is_token(parser: &mut Parser, token_type: TokenType) -> bool {
    parser.lookahead.token_type == token_type
}

///
/// Check the current token type
///
#[allow(dead_code)]
pub(super) fn is_any_of_token(parser: &mut Parser, token_types: &[TokenType]) -> bool {
    for token_type in token_types {
        if parser.lookahead.token_type == *token_type {
            return true;
        }
    }

    false
}

///
/// Check if the expression is valid assignment target
///
#[allow(dead_code)]
pub(super) fn is_valid_assignment_target(expression: &ExpressionRef) -> bool {
    match expression.as_ref() {
        Expression::Identifier(_) => true,
        _ => false,
    }
}

///
/// Check if the current token is literal
///
#[allow(dead_code)]
pub(super) fn is_literal_token(parser: &mut Parser) -> bool {
    is_any_of_token(
        parser,
        &[
            TokenType::Boolean,
            TokenType::Nil,
            TokenType::Number,
            TokenType::String,
        ],
    )
}

///
/// Check if the current token is assignment operator
///
#[allow(dead_code)]
pub(super) fn is_assignment_operator_token(parser: &mut Parser) -> bool {
    is_any_of_token(
        parser,
        &[
            TokenType::SimpleAssignmentOperator,
            TokenType::ComplexAssignmentOperator,
        ],
    )
}

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
        let operator_token = eat(parser, token_type);
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
        let operator_token = eat(parser, token_type);
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
