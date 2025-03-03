use std::{cell::Cell, rc::Rc};

use crate::{lexer::TokenType, Lexer, Token};

use super::Expression;

/**
 * Senbonzakura recursive descent parser
 */
#[derive(Debug, Clone)]
pub struct Parser<'a> {
    source: &'a str,
    lexer: Lexer<'a>,
    lookahead: Cell<Token>,
}

impl<'a> Parser<'a> {
    /**
     * Parses a string into an AST
     */
    pub fn new(source: &'a str) -> Self {
        let lexer = Lexer::new(&source);
        let lookahead = lexer.next_token();

        Parser {
            source,
            lexer,
            lookahead: Cell::new(lookahead),
        }
    }

    pub fn parse(&self) -> Expression {
        self.program()
    }

    /**
     * Main entry point
     *
     * Program
     *  : NumericLiteral
     *  ;
     */
    fn program(&self) -> Expression {
        let literal = self.literal();
        Expression::Program { body: literal }
    }

    /**
     * Literal
     *  : NumericLiteral
     *  | StringLiteral
     *  ;
     */
    fn literal(&self) -> Rc<Expression> {
        let lookahead = self.lookahead.get();

        match lookahead.token_type {
            TokenType::String => self.string_literal(),
            TokenType::Number => self.numeric_literal(),
            _ => panic!("Literal: unexpected literal production"),
        }
    }

    /**
     * StringLiteral
     *  : STRING
     *  ;
     */
    fn string_literal(&self) -> Rc<Expression> {
        let token = self.eat(TokenType::String);
        let token_value = &self.source[token.i + 1..token.j - 1];

        Rc::new(Expression::StringLiteral(String::from(token_value)))
    }

    /**
     * NumericLiteral
     *  : NUMBER
     *  ;
     */
    fn numeric_literal(&self) -> Rc<Expression> {
        let token = self.eat(TokenType::Number);
        let token_value = &self.source[token.i..token.j];
        let token_value = token_value.trim().parse().unwrap();

        Rc::new(Expression::NumericLiteral(token_value))
    }

    /**
     * Expects a token of a given type
     */
    fn eat(&self, token_type: TokenType) -> Token {
        let lookahead = self.lookahead.get();

        if lookahead.token_type != token_type {
            panic!(
                "Unexpected token: {}, expected token: '{}'",
                lookahead.token_type, token_type
            );
        }

        self.lookahead.set(self.lexer.next_token());

        lookahead
    }
}
