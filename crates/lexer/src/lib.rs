//! Turns source text into tokens.
//!
//! - Tokens hold byte offsets only. [`SourceFile`] maps them to text, and to lines and columns.
//! - Errors never panic. They are collected in [`Lexer::diagnostics`]. A malformed literal keeps
//!   its kind, and a character that starts no token becomes a [`Kind::Error`] token.
//! - Whitespace and comments produce no tokens. The token after them records them in its
//!   [`Token::SPACE_BEFORE`] and [`Token::NEWLINE_BEFORE`] flags.
//! - [`Lexer`] picks a scanner from the first character of each token. `Cursor` holds all mutable
//!   state, and the `scan_*` modules are stateless.

mod cursor;
mod scan_comment;
mod scan_identifier;
mod scan_number;
mod scan_operator;
mod scan_quoted;
mod source;
mod token;

#[cfg(test)]
mod tests;

pub use source::char_count;
pub use source::{LineCol, SourceFile};
pub use token::{Diagnostic, Kind, Token};

use cursor::Cursor;

/// Splits a [`SourceFile`] into [`Token`]s, one [`next_token`](Self::next_token) call at a time.
///
/// Lexical errors never panic. They are collected in [`diagnostics`](Self::diagnostics) and lexing
/// continues.
pub struct Lexer<'a> {
    source: &'a SourceFile,
    cursor: Cursor<'a>,
    /// The EOF token once reached, so every later call returns it with the same flags.
    eof: Option<Token>,
}

impl<'a> Lexer<'a> {
    pub fn new(source: &'a SourceFile) -> Lexer<'a> {
        Lexer { source, cursor: Cursor::new(source.text()), eof: None }
    }

    pub fn source(&self) -> &'a SourceFile {
        self.source
    }

    /// Returns the problems found so far, in the order found.
    pub fn diagnostics(&self) -> &[Diagnostic] {
        self.cursor.diagnostics()
    }

    /// Returns the next token. Once the input is used up, every call returns the same zero-width
    /// EOF token.
    pub fn next_token(&mut self) -> Token {
        if let Some(eof) = self.eof {
            return eof;
        }
        let flags = self.skip_trivia();
        let mut token = self.scan();
        token.flags |= flags;
        if token.kind == Kind::Eof {
            self.eof = Some(token);
        }
        token
    }

    fn scan(&mut self) -> Token {
        let c = &mut self.cursor;
        match c.peek() {
            None => Token::eof(c.pos(), 0),
            Some(b'0'..=b'9') => scan_number::literal(c),
            Some(b'"') => scan_quoted::string(c),
            Some(b'`') => scan_quoted::multiline(c),
            Some(b'\'') => scan_quoted::character(c),
            Some(_) if c.at_identifier_start() => scan_identifier::identifier_or_keyword(c),
            Some(_) => scan_operator::operator(c).unwrap_or_else(|| unknown_character(c)),
        }
    }

    /// Skips whitespace and comments, and returns the [`Token::SPACE_BEFORE`] and
    /// [`Token::NEWLINE_BEFORE`] flags they earn the token after them.
    fn skip_trivia(&mut self) -> u8 {
        let c = &mut self.cursor;
        let start = c.pos();
        let mut line_break = false;
        loop {
            match c.peek() {
                Some(b' ' | b'\t') => c.bump(),
                Some(b'\n' | b'\r') => {
                    line_break = true;
                    c.bump();
                }
                _ if scan_comment::at_comment(c) => line_break |= scan_comment::skip(c),
                _ => break,
            }
        }

        let flags = if c.pos() > start { Token::SPACE_BEFORE } else { 0 };
        if line_break { flags | Token::NEWLINE_BEFORE } else { flags }
    }
}

/// Consumes one character nothing else recognized, so lexing always makes progress.
fn unknown_character(c: &mut Cursor) -> Token {
    debug_assert!(!c.is_at_end(), "no character to consume at {}", c.pos());
    let start = c.pos();
    c.advance();
    c.report(start, "unexpected character");
    c.token(Kind::Error, start, 0)
}
