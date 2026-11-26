mod lexer;
mod scan_comment;
mod scan_identifier;
mod scan_number;
mod scan_operator;
mod scan_quoted;
mod source;
mod token;

use super::{Diagnostic, Kind, Lexer, SourceFile, Token};

/// A whole input lexed up to and including EOF, with everything the tests look at.
pub(super) struct Lexed {
    pub source: SourceFile,
    pub tokens: Vec<Token>,
    pub diagnostics: Vec<Diagnostic>,
}

pub(super) fn lex(text: &str) -> Lexed {
    let source = SourceFile::new("test", text);
    let (tokens, diagnostics) = {
        let mut lexer = Lexer::new(&source);
        let mut tokens = Vec::new();
        // Every token except EOF consumes at least one byte, so more than len + 1 means it's stuck.
        for _ in 0..=text.len() + 1 {
            let token = lexer.next_token();
            tokens.push(token);
            if token.kind == Kind::Eof {
                break;
            }
        }
        assert_eq!(tokens.last().map(|t| t.kind), Some(Kind::Eof), "lexer did not reach EOF");
        (tokens, lexer.diagnostics().to_vec())
    };
    Lexed { source, tokens, diagnostics }
}

impl Lexed {
    /// Kinds of all tokens before EOF.
    pub fn kinds(&self) -> Vec<Kind> {
        self.before_eof().iter().map(|t| t.kind).collect()
    }

    /// Source text of all tokens before EOF.
    pub fn texts(&self) -> Vec<&str> {
        self.before_eof().iter().map(|t| self.source.text_of(t)).collect()
    }

    pub fn eof(&self) -> Token {
        *self.tokens.last().expect("EOF")
    }

    /// The only token before EOF; fails if there are more or none.
    pub fn single(&self) -> Token {
        assert_eq!(
            self.before_eof().len(),
            1,
            "expected exactly one token, got {:?}",
            self.texts()
        );
        self.tokens[0]
    }

    pub fn text(&self, token: Token) -> &str {
        self.source.text_of(&token)
    }

    pub fn messages(&self) -> Vec<&str> {
        self.diagnostics.iter().map(|d| d.message.as_str()).collect()
    }

    /// The source text each diagnostic points at.
    pub fn diagnostic_texts(&self) -> Vec<&str> {
        self.diagnostics.iter().map(|d| &self.source.text()[d.start..d.end]).collect()
    }

    fn before_eof(&self) -> &[Token] {
        &self.tokens[..self.tokens.len() - 1]
    }
}
