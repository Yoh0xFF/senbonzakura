use super::{Kind, lex};

#[test]
fn integers() {
  let inputs = [
    "0",
    "7",
    "123",
    "007",
    "1_000",
    "1__0",
    "0x1F",
    "0XfF",
    "0xdead_BEEF",
    "0o17",
    "0O7",
    "0b1010",
    "0B1",
    "0b_1",
  ];
  for text in inputs {
    assert_valid(text, Kind::IntLiteral);
  }
}

#[test]
fn floats() {
  for text in ["1.5", "0.0", "1_0.5_0", "1e10", "1E10", "1e+4", "1e-3", "1.5e-3", "2E+4"] {
    assert_valid(text, Kind::FloatLiteral);
  }
}

#[test]
fn dot_without_digit_after_it_is_not_part_of_the_number() {
  for (input, number) in [("1.foo", "1"), ("1..2", "1"), ("1.", "1"), ("0x1.5", "0x1")] {
    let lexed = lex(input);

    assert_eq!(lexed.tokens[0].kind, Kind::IntLiteral, "{input:?}");
    assert_eq!(lexed.texts()[0], number);
  }
}

#[test]
fn second_dot_ends_the_float() {
  let lexed = lex("1.5.6");

  assert_eq!(lexed.kinds(), [Kind::FloatLiteral, Kind::Dot, Kind::IntLiteral]);
  assert_eq!(lexed.texts(), ["1.5", ".", "6"]);
}

#[test]
fn malformed_prefixed_integers() {
  let cases = [
    ("0x", "0x", "expected hexadecimal digits after '0x'"),
    ("0X", "0X", "expected hexadecimal digits after '0X'"),
    ("0x_", "0x_", "expected hexadecimal digits after '0x'"),
    ("0xg", "0xg", "expected hexadecimal digits after '0x'"),
    ("0o", "0o", "expected octal digits after '0o'"),
    ("0b", "0b", "expected binary digits after '0b'"),
    ("0b102", "2", "invalid binary digit or suffix"),
    ("0o78", "8", "invalid octal digit or suffix"),
    ("0x1g", "g", "invalid hexadecimal digit or suffix"),
  ];
  for (input, reported, message) in cases {
    assert_malformed(input, Kind::IntLiteral, reported, message);
  }
}

#[test]
fn malformed_decimal_numbers() {
  let cases = [
    ("12abc", Kind::IntLiteral, "abc", "invalid suffix on number literal"),
    ("1_x", Kind::IntLiteral, "x", "invalid suffix on number literal"),
    ("1.5f", Kind::FloatLiteral, "f", "invalid suffix on number literal"),
    ("1e", Kind::FloatLiteral, "1e", "expected digits in exponent"),
    ("1e+", Kind::FloatLiteral, "1e+", "expected digits in exponent"),
    ("1.5E-", Kind::FloatLiteral, "1.5E-", "expected digits in exponent"),
    ("1ex", Kind::FloatLiteral, "x", "invalid suffix on number literal"),
  ];
  for (input, kind, reported, message) in cases {
    assert_malformed(input, kind, reported, message);
  }
}

#[test]
fn only_ascii_digits_count() {
  // U+0663 is ARABIC-INDIC DIGIT THREE, a decimal digit of another script.
  let lexed = lex("1٣ ٣");

  assert_eq!(lexed.kinds(), [Kind::IntLiteral, Kind::Error]);
  assert_eq!(lexed.texts(), ["1٣", "٣"]);
  assert_eq!(lexed.messages(), ["invalid suffix on number literal", "unexpected character"]);
  assert_eq!(lexed.diagnostic_texts(), ["٣", "٣"]);
}

fn assert_valid(text: &str, kind: Kind) {
  let lexed = lex(text);

  let token = lexed.single();
  assert_eq!(token.kind, kind, "{text:?}");
  assert_eq!(lexed.text(token), text);
  assert_eq!(token.flags, 0);
  assert_eq!(lexed.messages(), Vec::<&str>::new(), "{text:?}");
}

/// Malformed literals keep their kind and cover the whole literal; the diagnostic says what's
/// wrong.
fn assert_malformed(input: &str, kind: Kind, reported: &str, message: &str) {
  let lexed = lex(input);

  let token = lexed.single();
  assert_eq!(token.kind, kind, "{input:?}");
  assert_eq!(lexed.text(token), input);
  assert_eq!(lexed.messages(), [message], "{input:?}");
  assert_eq!(lexed.diagnostic_texts(), [reported], "{input:?}");
}
