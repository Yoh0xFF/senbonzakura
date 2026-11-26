//! Scans identifiers and keywords. An identifier starts with a letter or `_` and continues with
//! letters, digits and `_`, from any script. A keyword is an identifier whose text is the
//! [spelling](Kind::spelling) of a keyword kind; matching is case-sensitive.

use std::collections::HashMap;
use std::sync::OnceLock;

use super::cursor::Cursor;
use super::{Kind, Token};

fn keywords() -> &'static HashMap<&'static str, Kind> {
  static KEYWORDS: OnceLock<HashMap<&'static str, Kind>> = OnceLock::new();
  KEYWORDS.get_or_init(|| {
    Kind::ALL
      .iter()
      .filter(|kind| kind.is_keyword())
      .map(|&kind| (kind.spelling().expect("a keyword has a spelling"), kind))
      .collect()
  })
}

/// Scans the word at the cursor, which must be on its start.
pub(crate) fn identifier_or_keyword(c: &mut Cursor) -> Token {
  debug_assert!(c.at_identifier_start(), "not on an identifier start at {}", c.pos());
  let start = c.pos();
  c.skip_identifier_chars();
  let kind = keywords().get(c.text(start, c.pos())).copied().unwrap_or(Kind::Identifier);
  c.token(kind, start, 0)
}
