use super::{Diagnostic, Kind, Token, lex};

/// The character after the backslash for every simple escape.
const SIMPLE_ESCAPES: [&str; 8] = ["n", "t", "r", "0", "\\", "\"", "'", "`"];

/// Escaped text the lexer rejects, the part of it the diagnostic points at, and the message.
const INVALID_ESCAPES: [(&str, &str, &str); 10] = [
  ("\\q", "\\q", "unknown escape sequence"),
  ("\\😀", "\\😀", "unknown escape sequence"),
  ("\\u41", "\\u", "expected '{' after '\\u'"),
  ("\\u{}", "\\u{}", "unicode escape must have 1 to 6 hex digits"),
  ("\\u{1234567}", "\\u{1234567}", "unicode escape must have 1 to 6 hex digits"),
  ("\\u{110000}", "\\u{110000}", "unicode escape is not a valid code point"),
  ("\\u{D800}", "\\u{D800}", "unicode escape is not a valid code point"),
  ("\\u{DFFF}", "\\u{DFFF}", "unicode escape is not a valid code point"),
  ("\\u{12", "\\u{12", "unterminated unicode escape, expected '}'"),
  ("\\u{12x}", "\\u{12", "unterminated unicode escape, expected '}'"),
];

mod strings {
  use super::*;

  #[test]
  fn plain() {
    for text in ["\"\"", "\"hi\"", "\"a b\"", "\"😀\"", "\"`'\"", "\"${x}\""] {
      let token = assert_valid(text, Kind::StringLiteral);
      assert_eq!(token.flags, 0);
    }
  }

  #[test]
  fn simple_escape() {
    for escaped in SIMPLE_ESCAPES {
      let token = assert_valid(&format!("\"\\{escaped}\""), Kind::StringLiteral);
      assert_eq!(token.flags, Token::HAS_ESCAPES);
    }
  }

  #[test]
  fn unicode_escape() {
    for hex in ["0", "41", "e9", "1F600", "10FFFF", "000041"] {
      let token = assert_valid(&format!("\"\\u{{{hex}}}\""), Kind::StringLiteral);
      assert_eq!(token.flags, Token::HAS_ESCAPES);
    }
  }

  #[test]
  fn invalid_escape() {
    for (escape, reported, message) in INVALID_ESCAPES {
      // 'z' after the escape: a hex letter there would extend an unterminated unicode escape.
      assert_invalid_escape(&format!("\"a{escape}z\""), Kind::StringLiteral, reported, message);
    }
  }

  #[test]
  fn line_break_ends_an_unterminated_string() {
    for line_break in ["\n", "\r", "\r\n"] {
      let lexed = lex(&format!("\"abc{line_break}1"));

      assert_eq!(lexed.kinds(), [Kind::StringLiteral, Kind::IntLiteral]);
      assert_eq!(lexed.texts(), ["\"abc", "1"]);
      assert_eq!(lexed.tokens[0].flags, Token::UNTERMINATED);
      assert_eq!(lexed.diagnostics, [Diagnostic::new(0, 4, "unterminated string literal")]);
    }
  }

  #[test]
  fn end_of_input_ends_an_unterminated_string() {
    let lexed = lex("\"abc");

    let token = lexed.single();
    assert_eq!(lexed.text(token), "\"abc");
    assert_eq!(token.flags, Token::UNTERMINATED);
    assert_eq!(lexed.messages(), ["unterminated string literal"]);
  }

  #[test]
  fn backslash_at_end_of_input() {
    let lexed = lex("\"abc\\");

    let token = lexed.single();
    assert_eq!(lexed.text(token), "\"abc\\");
    assert_eq!(token.flags, Token::UNTERMINATED | Token::HAS_ESCAPES);
    assert_eq!(lexed.messages(), ["unterminated string literal"]);
  }

  #[test]
  fn backslash_before_line_break_does_not_continue_the_line() {
    let lexed = lex("\"a\\\nb\"");

    let first = lexed.tokens[0];
    assert_eq!(lexed.text(first), "\"a\\");
    assert!(first.has(Token::UNTERMINATED));
    assert_eq!(lexed.messages()[0], "unterminated string literal");
  }
}

mod multiline_strings {
  use super::*;

  #[test]
  fn plain() {
    for text in ["``", "`hi`", "`a\nb`", "`a\r\nb\rc`", "`\"'`", "`${x}`", "`😀`", "`\n\n`"] {
      let token = assert_valid(text, Kind::StringLiteral);
      assert_eq!(token.flags, 0);
    }
  }

