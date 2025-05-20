use crate::{
    ast::{Expression, ExpressionList, ExpressionRef},
    lexer::TokenType,
    Parser,
};

use super::{
    expression_parse_assignment::parse_assignment_expression,
    expression_parse_primary::{parse_identifier_expression, parse_primary_expression},
    parse_root_expression,
    internal_util::{eat, is_any_of_token, is_token},
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
    if is_token(parser, TokenType::OpeningParenthesis) {
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

    if is_token(parser, TokenType::OpeningParenthesis) {
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
    eat(parser, TokenType::OpeningParenthesis);
    let arguments = if is_token(parser, TokenType::ClosingParenthesis) {
        vec![]
    } else {
        parse_arguments_list(parser)
    };
    eat(parser, TokenType::ClosingParenthesis);

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

        if is_token(parser, TokenType::Comma) {
            eat(parser, TokenType::Comma);
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

    while is_any_of_token(parser, &[TokenType::Dot, TokenType::OpeningBracket]) {
        if is_token(parser, TokenType::Dot) {
            eat(parser, TokenType::Dot);
            let property = parse_identifier_expression(parser);

            object = Box::new(Expression::Member {
                computed: false,
                object,
                property,
            });
        }

        if is_token(parser, TokenType::OpeningBracket) {
            eat(parser, TokenType::OpeningBracket);
            let property = parse_root_expression(parser);
            eat(parser, TokenType::ClosingBracket);

            object = Box::new(Expression::Member {
                computed: true,
                object,
                property,
            });
        }
    }

    object
}
