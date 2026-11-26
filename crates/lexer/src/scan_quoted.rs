//! Scans literals between quotes, `"..."`, `` `...` `` and `'...'`, and the escapes they share.

use super::cursor::Cursor;
use super::{Kind, Token};

/// Scans a `"..."` string, which must end on the line it starts.
pub(crate) fn string(c: &mut Cursor) -> Token {
  debug_assert!(c.peek() == Some(b'"'), "not on '\"' at {}", c.pos());
  let start = c.pos();
  c.bump(); // opening '"'
  let mut flags = 0;
  loop {
    match c.peek() {
      None | Some(b'\n' | b'\r') => {
        c.report(start, "unterminated string literal");
        flags |= Token::UNTERMINATED;
        break;
      }
      Some(b'"') => {
        c.bump();
        break;
      }
      Some(b'\\') => {
        flags |= Token::HAS_ESCAPES;
        escape(c, false);
      }
      Some(_) => c.advance(),
    }
  }
  c.token(Kind::StringLiteral, start, flags)
}

/// Scans a `` `...` `` string, which may span lines. A backslash before a line break joins the
/// two lines. There is no `${...}` interpolation; it is plain text.
pub(crate) fn multiline(c: &mut Cursor) -> Token {
  debug_assert!(c.peek() == Some(b'`'), "not on '`' at {}", c.pos());
  let start = c.pos();
  c.bump(); // opening '`'
  let mut flags = 0;
  loop {
    match c.peek() {
      None => {
        // Point at the opening backtick: the token itself runs to the end of the file.
        c.report_range(start, start + 1, "unterminated multi-line string literal");
        flags |= Token::UNTERMINATED;
        break;
      }
      Some(b'`') => {
        c.bump();
        break;
      }
      Some(b'\\') => {
        flags |= Token::HAS_ESCAPES;
        escape(c, true);
      }
      Some(_) => c.advance(),
    }
  }
  c.token(Kind::StringLiteral, start, flags)
}

/// Scans a `'...'` literal, which must hold exactly one character or escape.
pub(crate) fn character(c: &mut Cursor) -> Token {
  debug_assert!(c.peek() == Some(b'\''), "not on '\\'' at {}", c.pos());
  let start = c.pos();
  c.bump(); // opening '\''
  let mut flags = 0;
  let mut count = 0;
  loop {
    match c.peek() {
      None | Some(b'\n' | b'\r') => {
        c.report(start, "unterminated character literal");
        flags |= Token::UNTERMINATED;
        break;
      }
      Some(b'\'') => {
        c.bump();
        if count == 0 {
          c.report(start, "empty character literal");
        } else if count > 1 {
          c.report(start, "character literal must contain exactly one character");
        }
        break;
      }
      Some(b'\\') => {
        flags |= Token::HAS_ESCAPES;
        escape(c, false);
      }
      Some(_) => c.advance(),
    }
    count += 1;
  }
  c.token(Kind::CharLiteral, start, flags)
}

/// Consumes one escape, starting at the backslash: `\n \t \r \0 \\ \" \' \``, or a unicode escape
/// of 1 to 6 hex digits in braces. With `multiline`, a backslash before a line break continues
/// the line.
fn escape(c: &mut Cursor, multiline: bool) {
  debug_assert!(c.peek() == Some(b'\\'), "not on '\\' at {}", c.pos());
  let start = c.pos();
  c.bump(); // '\\'
  match c.peek() {
    Some(b'n' | b't' | b'r' | b'0' | b'\\' | b'"' | b'\'' | b'`') => c.bump(),
    Some(b'u') => unicode_escape(c, start),
    None => {} // the caller reports the unterminated literal
    Some(b'\n' | b'\r') => {
      if multiline {
        c.skip_line_break();
      } // otherwise the caller reports the unterminated literal
    }
    Some(_) => {
      c.advance();
      c.report(start, "unknown escape sequence");
    }
  }
}

fn unicode_escape(c: &mut Cursor, start: usize) {
  debug_assert!(c.peek() == Some(b'u') && c.pos() == start + 1, "not on the 'u' of '\\u'");
  c.bump(); // 'u'
  if c.peek() != Some(b'{') {
    c.report(start, "expected '{' after '\\u'");
    return;
  }
  c.bump();

  let mut value: u32 = 0;
  let mut count = 0;
  while let Some(digit) = c.peek_digit(16) {
    if count < 6 {
      value = value * 16 + digit;
    }
    count += 1;
    c.bump();
  }

  if c.peek() != Some(b'}') {
    c.report(start, "unterminated unicode escape, expected '}'");
    return;
  }
  c.bump();

  if count == 0 || count > 6 {
    c.report(start, "unicode escape must have 1 to 6 hex digits");
  } else if char::from_u32(value).is_none() {
    c.report(start, "unicode escape is not a valid code point");
  }
}
