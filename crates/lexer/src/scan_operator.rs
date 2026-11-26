//! Scans operators and punctuation by longest match, so `<=` is one token rather than `<` then
//! `=`. The candidates are the operator kinds.

use std::sync::OnceLock;

use super::cursor::Cursor;
use super::{Kind, Token};

/// The operators sorted longest first, so the first spelling that matches is the longest one.
fn operators() -> &'static [(&'static str, Kind)] {
    static OPERATORS: OnceLock<Vec<(&'static str, Kind)>> = OnceLock::new();
    OPERATORS.get_or_init(|| {
        let mut operators: Vec<_> = Kind::ALL
            .iter()
            .filter(|kind| kind.is_operator())
            .map(|&kind| (kind.spelling().expect("an operator has a spelling"), kind))
            .collect();
        operators.sort_by_key(|(spelling, _)| std::cmp::Reverse(spelling.len()));
        operators
    })
}

/// Scans the operator at the cursor, or returns `None` and consumes nothing if none starts there.
pub(crate) fn operator(c: &mut Cursor) -> Option<Token> {
    let first = c.peek()?;
    let &(spelling, kind) = operators()
        .iter()
        .find(|(spelling, _)| spelling.as_bytes()[0] == first && c.looking_at(spelling))?;
    let start = c.pos();
    c.bump_n(spelling.len());
    Some(c.token(kind, start, 0))
}
