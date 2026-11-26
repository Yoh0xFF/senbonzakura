use super::{Diagnostic, Kind, lex};

#[test]
fn comments_produce_no_tokens() {
  let inputs = [
    "// only a comment",
    "//",
    "// ends the line\n",
    "/* block */",
    "/**/",
    "/***/",
    "/* spans\nlines */",
    "/* outer /* inner */ still outer */",
    "/*/ not closed by its own slash */",
    "// a\n// b\r\n/* c */ // d",
  ];
  for input in inputs {
    let lexed = lex(input);

    assert_eq!(lexed.kinds(), [], "{input:?}");
    assert_eq!(lexed.diagnostics, [], "{input:?}");
  }
}

#[test]
fn line_comment_ends_at_the_line_break() {
  let lexed = lex("1 // two\n3 // four\r\n5 // six\r7");

  assert_eq!(lexed.texts(), ["1", "3", "5", "7"]);
}

#[test]
fn block_comment_can_sit_between_tokens() {
  let lexed = lex("a/*x*/b /* y\n */ c");

  assert_eq!(lexed.texts(), ["a", "b", "c"]);
}

#[test]
fn nested_block_comments_end_at_their_own_close() {
  let lexed = lex("/* a /* b */ c */ x /* d */ y");

  assert_eq!(lexed.texts(), ["x", "y"]);
}

#[test]
fn comments_hold_anything_without_diagnostics() {
  let lexed = lex("// \"no close 'x` # 😀 /*\n/* \"also\" '' # 😀 // */ 1");

  assert_eq!(lexed.texts(), ["1"]);
  assert_eq!(lexed.diagnostics, []);
}

#[test]
fn slash_outside_a_comment_is_still_an_operator() {
  let lexed = lex("a / b /= c/d */ e");

  assert_eq!(
    lexed.kinds(),
    [
      Kind::Identifier,
      Kind::Slash,
      Kind::Identifier,
      Kind::SlashEq,
      Kind::Identifier,
      Kind::Slash,
      Kind::Identifier,
      Kind::Star,
      Kind::Slash,
      Kind::Identifier,
    ]
  );
}

#[test]
fn comment_markers_inside_strings_are_text() {
  let lexed = lex("\"// not a comment\" '/' `/* nor this */`");

  assert_eq!(lexed.kinds(), [Kind::StringLiteral, Kind::CharLiteral, Kind::StringLiteral]);
}

#[test]
fn unterminated_block_comment_is_reported_at_its_opening() {
  for input in ["/* x", "/*", "/* /* */ x", "/* x */ /* y"] {
    let lexed = lex(input);

    assert_eq!(lexed.kinds(), [], "{input:?}");
    assert_eq!(lexed.messages(), ["unterminated block comment"], "{input:?}");
    assert_eq!(lexed.diagnostic_texts(), ["/*"], "{input:?}");
  }
}

#[test]
fn unterminated_nested_comment_points_at_the_outermost_opening() {
  let lexed = lex("1 /* a /* b */");

  assert_eq!(lexed.texts(), ["1"]);
  assert_eq!(lexed.diagnostics, [Diagnostic::new(2, 4, "unterminated block comment")]);
}
