use crate::{
    ast::{Expression, ExpressionList, ExpressionRef},
    lexer::TokenType,
    Parser,
};

use super::{
    expression_parse_assignment::parse_assignment_expression,
    expression_parse_primary::{parse_identifier_expression, parse_primary_expression},
    internal_util::{eat_token, is_next_token_any_of_type, is_next_token_of_type},
    parse_root_expression,
};

///
/// LeftHandSideExpression
///  : CallMemberExpression
///  ;
///
pub(super) fn parse_left_hand_side_expression(parser: &mut Parser) -> ExpressionRef {
    parse_call_member_expression(parser)
}

///
/// CallMemberExpression
///  : MemberExpression
///  | CallExpression
/// ;
///
pub(super) fn parse_call_member_expression(parser: &mut Parser) -> ExpressionRef {
    // Member part might be part of a call
    let member = parse_member_expression(parser);

    // See if we have a call expression
    if is_next_token_of_type(parser, TokenType::OpeningParenthesis) {
        return parse_call_expression(parser, member);
    }

    // Simple member expression
    member
}

///
/// Generic call expression helper
///
/// CallExpression
///  : Callee Arguments
/// ;
///
/// Callee
///  : MemberExpression
///  | CallExpression
/// ;
///
pub(super) fn parse_call_expression(parser: &mut Parser, callee: ExpressionRef) -> ExpressionRef {
    let mut call_expression = Box::new(Expression::Call {
        callee: callee.clone(),
        arguments: parse_arguments(parser),
    });

    if is_next_token_of_type(parser, TokenType::OpeningParenthesis) {
        call_expression = parse_call_expression(parser, call_expression);
    }

    call_expression
}

///
/// Arguments
///  : '(' [ArgumentList] ')'
/// ;
///
pub(super) fn parse_arguments(parser: &mut Parser) -> ExpressionList {
    eat_token(parser, TokenType::OpeningParenthesis);
    let arguments = if is_next_token_of_type(parser, TokenType::ClosingParenthesis) {
        vec![]
    } else {
        parse_arguments_list(parser)
    };
    eat_token(parser, TokenType::ClosingParenthesis);

    arguments
}

///
/// ArgumentList
///  : AssignmentExpression
///  | ArgumentList ',' AssignmentExpression
/// ;
///
pub(super) fn parse_arguments_list(parser: &mut Parser) -> ExpressionList {
    let mut arguments = vec![];

    loop {
        arguments.push(*parse_assignment_expression(parser));

        if is_next_token_of_type(parser, TokenType::Comma) {
            eat_token(parser, TokenType::Comma);
        } else {
            break;
        }
    }

    arguments
}

///
/// MemberExpression
///  : PrimaryExpression
///  | MemberExpression '.' Identifier
///  | MemberExpression '[' Expression ']'
///  ;
///
pub(super) fn parse_member_expression(parser: &mut Parser) -> ExpressionRef {
    let mut object = parse_primary_expression(parser);

    while is_next_token_any_of_type(parser, &[TokenType::Dot, TokenType::OpeningBracket]) {
        if is_next_token_of_type(parser, TokenType::Dot) {
            eat_token(parser, TokenType::Dot);
            let property = parse_identifier_expression(parser);

            object = Box::new(Expression::Member {
                computed: false,
                object,
                property,
            });
        }

        if is_next_token_of_type(parser, TokenType::OpeningBracket) {
            eat_token(parser, TokenType::OpeningBracket);
            let property = parse_root_expression(parser);
            eat_token(parser, TokenType::ClosingBracket);

            object = Box::new(Expression::Member {
                computed: true,
                object,
                property,
            });
        }
    }

    object
}
