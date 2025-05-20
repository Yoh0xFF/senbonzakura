use crate::ast::{AssignmentOperator, Expression, ExpressionRef};
use crate::lexer::TokenType;
use crate::parser::parsers::expression_parse_relational_and_logical::parse_logical_or_expression;
use crate::parser::Parser;

///
/// AssignmentExpression
///  : LogicalOrExpression
///  | LeftHandSideExpression ASSIGNMENT_OPERATOR AssignmentExpression
///  ;
///
pub(super) fn parse_assignment_expression(parser: &mut Parser) -> ExpressionRef {
    let left = parse_logical_or_expression(parser);

    if !parser.is_next_token_assignment_operator() {
        return left;
    }

    let assignment_operator_token = parser.eat_any_of_token(&[
        TokenType::SimpleAssignmentOperator,
        TokenType::ComplexAssignmentOperator,
    ]);
    let assignment_operator_value =
        &parser.source[assignment_operator_token.i..assignment_operator_token.j];
    let assignment_operator = match assignment_operator_value {
        "=" => AssignmentOperator::Assign,
        "+=" => AssignmentOperator::AssignAdd,
        "-=" => AssignmentOperator::AssignSubtract,
        "*=" => AssignmentOperator::AssignMultiply,
        "/=" => AssignmentOperator::AssignDivide,
        _ => panic!("Unknown assignment operator {}", assignment_operator_value),
    };

    if !parser.is_expression_valid_assignment_target(&left) {
        panic!("Invalid left-hand side in the assignment expression");
    }

    Box::new(Expression::Assignment {
        operator: assignment_operator,
        left,
        right: parse_assignment_expression(parser),
    })
}
