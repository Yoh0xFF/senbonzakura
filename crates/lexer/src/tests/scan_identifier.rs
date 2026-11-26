use super::{Kind, lex};

#[test]
fn identifiers() {
  let inputs =
    ["x", "_", "__", "_x1", "a1_b2", "camelCase", "snake_case", "ünïcödé", "日本語", "x٣", "𝑥"];
  for text in inputs {
    let lexed = lex(text);

    let token = lexed.single();
    assert_eq!(token.kind, Kind::Identifier, "{text:?}");
    assert_eq!(lexed.text(token), text);
    assert_eq!(token.flags, 0);
    assert_eq!(lexed.messages(), Vec::<&str>::new());
  }
}

#[test]
fn keywords() {
  for &keyword in Kind::ALL.iter().filter(|kind| kind.is_keyword()) {
    let spelling = keyword.spelling().unwrap();
    let lexed = lex(spelling);

    let token = lexed.single();
    assert_eq!(token.kind, keyword);
    assert_eq!(lexed.text(token), spelling);
    assert_eq!(lexed.messages(), Vec::<&str>::new());
  }
}

#[test]
fn words_that_only_resemble_keywords_are_identifiers() {
  for text in ["If", "TRUE", "Fn", "iff", "fnord", "in_", "_if", "if2", "returns", "elseif"] {
    assert_eq!(lex(text).single().kind, Kind::Identifier, "{text:?}");
  }
}

#[test]
fn identifier_ends_at_the_first_character_that_cannot_continue_it() {
  let lexed = lex("var x\t= y1\"s\"'c'`m`");

  assert_eq!(
    lexed.kinds(),
    [
      Kind::Var,
      Kind::Identifier,
      Kind::Eq,
      Kind::Identifier,
      Kind::StringLiteral,
      Kind::CharLiteral,
      Kind::StringLiteral,
    ]
  );
  assert_eq!(lexed.texts(), ["var", "x", "=", "y1", "\"s\"", "'c'", "`m`"]);
}

#[test]
fn digits_cannot_start_an_identifier() {
  let lexed = lex("1abc");

  assert_eq!(lexed.kinds(), [Kind::IntLiteral]);
  assert_eq!(lexed.messages(), ["invalid suffix on number literal"]);
}

#[test]
fn other_symbols_cannot_start_an_identifier() {
  // U+2160 ROMAN NUMERAL ONE is a letter-like number, not a letter; the emoji is a symbol.
  let lexed = lex("Ⅰ 😀 $x");

  assert_eq!(lexed.kinds(), [Kind::Error, Kind::Error, Kind::Error, Kind::Identifier]);
}
