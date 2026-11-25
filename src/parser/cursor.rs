use crate::lexer::{Diagnostic, Kind, Lexer, Token};

/// The parser's only mutable state: the lexer, the next token and the diagnostics so far. Parsers
/// move it forward and report problems through it.
pub(crate) struct TokenCursor<'a> {
    lexer: Lexer<'a>,
    current: Token,
    diagnostics: Vec<Diagnostic>,
}

impl<'a> TokenCursor<'a> {
    pub fn new(mut lexer: Lexer<'a>) -> TokenCursor<'a> {
        let current = lexer.next_token();
        TokenCursor { lexer, current, diagnostics: Vec::new() }
    }

    pub fn lexer(&self) -> &Lexer<'a> {
        &self.lexer
    }

    pub fn peek(&self) -> Token {
        self.current
    }

    pub fn at(&self, kind: Kind) -> bool {
        self.current.kind == kind
    }

    pub fn at_end(&self) -> bool {
        self.at(Kind::Eof)
    }

    /// Consumes the next token and returns it. At the end, returns EOF without moving.
    pub fn advance(&mut self) -> Token {
        let token = self.current;
        self.current = self.lexer.next_token();
        token
    }

    /// Consumes `start` if it is still the next token, so a loop whose last step parsed nothing
    /// moves on. Whatever left the token in place has already reported it.
    pub fn skip_if_stuck(&mut self, start: Token) {
        if self.current == start && !self.at_end() {
            self.advance();
        }
    }

    pub fn text(&self, token: Token) -> &'a str {
        self.lexer.source().text_of(&token)
    }

    /// Describes a token for a diagnostic, like `')'` or `end of input`.
    pub fn describe(&self, token: Token) -> String {
        if token.kind == Kind::Eof {
            return "end of input".to_string();
        }
        format!("'{}'", self.text(token))
    }

    /// Reports `message` followed by `, found ` and a description of the next token, at that token.
    pub fn report_found(&mut self, message: &str) {
        let token = self.current;
        let found = self.describe(token);
        self.report_token(token, format!("{message}, found {found}"));
    }

    pub fn report_token(&mut self, token: Token, message: impl Into<String>) {
        self.report(token.start, token.end, message);
    }

    pub fn report(&mut self, start: usize, end: usize, message: impl Into<String>) {
        self.diagnostics.push(Diagnostic::new(start, end, message));
    }

    pub fn diagnostics(&self) -> &[Diagnostic] {
        &self.diagnostics
    }

    pub fn problem_count(&self) -> usize {
        self.diagnostics.len()
    }
}
