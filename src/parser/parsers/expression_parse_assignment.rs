use crate::ast::{AssignmentOperator, Expression, ExpressionRef};
use crate::lexer::TokenType;
use crate::parser::parsers::expression_parse_relational_and_logical::parse_logical_or_expression;
use crate::parser::{Parser, ParserError, ParserResult};

///
/// AssignmentExpression
///  : LogicalOrExpression
///  | LeftHandSideExpression ASSIGNMENT_OPERATOR AssignmentExpression
///  ;
///
pub(super) fn parse_assignment_expression(parser: &mut Parser) -> ParserResult<ExpressionRef> {
    let left = parse_logical_or_expression(parser)?;

    if !parser.is_next_token_assignment_operator() {
        return Ok(left);
    }

    let assignment_operator_token = parser.eat_any_of_token(&[
        TokenType::SimpleAssignmentOperator,
        TokenType::ComplexPlusAssignmentOperator,
        TokenType::ComplexMinusAssignmentOperator,
        TokenType::ComplexMultiplyAssignmentOperator,
        TokenType::ComplexDivideAssignmentOperator,
    ])?;

    let assignment_operator = match assignment_operator_token.token_type {
        TokenType::SimpleAssignmentOperator => AssignmentOperator::Assign,
        TokenType::ComplexPlusAssignmentOperator => AssignmentOperator::AssignAdd,
        TokenType::ComplexMinusAssignmentOperator => AssignmentOperator::AssignSubtract,
        TokenType::ComplexMultiplyAssignmentOperator => AssignmentOperator::AssignMultiply,
        TokenType::ComplexDivideAssignmentOperator => AssignmentOperator::AssignDivide,
        _ => {
            return Err(ParserError::ParserError {
                message: format!("Unknown assignment operator {}", assignment_operator_token),
            })
        }
    };

    if !parser.is_expression_valid_assignment_target(&left) {
        return Err(ParserError::SemanticError {
            message: String::from("Invalid left-hand side in the assignment expression"),
        });
    }

    let right = parse_assignment_expression(parser)?;
    Ok(Box::new(Expression::Assignment {
        operator: assignment_operator,
        left,
        right: right,
    }))
}
