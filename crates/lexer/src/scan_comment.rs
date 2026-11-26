//! Skips comments: `// ...` up to the end of the line, and `/* ... */`, which may span lines and
//! nests, so commenting out code that already holds a block comment works. Comments produce no
//! tokens; the [`Lexer`](super::Lexer) records them in the next token's flags.

use super::cursor::Cursor;

pub(crate) fn at_comment(c: &Cursor) -> bool {
  c.peek() == Some(b'/') && matches!(c.peek_at(1), Some(b'/' | b'*'))
}

/// Skips the comment at the cursor, which must be [`at_comment`] on one, and returns whether it
/// held a line break. A line comment stops before its line break, so it never holds one.
pub(crate) fn skip(c: &mut Cursor) -> bool {
  debug_assert!(at_comment(c), "not on a comment at {}", c.pos());
  if c.peek_at(1) == Some(b'/') {
    line(c);
    return false;
  }
  block(c)
}

fn line(c: &mut Cursor) {
  c.bump_n(2); // '//'
  while !c.is_at_end() && !matches!(c.peek(), Some(b'\n' | b'\r')) {
    c.advance();
  }
}

/// An unterminated comment is reported at its outermost opening, and runs to the end.
fn block(c: &mut Cursor) -> bool {
  let start = c.pos();
  c.bump_n(2); // '/*'
  let mut depth = 1;
  let mut line_break = false;
  while depth > 0 {
    if c.is_at_end() {
      c.report_range(start, start + 2, "unterminated block comment");
      break;
    }
    if c.looking_at("/*") {
      c.bump_n(2);
      depth += 1;
    } else if c.looking_at("*/") {
      c.bump_n(2);
      depth -= 1;
    } else {
      line_break |= matches!(c.peek(), Some(b'\n' | b'\r'));
      c.advance();
    }
  }
  line_break
}
