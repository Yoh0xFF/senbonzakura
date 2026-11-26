//! Turns the source text of literal tokens into values.
//!
//! The lexer has already checked literal syntax and reported what was wrong, so decoding never
//! fails: a malformed literal gets a best-effort value, and anything that cannot be decoded
//! becomes [`REPLACEMENT`]. Problems the lexer cannot see, like an integer too big for 64 bits,
//! are left to the caller to report.

pub(crate) const REPLACEMENT: char = '\u{FFFD}';

/// Decodes an integer like `1_000`, `0xFF`, `0o17` or `0b1010`, or returns `None` when it does not
/// fit in an `i64`. Stops at the first character that is not a digit, so an invalid suffix is
/// ignored.
pub(crate) fn integer(text: &str) -> Option<i64> {
  let bytes = text.as_bytes();
  let (radix, digits) = match bytes {
    [b'0', b'x' | b'X', rest @ ..] => (16, rest),
    [b'0', b'o' | b'O', rest @ ..] => (8, rest),
    [b'0', b'b' | b'B', rest @ ..] => (2, rest),
    _ => (10, bytes),
  };

  let mut value: i64 = 0;
  for &ch in digits {
    if ch == b'_' {
      continue;
    }
    let Some(digit) = digit(ch, radix) else { break };
    value = value.checked_mul(radix as i64)?.checked_add(digit as i64)?;
  }
  Some(value)
}

/// Decodes a float like `1.5`, `1_0.5` or `1.5e-3`. Returns infinity when it is too large for an
/// `f64`, and `0.0` when it is malformed.
pub(crate) fn floating(text: &str) -> f64 {
  let digits: String = text
    .chars()
    .filter(|&ch| ch != '_')
    .take_while(|&ch| ch.is_ascii_digit() || matches!(ch, '.' | 'e' | 'E' | '+' | '-'))
    .collect();
  // e.g. "1e" with no exponent digits does not parse
  digits.parse().unwrap_or(0.0)
}

/// Decodes a `"..."` or `` `...` `` string, applying its escapes.
///
/// `terminated` says whether the text ends with a closing quote.
pub(crate) fn string(text: &str, terminated: bool) -> String {
  debug_assert!(text.starts_with(['"', '`', '\'']), "no opening quote: {text}");
  let end = if terminated { text.len() - 1 } else { text.len() };
  unescape(&text[1..end])
}

/// Decodes a `'...'` literal into one character, or [`REPLACEMENT`] when it does not hold exactly
/// one.
pub(crate) fn character(text: &str, terminated: bool) -> char {
  let value = string(text, terminated);
  let mut chars = value.chars();
  match (chars.next(), chars.next()) {
    (Some(c), None) => c,
    _ => REPLACEMENT,
  }
}

fn unescape(s: &str) -> String {
  if !s.contains('\\') {
    return s.to_string();
  }
  let mut out = String::with_capacity(s.len());
  let mut chars = s.chars().peekable();
  while let Some(ch) = chars.next() {
    if ch != '\\' {
      out.push(ch);
      continue;
    }
    // None: a backslash at the end of an unterminated literal
    let Some(escaped) = chars.next() else { break };
    match escaped {
      'n' => out.push('\n'),
      't' => out.push('\t'),
      'r' => out.push('\r'),
      '0' => out.push('\0'),
      'u' => out.push(unicode_escape(&mut chars)),
      // A line continuation; \r\n counts as one line break.
      '\r' => {
        chars.next_if_eq(&'\n');
      }
      '\n' => {}
      // \\ \" \' \`, or an unknown escape kept as its character
      other => out.push(other),
    }
  }
  out
}

/// Decodes the braces and hex digits of a unicode escape, just after the `u`.
fn unicode_escape(chars: &mut std::iter::Peekable<std::str::Chars>) -> char {
  if chars.next_if_eq(&'{').is_none() {
    return REPLACEMENT;
  }

  let mut value: u32 = 0;
  let mut count = 0;
  while let Some(digit) = chars.peek().and_then(|&c| c.to_digit(16).filter(|_| c.is_ascii())) {
    if count < 6 {
      value = value * 16 + digit;
    }
    count += 1;
    chars.next();
  }

  if chars.next_if_eq(&'}').is_none() {
    return REPLACEMENT;
  }
  if !(1..=6).contains(&count) {
    return REPLACEMENT;
  }
  char::from_u32(value).unwrap_or(REPLACEMENT)
}

/// Returns the value of an ASCII digit in `radix`.
fn digit(ch: u8, radix: u32) -> Option<u32> {
  (ch as char).to_digit(radix).filter(|_| ch.is_ascii())
}
