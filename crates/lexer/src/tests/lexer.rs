use super::{Diagnostic, Kind, Lexer, SourceFile, Token, lex};
use crate::LineCol;

#[test]
fn empty_input_is_just_eof() {
  let lexed = lex("");

  assert_eq!(lexed.kinds(), []);
  assert_eq!(lexed.eof(), Token::new(Kind::Eof, 0, 0, 0));
  assert_eq!(lexed.diagnostics, []);
}

#[test]
fn whitespace_is_skipped() {
  for whitespace in [" ", "\t", "\n", "\r", "\r\n", " \t\r\n \n"] {
    let lexed = lex(whitespace);

    assert_eq!(lexed.kinds(), [], "{whitespace:?}");
    assert_eq!(lexed.eof().start, whitespace.len());
  }
}

#[test]
fn eof_is_returned_forever_once_input_is_used_up() {
  let source = SourceFile::new("test", "1");
  let mut lexer = Lexer::new(&source);
  lexer.next_token();

  let eof = lexer.next_token();
  assert_eq!(eof, Token::new(Kind::Eof, 1, 1, 0));
  assert_eq!(lexer.next_token(), eof);
  assert_eq!(lexer.next_token(), eof);
}

#[test]
fn second_token_records_what_came_between() {
  let cases = [
    ("a b", false),
    ("a\tb", false),
    ("a/*x*/b", false),
    ("a\nb", true),
    ("a\r\nb", true),
    ("a\rb", true),
    ("a/*\n*/b", true),
    ("a // c\nb", true),
    ("a /* x */ \n b", true),
  ];
  for (input, newline) in cases {
    let second = lex(input).tokens[1];

    assert!(second.has(Token::SPACE_BEFORE), "space or comment before the second token: {input:?}");
    assert_eq!(second.has(Token::NEWLINE_BEFORE), newline, "{input:?}");
  }
}

#[test]
fn adjacent_tokens_have_no_space_flags() {
  for input in ["ab", "a+b", "a(b", "1.5", "\"s\"x"] {
    for token in lex(input).tokens {
      assert_eq!(token.flags, 0, "{token:?}");
    }
  }
}

#[test]
fn first_token_records_what_came_before_it() {
  assert_eq!(lex(" a").tokens[0].flags, Token::SPACE_BEFORE);
  assert_eq!(lex("\na").tokens[0].flags, Token::SPACE_BEFORE | Token::NEWLINE_BEFORE);
  assert_eq!(lex("/**/a").tokens[0].flags, Token::SPACE_BEFORE);
}

#[test]
fn space_flags_combine_with_literal_flags() {
  let token = lex("x \"unterminated").tokens[1];

  assert!(token.has(Token::UNTERMINATED));
  assert!(token.has(Token::SPACE_BEFORE));
}

#[test]
fn eof_records_trailing_space_and_stays_the_same() {
  let source = SourceFile::new("test", "1 // done\n");
  let mut lexer = Lexer::new(&source);
  lexer.next_token();

  let eof = lexer.next_token();
  assert_eq!(eof, Token::new(Kind::Eof, 10, 10, Token::SPACE_BEFORE | Token::NEWLINE_BEFORE));
  assert_eq!(lexer.next_token(), eof);
  assert_eq!(lexer.next_token(), eof);
}

#[test]
fn tokens_cover_their_source_text() {
  let lexed = lex("  12\t\n0x1F \"s\" `m` 'c'  ");

  assert_eq!(
    lexed.kinds(),
    [
      Kind::IntLiteral,
      Kind::IntLiteral,
      Kind::StringLiteral,
      Kind::StringLiteral,
      Kind::CharLiteral,
    ]
  );
  assert_eq!(lexed.texts(), ["12", "0x1F", "\"s\"", "`m`", "'c'"]);
  assert_eq!(lexed.tokens[0].start, 2);
  assert_eq!(lexed.eof().start, lexed.source.len());
}

#[test]
fn unknown_character_becomes_error_token_and_lexing_continues() {
  let lexed = lex("1 # 2");

  assert_eq!(lexed.kinds(), [Kind::IntLiteral, Kind::Error, Kind::IntLiteral]);
  assert_eq!(lexed.texts(), ["1", "#", "2"]);
  assert_eq!(lexed.diagnostics, [Diagnostic::new(2, 3, "unexpected character")]);
}

#[test]
fn unknown_multi_byte_character_is_one_token() {
  let lexed = lex("😀");

  let token = lexed.single();
  assert_eq!(token.kind, Kind::Error);
  assert_eq!(token.len(), 4); // all four bytes of the emoji
}

#[test]
fn u_ffff_is_an_ordinary_character_not_end_of_input() {
  let lexed = lex("\u{FFFF} 1");

  assert_eq!(lexed.kinds(), [Kind::Error, Kind::IntLiteral]);
}

#[test]
fn diagnostics_can_be_located_in_the_source() {
  let lexed = lex("1\r\n  #");

  let diagnostic = &lexed.diagnostics[0];
  assert_eq!(lexed.source.locate(diagnostic.start), LineCol::new(2, 3));
}

#[test]
fn valid_input_has_no_diagnostics() {
  let lexed = lex("0 1.5 0b1 \"a\\n\" `x\ny` '\\''");

  assert!(lexed.diagnostics.is_empty(), "{:?}", lexed.messages());
}
