//! Parses expressions between delimiters. So far only `(...)`.

use super::cursor::TokenCursor;
use senbonzakura_ast::{Expr, ExprKind};
use senbonzakura_lexer::Kind;

/// Parses the parenthesized expression at the cursor, which must be on `(`. A missing closing
/// parenthesis is reported, and the group then ends where its contents do.
pub(crate) fn parenthesized(c: &mut TokenCursor) -> Expr {
  debug_assert!(c.at(Kind::LeftParen));
  let open = c.advance();
  let inner = super::expression(c);
  let end = if c.at(Kind::RightParen) {
    c.advance().end
  } else {
    c.report_found("expected ')'");
    inner.end
  };
  Expr::new(ExprKind::Group(Box::new(inner)), open.start, end)
}
