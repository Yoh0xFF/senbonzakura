//! Parses literal tokens into nodes holding their values, using the `decode` module.
//!
//! Each function consumes one token, which must be of the kind it names. Problems in the
//! literal's syntax were reported by the lexer; only values out of range are reported here.

use super::cursor::TokenCursor;
use super::decode;
use senbonzakura_ast::{Expr, ExprKind};
use senbonzakura_lexer::{Kind, Token};

pub(crate) fn integer(c: &mut TokenCursor) -> Expr {
  debug_assert!(c.at(Kind::IntLiteral));
  let token = c.advance();
  let value = decode::integer(c.text(token)).unwrap_or_else(|| {
    c.report_token(token, "integer literal is too large");
    0
  });
  Expr::new(ExprKind::IntLiteral(value), token.start, token.end)
}

pub(crate) fn floating(c: &mut TokenCursor) -> Expr {
  debug_assert!(c.at(Kind::FloatLiteral));
  let token = c.advance();
  let mut value = decode::floating(c.text(token));
  if value.is_infinite() {
    c.report_token(token, "float literal is out of range");
    value = 0.0;
  }
  Expr::new(ExprKind::FloatLiteral(value), token.start, token.end)
}

pub(crate) fn string(c: &mut TokenCursor) -> Expr {
  debug_assert!(c.at(Kind::StringLiteral));
  let token = c.advance();
  let value = decode::string(c.text(token), !token.has(Token::UNTERMINATED));
  Expr::new(ExprKind::StringLiteral(value), token.start, token.end)
}

pub(crate) fn character(c: &mut TokenCursor) -> Expr {
  debug_assert!(c.at(Kind::CharLiteral));
  let token = c.advance();
  let value = decode::character(c.text(token), !token.has(Token::UNTERMINATED));
  Expr::new(ExprKind::CharLiteral(value), token.start, token.end)
}

pub(crate) fn boolean(c: &mut TokenCursor) -> Expr {
  debug_assert!(c.at(Kind::True) || c.at(Kind::False));
  let token = c.advance();
  Expr::new(ExprKind::BoolLiteral(token.kind == Kind::True), token.start, token.end)
}
