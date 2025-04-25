use crate::ast::{AssignmentOperator, Expression, ExpressionRef, PrimitiveType, Type};
use crate::lexer::TokenType;
use crate::parser::parsers::expression_parse_primary::parse_identifier_expression;
use crate::parser::parsers::expression_parse_relational_and_logical::parse_logical_or_expression;
use crate::parser::parsers::utils::{
    eat, eat_any_of, is_any_of_token, is_assignment_operator_token, is_valid_assignment_target,
};
use crate::parser::Parser;

use super::utils::is_token;

///
/// VariableInitializationExpression
///  : Identifier ['=' Initializer]
///  ;
///
pub(super) fn parse_variable_initialization_expression(parser: &mut Parser) -> ExpressionRef {
    let identifier = parse_identifier_expression(parser);

    // Require type annotation
    eat(parser, TokenType::Colon);
    let type_annotation = parse_type(parser);

    let initializer: Option<ExpressionRef> =
        if is_any_of_token(parser, &[TokenType::StatementEnd, TokenType::Comma]) {
            None
        } else {
            eat(parser, TokenType::SimpleAssignmentOperator);
            let initializer = parse_assignment_expression(parser);
            Some(initializer)
        };

    Box::new(Expression::VariableInitialization {
        identifier,
        type_annotation,
        initializer,
    })
}

pub(super) fn parse_type(parser: &mut Parser) -> Type {
    match parser.lookahead.token_type {
        TokenType::NumberTypeKeyword => {
            eat(parser, TokenType::NumberTypeKeyword);
            Type::Primitive(PrimitiveType::Number)
        }
        TokenType::StringTypeKeyword => {
            eat(parser, TokenType::StringTypeKeyword);
            Type::Primitive(PrimitiveType::String)
        }
        TokenType::BooleanTypeKeyword => {
            eat(parser, TokenType::BooleanTypeKeyword);
            Type::Primitive(PrimitiveType::Boolean)
        }
        TokenType::VoidTypeKeyword => {
            eat(parser, TokenType::VoidTypeKeyword);
            Type::Void
        }
        TokenType::Identifier => {
            // Handle class types or custom types
            let identifier_token = eat(parser, TokenType::Identifier);
            let type_name = &parser.source[identifier_token.i..identifier_token.j];

            // Check for generic type parameters
            if is_token(parser, TokenType::OpeningBracket) {
                eat(parser, TokenType::OpeningBracket);
                let mut type_args = vec![];

                loop {
                    type_args.push(parse_type(parser));

                    if !is_token(parser, TokenType::Comma) {
                        break;
                    }

                    eat(parser, TokenType::Comma);
                }

                eat(parser, TokenType::ClosingBracket);

                Type::Generic {
                    base: String::from(type_name),
                    type_args,
                }
            } else {
                Type::Class {
                    name: String::from(type_name),
                    super_class: None,
                }
            }
        }
        TokenType::OpeningBracket => {
            // Handle array types
            eat(parser, TokenType::OpeningBracket);
            let element_type = parse_type(parser);
            eat(parser, TokenType::ClosingBracket);

            Type::Array(Box::new(element_type))
        }
        _ => panic!(
            "Expected type annotation, found: {}",
            parser.lookahead.token_type
        ),
    }
}

///
/// AssignmentExpression
///  : LogicalOrExpression
///  | LeftHandSideExpression ASSIGNMENT_OPERATOR AssignmentExpression
///  ;
///
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
