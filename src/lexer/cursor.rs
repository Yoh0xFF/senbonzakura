use unicode_general_category::{GeneralCategory, get_general_category};

use super::{Diagnostic, Kind, Token};

/// The lexer's only mutable state: the text, the current position and the diagnostics so far.
/// Scanners move it forward and build tokens from it.
pub(crate) struct Cursor<'a> {
    text: &'a str,
    pos: usize,
    diagnostics: Vec<Diagnostic>,
}

impl<'a> Cursor<'a> {
    pub fn new(text: &'a str) -> Cursor<'a> {
        Cursor { text, pos: 0, diagnostics: Vec::new() }
    }

    pub fn pos(&self) -> usize {
        self.pos
    }

    pub fn is_at_end(&self) -> bool {
        self.pos >= self.text.len()
    }

    /// The byte at the cursor, or `None` past the end. A byte of a non-ASCII character is never
    /// equal to an ASCII one, so comparing it against ASCII is safe.
    pub fn peek(&self) -> Option<u8> {
        self.peek_at(0)
    }

    pub fn peek_at(&self, ahead: usize) -> Option<u8> {
        self.text.as_bytes().get(self.pos + ahead).copied()
    }

    pub fn peek_digit(&self, radix: u32) -> Option<u32> {
        self.peek_digit_at(0, radix)
    }

    /// Returns the value of the digit `ahead` bytes past the cursor in `radix`, if there is one.
    /// Digits above 9 are the letters `a`-`z` in either case. ASCII only, unlike
    /// [`char::is_numeric`], which accepts digits from other scripts.
    pub fn peek_digit_at(&self, ahead: usize, radix: u32) -> Option<u32> {
        let ch = self.peek_at(ahead)?;
        (ch as char).to_digit(radix).filter(|_| ch.is_ascii())
    }

    pub fn looking_at(&self, s: &str) -> bool {
        self.text[self.pos..].starts_with(s)
    }

    pub fn text(&self, start: usize, end: usize) -> &'a str {
        &self.text[start..end]
    }

    /// Skips `n` bytes. Only for characters [`peek`](Self::peek) has shown to be ASCII.
    pub fn bump_n(&mut self, n: usize) {
        debug_assert!(
            (0..n).all(|i| self.peek_at(i).is_some_and(|b| b.is_ascii())),
            "bump_n({n}) over non-ASCII or past the end at {}",
            self.pos
        );
        self.pos += n;
    }

    /// Skips one byte. Only for a character [`peek`](Self::peek) has shown to be ASCII.
    pub fn bump(&mut self) {
        self.bump_n(1);
    }

    /// Consumes one character, however many bytes it takes.
    pub fn advance(&mut self) {
        if let Some(c) = self.text[self.pos..].chars().next() {
            self.pos += c.len_utf8();
        }
    }

    /// Consumes `\n`, `\r\n` or a lone `\r` as one line break, if one is next.
    pub fn skip_line_break(&mut self) {
        if self.peek() == Some(b'\r') {
            self.pos += 1;
        }
        if self.peek() == Some(b'\n') {
            self.pos += 1;
        }
    }

    pub fn at_identifier_start(&self) -> bool {
        self.text[self.pos..].chars().next().is_some_and(|c| c == '_' || is_letter(c))
    }

    /// Consumes letters, digits and `_`, and returns whether there were any.
    pub fn skip_identifier_chars(&mut self) -> bool {
        let start = self.pos;
        for c in self.text[self.pos..].chars() {
            if c != '_' && !is_letter(c) && !is_digit(c) {
                break;
            }
            self.pos += c.len_utf8();
        }
        self.pos > start
    }

    /// Returns a token from `start` up to the current position.
    pub fn token(&self, kind: Kind, start: usize, flags: u8) -> Token {
        Token::new(kind, start, self.pos, flags)
    }

    /// Records a diagnostic from `start` up to the current position.
    pub fn report(&mut self, start: usize, message: impl Into<String>) {
        self.report_range(start, self.pos, message);
    }

    pub fn report_range(&mut self, start: usize, end: usize, message: impl Into<String>) {
        self.diagnostics.push(Diagnostic::new(start, end, message));
    }

    pub fn diagnostics(&self) -> &[Diagnostic] {
        &self.diagnostics
    }
}

/// A letter of any script: a character in one of the Unicode letter categories. Not
/// [`char::is_alphabetic`], which also takes letter-like numbers such as `Ⅰ` and combining marks.
fn is_letter(c: char) -> bool {
    matches!(
        get_general_category(c),
        GeneralCategory::UppercaseLetter
            | GeneralCategory::LowercaseLetter
            | GeneralCategory::TitlecaseLetter
            | GeneralCategory::ModifierLetter
            | GeneralCategory::OtherLetter
    )
}

/// A decimal digit of any script, like `٣`.
fn is_digit(c: char) -> bool {
    get_general_category(c) == GeneralCategory::DecimalNumber
}
