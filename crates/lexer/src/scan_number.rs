//! Scans number literals: `123`, `1_000`, `0x1F`, `0o17`, `0b1010`, `1.5`, `1e10`, `1.5e-3`.
//!
//! A `.` only starts a fraction when a digit follows, so `1.foo` and `1..2` leave it for the next
//! token.

use super::cursor::Cursor;
use super::{Kind, Token};

/// Scans the number at the cursor, which must be on a decimal digit.
pub(crate) fn literal(c: &mut Cursor) -> Token {
  debug_assert!(c.peek_digit(10).is_some(), "not on a decimal digit at {}", c.pos());
  let start = c.pos();
  if c.peek() == Some(b'0') {
    match c.peek_at(1) {
      Some(b'x' | b'X') => return prefixed_integer(c, start, 16, "hexadecimal"),
      Some(b'o' | b'O') => return prefixed_integer(c, start, 8, "octal"),
      Some(b'b' | b'B') => return prefixed_integer(c, start, 2, "binary"),
      _ => {}
    }
  }

  let mut kind = Kind::IntLiteral;
  digits(c, 10);

  if c.peek() == Some(b'.') && c.peek_digit_at(1, 10).is_some() {
    kind = Kind::FloatLiteral;
    c.bump(); // '.'
    digits(c, 10);
  }

  let mut missing_exponent_digits = false;
  if matches!(c.peek(), Some(b'e' | b'E')) {
    kind = Kind::FloatLiteral;
    c.bump(); // 'e'
    if matches!(c.peek(), Some(b'+' | b'-')) {
      c.bump();
    }
    missing_exponent_digits = digits(c, 10) == 0;
  }

  let suffix_start = c.pos();
  if c.skip_identifier_chars() {
    c.report(suffix_start, "invalid suffix on number literal");
  } else if missing_exponent_digits {
    c.report(start, "expected digits in exponent");
  }
  c.token(kind, start, 0)
}

fn prefixed_integer(c: &mut Cursor, start: usize, radix: u32, name: &str) -> Token {
  debug_assert!(c.pos() == start && c.peek() == Some(b'0'), "not on a radix prefix");
  c.bump_n(2); // '0x', '0o' or '0b'
  let count = digits(c, radix);

  let suffix_start = c.pos();
  let has_suffix = c.skip_identifier_chars();
  if count == 0 {
    let prefix = c.text(start, start + 2);
    c.report(start, format!("expected {name} digits after '{prefix}'"));
  } else if has_suffix {
    c.report(suffix_start, format!("invalid {name} digit or suffix"));
  }
  c.token(Kind::IntLiteral, start, 0)
}

/// Consumes digits of `radix` and `_` separators, and returns how many digits.
fn digits(c: &mut Cursor, radix: u32) -> usize {
  let mut count = 0;
  loop {
    if c.peek() == Some(b'_') {
      c.bump();
    } else if c.peek_digit(radix).is_some() {
      c.bump();
      count += 1;
    } else {
      return count;
    }
  }
}
