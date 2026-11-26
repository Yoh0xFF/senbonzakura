use super::{Kind, SourceFile, Token};
use crate::LineCol;

#[test]
fn empty_text_has_one_line() {
  let file = SourceFile::new("test", "");

  assert_eq!(file.line_count(), 1);
  assert_eq!(file.locate(0), LineCol::new(1, 1));
}

#[test]
fn locate_handles_every_line_break_style() {
  let file = SourceFile::new("test", "ab\nc\r\nd\re");
  let cases = [
    (0, 1, 1), // 'a'
    (1, 1, 2), // 'b'
    (2, 1, 3), // '\n'
    (3, 2, 1), // 'c'
    (4, 2, 2), // '\r' of \r\n
    (5, 2, 3), // '\n' of \r\n
    (6, 3, 1), // 'd'
    (7, 3, 2), // lone '\r'
    (8, 4, 1), // 'e'
    (9, 4, 2), // end of text
  ];
  for (offset, line, column) in cases {
    assert_eq!(file.locate(offset), LineCol::new(line, column), "offset {offset}");
  }
}

#[test]
fn crlf_counts_as_one_line_break() {
  assert_eq!(SourceFile::new("test", "a\r\nb\r\nc").line_count(), 3);
}

#[test]
fn trailing_line_break_starts_an_empty_last_line() {
  let file = SourceFile::new("test", "a\n");

  assert_eq!(file.line_count(), 2);
  assert_eq!(file.locate(2), LineCol::new(2, 1));
}

#[test]
fn columns_count_characters() {
  let file = SourceFile::new("test", "😀x");

  assert_eq!(file.locate(4), LineCol::new(1, 2));
}

#[test]
#[should_panic(expected = "outside")]
fn locate_rejects_offsets_past_the_text() {
  SourceFile::new("test", "abc").locate(4);
}

#[test]
fn line_text_leaves_out_the_line_break() {
  let file = SourceFile::new("test", "one\ntwo\r\nthree\rfour");

  assert_eq!(lines(&file), ["one", "two", "three", "four"]);
}

#[test]
fn line_text_keeps_empty_lines() {
  let file = SourceFile::new("test", "\n\r\nx\n");

  assert_eq!(lines(&file), ["", "", "x", ""]);
}

#[test]
#[should_panic(expected = "outside")]
fn line_text_rejects_line_zero() {
  SourceFile::new("test", "a\nb").line_text(0);
}

#[test]
#[should_panic(expected = "outside")]
fn line_text_rejects_lines_past_the_text() {
  SourceFile::new("test", "a\nb").line_text(3);
}

fn lines(file: &SourceFile) -> Vec<&str> {
  (1..=file.line_count()).map(|line| file.line_text(line)).collect()
}

#[test]
fn text_of_token() {
  let file = SourceFile::new("test", "var x = 42");

  assert_eq!(file.text_of(&Token::new(Kind::IntLiteral, 8, 10, 0)), "42");
}

#[test]
fn accessors() {
  let file = SourceFile::new("main.sz", "abc");

  assert_eq!(file.path(), "main.sz");
  assert_eq!(file.text(), "abc");
  assert_eq!(file.len(), 3);
  assert_eq!(file.to_string(), "main.sz");
}

#[test]
fn line_col_prints_as_line_and_column() {
  assert_eq!(LineCol::new(3, 7).to_string(), "3:7");
}

#[test]
#[should_panic(expected = "1-based")]
fn line_col_rejects_line_zero() {
  LineCol::new(0, 1);
}

#[test]
#[should_panic(expected = "1-based")]
fn line_col_rejects_column_zero() {
  LineCol::new(1, 0);
}
