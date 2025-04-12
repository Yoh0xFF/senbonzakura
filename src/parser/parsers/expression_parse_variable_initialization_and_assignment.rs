use crate::ast::{AssignmentOperator, Expression, ExpressionRef};
use crate::lexer::TokenType;
use crate::parser::parsers::expression_parse_primary::parse_identifier_expression;
use crate::parser::parsers::expression_parse_relational_and_logical::parse_logical_or_expression;
use crate::parser::parsers::utils::{
    eat, eat_any_of, is_any_of_token, is_assignment_operator_token, is_valid_assignment_target,
};
use crate::parser::Parser;

/**
 * VariableInitializationExpression
 *  : Identifier ['=' Initializer]
 *  ;
 */
pub(super) fn parse_variable_initialization_expression(parser: &mut Parser) -> ExpressionRef {
    let identifier = parse_identifier_expression(parser);

    let initializer: Option<ExpressionRef> =
        if is_any_of_token(parser, &[TokenType::StatementEnd, TokenType::Comma]) {
            None
        } else {
            eat(parser, TokenType::SimpleAssignmentOperator);
            let initializer = parse_assignment_expression(parser);
            Some(initializer)
        };

    Box::new(Expression::VariableIntialization {
        identifier,
        initializer,
    })
}

/**
 * AssignmentExpression
 *  : LogicalOrExpression
 *  | LeftHandSideExpression ASSIGNMENT_OPERATOR AssignmentExpression
 *  ;
 */
pub(super) fn parse_assignment_expression(parser: &mut Parser) -> ExpressionRef {
    let left = parse_logical_or_expression(parser);

    if !is_assignment_operator_token(parser) {
        return left;
    }

    let assignment_operator_token = eat_any_of(
        parser,
        &[
            TokenType::SimpleAssignmentOperator,
            TokenType::ComplexAssignmentOperator,
        ],
    );
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

    if !is_valid_assignment_target(&left) {
        panic!("Invalid left-hand side in the assignment expression");
    }

    Box::new(Expression::Assignment {
        operator: assignment_operator,
        left,
        right: parse_assignment_expression(parser),
    })
}