  #[test]
  fn simple_escape() {
    for escaped in SIMPLE_ESCAPES {
      let token = assert_valid(&format!("`\\{escaped}`"), Kind::StringLiteral);
      assert_eq!(token.flags, Token::HAS_ESCAPES);
    }
  }

  #[test]
  fn unicode_escape() {
    assert_valid("`\\u{1F600}`", Kind::StringLiteral);
  }

  #[test]
  fn backslash_before_line_break_continues_the_line() {
    for line_break in ["\n", "\r", "\r\n"] {
      let token = assert_valid(&format!("`a\\{line_break}b`"), Kind::StringLiteral);
      assert_eq!(token.flags, Token::HAS_ESCAPES);
    }
  }

  #[test]
  fn invalid_escape() {
    for (escape, reported, message) in INVALID_ESCAPES {
      assert_invalid_escape(&format!("`a{escape}z`"), Kind::StringLiteral, reported, message);
    }
  }

  #[test]
  fn unterminated_runs_to_end_of_input_and_points_at_the_opening_backtick() {
    let lexed = lex("1 `abc\ndef");

    assert_eq!(lexed.kinds(), [Kind::IntLiteral, Kind::StringLiteral]);
    assert_eq!(lexed.texts(), ["1", "`abc\ndef"]);
    assert_eq!(lexed.tokens[1].flags, Token::UNTERMINATED | Token::SPACE_BEFORE);
    assert_eq!(
      lexed.diagnostics,
      [Diagnostic::new(2, 3, "unterminated multi-line string literal")]
    );
  }

  #[test]
  fn escaped_backtick_does_not_close_the_string() {
    let lexed = lex("`a\\`");

    assert!(lexed.single().has(Token::UNTERMINATED));
  }
}

mod characters {
  use super::*;

  #[test]
  fn valid() {
    for text in ["'a'", "' '", "'\"'", "'`'", "'😀'", "'\\''", "'\\n'", "'\\u{41}'"] {
      assert_valid(text, Kind::CharLiteral);
    }
  }

  #[test]
  fn simple_escape() {
    for escaped in SIMPLE_ESCAPES {
      let token = assert_valid(&format!("'\\{escaped}'"), Kind::CharLiteral);
      assert_eq!(token.flags, Token::HAS_ESCAPES);
    }
  }

  #[test]
  fn empty() {
    let lexed = lex("''");

    assert_eq!(lexed.single().kind, Kind::CharLiteral);
    assert_eq!(lexed.diagnostics, [Diagnostic::new(0, 2, "empty character literal")]);
  }

  #[test]
  fn more_than_one_character() {
    for text in ["'ab'", "'\\n\\t'", "'😀😀'"] {
      let lexed = lex(text);

      assert_eq!(lexed.text(lexed.single()), text);
      assert_eq!(lexed.messages(), ["character literal must contain exactly one character"]);
      assert_eq!(lexed.diagnostic_texts(), [text]);
    }
  }

  #[test]
  fn invalid_escape() {
    for (escape, reported, message) in INVALID_ESCAPES {
      let lexed = lex(&format!("'{escape}'"));

      assert_eq!(lexed.tokens[0].kind, Kind::CharLiteral);
      assert_eq!(lexed.messages()[0], message, "{escape:?}");
      assert_eq!(lexed.diagnostic_texts()[0], reported);
    }
  }

  #[test]
  fn unterminated() {
    for input in ["'", "'x", "'x\n1", "'x\r\n1"] {
      let lexed = lex(input);

      let token = lexed.tokens[0];
      assert_eq!(token.kind, Kind::CharLiteral);
      assert_eq!(lexed.text(token), input.lines().next().unwrap_or(""));
      assert!(token.has(Token::UNTERMINATED));
      assert_eq!(lexed.messages(), ["unterminated character literal"]);
    }
  }
}

fn assert_valid(text: &str, kind: Kind) -> Token {
  let lexed = lex(text);

  let token = lexed.single();
  assert_eq!(token.kind, kind, "{text:?}");
  assert_eq!(lexed.text(token), text);
  assert_eq!(lexed.messages(), Vec::<&str>::new(), "{text:?}");
  token
}

/// A bad escape is reported, but the literal around it is still one complete token.
fn assert_invalid_escape(text: &str, kind: Kind, reported: &str, message: &str) {
  let lexed = lex(text);

  let token = lexed.single();
  assert_eq!(token.kind, kind, "{text:?}");
  assert_eq!(lexed.text(token), text);
  assert!(token.has(Token::HAS_ESCAPES));
  assert_eq!(lexed.messages(), [message], "{text:?}");
  assert_eq!(lexed.diagnostic_texts(), [reported]);
}
