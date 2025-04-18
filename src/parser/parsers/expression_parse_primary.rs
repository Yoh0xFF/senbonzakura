use crate::ast::{Expression, ExpressionList, ExpressionRef};
use crate::lexer::TokenType;
use crate::parser::parsers::expression_parse_literals::parse_literal_expression;
use crate::parser::parsers::root::parse_root_expression;
use crate::parser::parsers::utils::{eat, is_any_of_token, is_literal_token, is_token};
use crate::parser::Parser;
use crate::parser::parsers::expression_parse_variable_initialization_and_assignment::parse_assignment_expression;

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
fn parse_call_expression(parser: &mut Parser, callee: ExpressionRef) -> ExpressionRef {
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
fn parse_arguments(parser: &mut Parser) -> ExpressionList {
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
fn parse_arguments_list(parser: &mut Parser) -> ExpressionList {
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

///
/// PrimaryExpression
///  : LiteralExpression
///  | GroupExpression
///  | IdentifierExpression
///  ;
///
pub(super) fn parse_primary_expression(parser: &mut Parser) -> ExpressionRef {
    if is_literal_token(parser) {
        return parse_literal_expression(parser);
    }

    match parser.lookahead.token_type {
        TokenType::OpeningParenthesis => parse_group_expression(parser),
        TokenType::Identifier => parse_identifier_expression(parser),
        _ => parse_left_hand_side_expression(parser),
    }
}

///
/// GroupExpression
///  : '(' Expression ')'
///  ;
///
pub(super) fn parse_group_expression(parser: &mut Parser) -> ExpressionRef {
    eat(parser, TokenType::OpeningParenthesis);
    let expression_ref = parse_root_expression(parser);
    eat(parser, TokenType::ClosingParenthesis);

    expression_ref
}

///
/// IdentifierExpression
///  : IDENTIFIER
///  ;
///
pub(super) fn parse_identifier_expression(parser: &mut Parser) -> ExpressionRef {
    let identifier_token = eat(parser, TokenType::Identifier);
    let identifier_value = &parser.source[identifier_token.i..identifier_token.j];

    Box::new(Expression::Identifier(String::from(identifier_value)))
}
