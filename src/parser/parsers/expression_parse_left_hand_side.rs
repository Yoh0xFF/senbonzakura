use crate::{
    ast::{Expression, MemberProperty},
    lexer::TokenType,
    parser::ParserResult,
    Parser,
};

use super::{
    expression_parse_assignment::parse_assignment_expression,
    expression_parse_primary::parse_primary_expression, parse_root_expression,
};

///
/// LeftHandSideExpression
///  : CallMemberExpression
///  ;
///
pub(super) fn parse_left_hand_side_expression(
    parser: &mut Parser,
) -> ParserResult<Box<Expression>> {
    parse_call_member_expression(parser)
}

///
/// CallMemberExpression
///  : PrimaryExpression
///  | CallMemberExpression Arguments
///  | CallMemberExpression MemberAccess
/// ;
///
pub(super) fn parse_call_member_expression(parser: &mut Parser) -> ParserResult<Box<Expression>> {
    let mut expression = parse_primary_expression(parser)?;

    loop {
        expression = match parser.lookahead.token_type {
            TokenType::OpeningParenthesis => Box::new(Expression::Call {
                callee: expression,
                arguments: parse_arguments(parser)?,
            }),
            TokenType::Dot | TokenType::OpeningBracket => parse_member_access(parser, expression)?,
            _ => return Ok(expression),
        };
    }
}

///
/// Arguments
///  : '(' [ArgumentList] ')'
/// ;
///
pub(super) fn parse_arguments(parser: &mut Parser) -> ParserResult<Vec<Expression>> {
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
pub(super) fn parse_arguments_list(parser: &mut Parser) -> ParserResult<Vec<Expression>> {
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
///  | MemberExpression MemberAccess
///  ;
///
pub(super) fn parse_member_expression(parser: &mut Parser) -> ParserResult<Box<Expression>> {
    let mut object = parse_primary_expression(parser)?;

    while parser.is_next_token_any_of_type(&[TokenType::Dot, TokenType::OpeningBracket]) {
        object = parse_member_access(parser, object)?;
    }

    Ok(object)
}

///
/// MemberAccess
///  : '.' Identifier
///  | '[' Expression ']'
///  ;
///
fn parse_member_access(
    parser: &mut Parser,
    object: Box<Expression>,
) -> ParserResult<Box<Expression>> {
    let property = if parser.is_next_token_of_type(TokenType::Dot) {
        parser.eat_token(TokenType::Dot)?;
        let property_token = parser.eat_token(TokenType::Identifier)?;
        MemberProperty::Named(String::from(property_token.text(parser.source)))
    } else {
        parser.eat_token(TokenType::OpeningBracket)?;
        let property = parse_root_expression(parser)?;
        parser.eat_token(TokenType::ClosingBracket)?;
        MemberProperty::Computed(property)
    };

    Ok(Box::new(Expression::Member { object, property }))
}
