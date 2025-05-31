use crate::{
    ast::{Expression, ExpressionList, ExpressionRef},
    lexer::TokenType,
    parser::ParserResult,
    Parser,
};

use super::{
    expression_parse_assignment::parse_assignment_expression,
    expression_parse_primary::{parse_identifier_expression, parse_primary_expression},
    parse_root_expression,
};

///
/// LeftHandSideExpression
///  : CallMemberExpression
///  ;
///
pub(super) fn parse_left_hand_side_expression(parser: &mut Parser) -> ParserResult<ExpressionRef> {
    parse_call_member_expression(parser)
}

///
/// CallMemberExpression
///  : MemberExpression
///  | CallExpression
/// ;
///
pub(super) fn parse_call_member_expression(parser: &mut Parser) -> ParserResult<ExpressionRef> {
    // Member part might be part of a call
    let member = parse_member_expression(parser)?;

    // See if we have a call expression
    if parser.is_next_token_of_type(TokenType::OpeningParenthesis) {
        return parse_call_expression(parser, member);
    }

    // Simple member expression
    Ok(member)
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
pub(super) fn parse_call_expression(
    parser: &mut Parser,
    callee: ExpressionRef,
) -> ParserResult<ExpressionRef> {
    let mut call_expression = Box::new(Expression::Call {
        callee: callee.clone(),
        arguments: parse_arguments(parser)?,
    });

    if parser.is_next_token_of_type(TokenType::OpeningParenthesis) {
        call_expression = parse_call_expression(parser, call_expression)?;
    }

    Ok(call_expression)
}

///
/// Arguments
///  : '(' [ArgumentList] ')'
/// ;
///
pub(super) fn parse_arguments(parser: &mut Parser) -> ParserResult<ExpressionList> {
    parser.eat_token(TokenType::OpeningParenthesis)?;
    let arguments = if parser.is_next_token_of_type(TokenType::ClosingParenthesis) {
        vec![]
    } else {
        parse_arguments_list(parser)?
    };
    parser.eat_token(TokenType::ClosingParenthesis)?;

    Ok(arguments)
}

///
/// ArgumentList
///  : AssignmentExpression
///  | ArgumentList ',' AssignmentExpression
/// ;
///
pub(super) fn parse_arguments_list(parser: &mut Parser) -> ParserResult<ExpressionList> {
    let mut arguments = vec![];

    loop {
        let expression = parse_assignment_expression(parser)?;
        arguments.push(*expression);

        if parser.is_next_token_of_type(TokenType::Comma) {
            parser.eat_token(TokenType::Comma)?;
        } else {
            break;
        }
    }

    Ok(arguments)
}

///
/// MemberExpression
///  : PrimaryExpression
///  | MemberExpression '.' Identifier
///  | MemberExpression '[' Expression ']'
///  ;
///
pub(super) fn parse_member_expression(parser: &mut Parser) -> ParserResult<ExpressionRef> {
    let mut object = parse_primary_expression(parser)?;

    while parser.is_next_token_any_of_type(&[TokenType::Dot, TokenType::OpeningBracket]) {
        if parser.is_next_token_of_type(TokenType::Dot) {
            parser.eat_token(TokenType::Dot)?;
            let property = parse_identifier_expression(parser)?;

            object = Box::new(Expression::Member {
                computed: false,
                object,
                property,
            });
        }

        if parser.is_next_token_of_type(TokenType::OpeningBracket) {
            parser.eat_token(TokenType::OpeningBracket)?;
            let property = parse_root_expression(parser)?;
            parser.eat_token(TokenType::ClosingBracket)?;

            object = Box::new(Expression::Member {
                computed: true,
                object,
                property,
            });
        }
    }

    Ok(object)
}
