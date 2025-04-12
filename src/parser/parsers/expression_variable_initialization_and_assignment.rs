use crate::ast::{AssignmentOperator, Expression, ExpressionRef};
use crate::lexer::TokenType;
use crate::parser::Parser;
use crate::parser::parsers::{eat, eat_any_of, identifier_expression, is_any_of_token, is_assignment_operator_token, is_valid_assignment_target, logical_or_expression};

/**
 * VariableInitializationExpression
 *  : Identifier ['=' Initializer]
 *  ;
 */
pub fn variable_initialization_expression(parser: &mut Parser) -> ExpressionRef {
    let identifier = identifier_expression(parser);

    let initializer: Option<ExpressionRef> =
        if is_any_of_token(parser, &[TokenType::StatementEnd, TokenType::Comma]) {
            None
        } else {
            eat(parser, TokenType::SimpleAssignmentOperator);
            let initializer = assignment_expression(parser);
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
pub fn assignment_expression(parser: &mut Parser) -> ExpressionRef {
    let left = logical_or_expression(parser);

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
        right: assignment_expression(parser),
    })
}
