use std::collections::HashSet;

use super::{Kind, Token};

#[test]
fn eof_is_zero_width() {
  let eof = Token::eof(5, Token::NEWLINE_BEFORE);

  assert_eq!(eof, Token::new(Kind::Eof, 5, 5, Token::NEWLINE_BEFORE));
  assert_eq!(eof.len(), 0);
}

#[test]
fn length() {
  assert_eq!(Token::new(Kind::Identifier, 2, 5, 0).len(), 3);
}

#[test]
fn flags() {
  let token = Token::new(Kind::StringLiteral, 0, 4, Token::UNTERMINATED | Token::HAS_ESCAPES);

  assert!(token.has(Token::UNTERMINATED));
  assert!(token.has(Token::HAS_ESCAPES));
  assert!(!token.has(Token::NEWLINE_BEFORE));
  assert!(!token.has(Token::SPACE_BEFORE));
}

#[test]
fn flags_are_distinct_bits() {
  let flags = [Token::NEWLINE_BEFORE, Token::SPACE_BEFORE, Token::UNTERMINATED, Token::HAS_ESCAPES];
  let mut seen = 0;
  for flag in flags {
    assert_eq!(flag.count_ones(), 1);
    assert_eq!(seen & flag, 0);
    seen |= flag;
  }
}

#[test]
fn only_fixed_text_kinds_have_a_spelling() {
  assert_eq!(Kind::If.spelling(), Some("if"));
  assert_eq!(Kind::Identifier.spelling(), None);
  assert_eq!(Kind::StringLiteral.spelling(), None);
  assert_eq!(Kind::Eof.spelling(), None);
}

#[test]
fn categories() {
  assert!(Kind::If.is_keyword());
  assert!(!Kind::If.is_operator());
  assert!(Kind::Plus.is_operator());
  assert!(!Kind::Plus.is_keyword());
  assert!(!(Kind::Identifier.is_keyword() || Kind::Identifier.is_operator()));
  assert!(!(Kind::Eof.is_keyword() || Kind::Eof.is_operator()));
}

#[test]
fn every_kind_with_a_spelling_has_exactly_one_category() {
  for &kind in Kind::ALL {
    let has_spelling = kind.spelling().is_some();
    assert_eq!(has_spelling, kind.is_keyword() || kind.is_operator(), "{kind:?}");
    assert!(!(kind.is_keyword() && kind.is_operator()), "{kind:?}");
  }
}

#[test]
fn keywords_are_words_and_operators_are_not() {
  for &kind in Kind::ALL {
    if let Some(spelling) = kind.spelling() {
      let word = spelling.starts_with(|c: char| c.is_alphabetic());
      assert_eq!(kind.is_keyword(), word, "{kind:?}");
    }
  }
}

#[test]
fn spellings_are_unique() {
  let spellings: Vec<_> = Kind::ALL.iter().filter_map(|kind| kind.spelling()).collect();
  let unique: HashSet<_> = spellings.iter().collect();

  assert_eq!(spellings.len(), unique.len(), "{spellings:?}");
}

#[test]
#[should_panic(expected = "invalid token range")]
fn rejects_invalid_ranges() {
  Token::new(Kind::Error, 3, 2, 0);
}
