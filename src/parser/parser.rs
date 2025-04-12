use crate::ast::StatementRef;
use crate::parser::parsers::program;
use crate::{Lexer, Token};

/**
 * Recursive descent parser
 */
#[derive(Debug, Clone)]
pub struct Parser<'a> {
    pub(super) source: &'a str,
    pub(super) lexer: Lexer<'a>,
    pub(super) lookahead: Token,
}

impl<'a> Parser<'a> {
    pub fn new(source: &'a str) -> Self {
        let mut lexer = Lexer::new(&source);
        let lookahead = lexer.next_token();

        Parser {
            source,
            lexer,
            lookahead,
        }
    }

    /**
     * Parses a string into an AST
     */
    pub fn parse(&mut self) -> StatementRef {
        program(self)
    }
}
