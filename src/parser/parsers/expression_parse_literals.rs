use crate::ast::Expression;
use crate::lexer::TokenType;
use crate::parser::{Parser, ParserError, ParserResult};

///
/// Literal
///  : BooleanLiteral
///  : NilLiteral
///  : NumericLiteral
///  | StringLiteral
///  ;
///
pub(super) fn parse_literal_expression(parser: &mut Parser) -> ParserResult<Box<Expression>> {
    match parser.lookahead.token_type {
        TokenType::BooleanTrue => parse_boolean_literal_expression(parser),
        TokenType::BooleanFalse => parse_boolean_literal_expression(parser),
        TokenType::Nil => parse_nil_literal_expression(parser),
        TokenType::Number => parse_numeric_literal_expression(parser),
        TokenType::String => parse_string_literal_expression(parser),
        _ => Err(ParserError::SemanticError {
            message: String::from("Literal: unexpected literal production"),
        }),
    }
}

///
/// BooleanLiteral
///  : BOOLEAN
///  ;
///
pub(super) fn parse_boolean_literal_expression(
    parser: &mut Parser,
) -> ParserResult<Box<Expression>> {
    let token = parser.eat_any_of_token(&[TokenType::BooleanTrue, TokenType::BooleanFalse])?;
    let bool_value = token.token_type == TokenType::BooleanTrue;

    Ok(Box::new(Expression::BooleanLiteral { value: bool_value }))
}

///
/// NilLiteral
///  : NIL
///  ;
///
pub(super) fn parse_nil_literal_expression(parser: &mut Parser) -> ParserResult<Box<Expression>> {
    parser.eat_token(TokenType::Nil)?;

    Ok(Box::new(Expression::NilLiteral))
}

///
/// NumericLiteral
///  : NUMBER
///  ;
///
pub(super) fn parse_numeric_literal_expression(
    parser: &mut Parser,
) -> ParserResult<Box<Expression>> {
    let token = parser.eat_token(TokenType::Number)?;
    let token_value = token.text(parser.source);
    let token_value = token_value
        .parse::<f64>()
        .map_err(|_| ParserError::ParserError {
            message: format!("Invalid numeric literal: {token_value}"),
        })?;

    Ok(Box::new(Expression::NumericLiteral { value: token_value }))
}

///
/// StringLiteral
///  : STRING
///  ;
///
pub(super) fn parse_string_literal_expression(
    parser: &mut Parser,
) -> ParserResult<Box<Expression>> {
    let token = parser.eat_token(TokenType::String)?;
    let token_value = &parser.source[token.start.offset + 1..token.end.offset - 1];
    let value = decode_escape_sequences(token_value).ok_or_else(|| ParserError::ParserError {
        message: format!("Invalid escape sequence in string literal: {token_value}"),
    })?;

    Ok(Box::new(Expression::StringLiteral { value }))
}

///
/// Decodes the escape sequences of a string literal body (without quotes).
/// The lexer has already validated them, so `None` should not happen in practice.
///
fn decode_escape_sequences(raw: &str) -> Option<String> {
    let mut value = String::with_capacity(raw.len());
    let mut chars = raw.chars();

    while let Some(ch) = chars.next() {
        if ch != '\\' {
            value.push(ch);
            continue;
        }

        let decoded = match chars.next()? {
            'n' => '\n',
            't' => '\t',
            'r' => '\r',
            '0' => '\0',
            ch @ ('\\' | '\'' | '"') => ch,
            'x' => {
                let high = chars.next()?.to_digit(16)?;
                let low = chars.next()?.to_digit(16)?;
                char::from_u32(high * 16 + low).filter(char::is_ascii)?
            }
            _ => return None,
        };
        value.push(decoded);
    }

    Some(value)
}
