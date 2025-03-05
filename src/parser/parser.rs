use std::rc::Rc;

use crate::{lexer::TokenType, Lexer, Token};

use super::{parse_literals::ParseLiterals, parse_root::ParseRootExpression, Expression};

/**
 * Senbonzakura recursive descent parser
 */
#[derive(Debug, Clone)]
pub struct Parser<'a> {
    pub(super) source: &'a str,
    pub(super) lexer: Lexer<'a>,
    pub(super) lookahead: Token,
}

impl<'a> Parser<'a> {
    /**
     * Parses a string into an AST
     */
    pub fn new(source: &'a str) -> Self {
        let mut lexer = Lexer::new(&source);
        let lookahead = lexer.next_token();

        Parser {
            source,
            lexer,
            lookahead,
        }
    }

    pub fn parse(&mut self) -> Rc<Expression> {
        self.program()
    }

    /**
     * Main entry point
     *
     * Program
     *  : StatementList
     *  ;
     */
    fn program(&mut self) -> Rc<Expression>
    where
        Self: ParseLiterals,
    {
        let statement_list = self.statement_list();
        Rc::new(Expression::Program {
            body: statement_list,
        })
    }

    /**
     * Expects a token of a given type
     */
    pub(super) fn eat(&mut self, token_type: TokenType) -> Token {
        if self.lookahead.token_type != token_type {
            panic!(
                "Unexpected token: {}, expected token: '{}'",
                self.lookahead.token_type, token_type
            );
        }

        let pre_token = self.lookahead;
        self.lookahead = self.lexer.next_token();
        pre_token
    }
}
