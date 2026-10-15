use super::expression_parse_left_hand_side::{parse_arguments, parse_member_expression};
use crate::ast::Expression;
use crate::lexer::TokenType;
use crate::parser::parsers::expression_parse_literals::parse_literal_expression;
use crate::parser::parsers::root::parse_root_expression;
use crate::parser::{Parser, ParserError, ParserResult};

///
/// PrimaryExpression
///  : LiteralExpression
///  | GroupExpression
///  | IdentifierExpression
///  | ThisExpression
///  ;
///
pub(super) fn parse_primary_expression(parser: &mut Parser) -> ParserResult<Box<Expression>> {
    if parser.is_next_token_literal() {
        return parse_literal_expression(parser);
    }

    match parser.lookahead.token_type {
        TokenType::OpeningParenthesis => parse_group_expression(parser),
        TokenType::Identifier => parse_identifier_expression(parser),
        TokenType::ThisKeyword => parse_this_expression(parser),
        TokenType::SuperKeyword => parse_super_expression(parser),
        TokenType::NewKeyword => parse_new_expression(parser),
        _ => Err(ParserError::ParserError {
            message: format!(
                "Unexpected token: {}, expected an expression",
                parser.lookahead.token_type
            ),
        }),
    }
}

///
/// GroupExpression
///  : '(' Expression ')'
///  ;
///
pub(super) fn parse_group_expression(parser: &mut Parser) -> ParserResult<Box<Expression>> {
    parser.eat_token(TokenType::OpeningParenthesis)?;
    let expression_ref = parse_root_expression(parser)?;
    parser.eat_token(TokenType::ClosingParenthesis)?;

    Ok(expression_ref)
}

///
/// IdentifierExpression
///  : IDENTIFIER
///  ;
///
pub(super) fn parse_identifier_expression(parser: &mut Parser) -> ParserResult<Box<Expression>> {
    let identifier_token = parser.eat_token(TokenType::Identifier)?;
    let identifier_value = identifier_token.text(parser.source);

    Ok(Box::new(Expression::Identifier {
        name: String::from(identifier_value),
    }))
}

///
/// ThisExpression
///  : this
///  ;
///
pub(super) fn parse_this_expression(parser: &mut Parser) -> ParserResult<Box<Expression>> {
    parser.eat_token(TokenType::ThisKeyword)?;
    Ok(Box::new(Expression::This {}))
}

///
/// SuperExpression
///  : super
///  ;
///
pub(super) fn parse_super_expression(parser: &mut Parser) -> ParserResult<Box<Expression>> {
    parser.eat_token(TokenType::SuperKeyword)?;
    Ok(Box::new(Expression::Super {}))
}

///
/// NewExpression
///  : new MemberExpression Arguments
///  ;
///
pub(super) fn parse_new_expression(parser: &mut Parser) -> ParserResult<Box<Expression>> {
    parser.eat_token(TokenType::NewKeyword)?;

    let callee = parse_member_expression(parser)?;

    let arguments = parse_arguments(parser)?;

    Ok(Box::new(Expression::New { callee, arguments }))
}
