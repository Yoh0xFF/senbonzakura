use std::{mem, rc::Rc};

use crate::{lexer::TokenType, Lexer, Token};

use super::Expression;

/**
 * Senbonzakura recursive descent parser
 */
#[derive(Debug, Clone)]
pub struct Parser {
    lexer: Lexer,
    lookahead: Token,
}

impl Parser {
    /**
     * Parses a string into an AST
     */
    pub fn new() -> Self {
        Parser {
            lexer: Lexer::new(),
            lookahead: Token {
                index: 0,
                token_type: TokenType::End,
                value: ".".to_string(),
            },
        }
    }

    pub fn parse(&mut self, source: &str) -> Expression {
        self.lexer.init(source);
        self.lookahead = self.lexer.next_token();
        self.program()
    }

    /**
     * Main entry point
     *
     * Program
     *  : NumericLiteral
     *  ;
     */
    fn program(&mut self) -> Expression {
        let literal = self.literal();
        Expression::Program { body: literal }
    }

    /**
     * Literal
     *  : NumericLiteral
     *  | StringLiteral
     *  ;
     */
    fn literal(&mut self) -> Rc<Expression> {
        match self.lookahead.token_type {
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
    fn string_literal(&mut self) -> Rc<Expression> {
        let token = self.eat(TokenType::String);
        Rc::new(Expression::StringLiteral(token.value))
    }

    /**
     * NumericLiteral
     *  : NUMBER
     *  ;
     */
    fn numeric_literal(&mut self) -> Rc<Expression> {
        let token = self.eat(TokenType::Number);
        Rc::new(Expression::NumericLiteral(
            token.value.trim().parse().unwrap(),
        ))
    }

    /**
     * Expects a token of a given type
     */
    fn eat(&mut self, token_type: TokenType) -> Token {
        if self.lookahead.token_type != token_type {
            panic!(
                "Unexpected token: {}, expected token: '{}'",
                self.lookahead.token_type, token_type
            );
        }

        let token = mem::replace(&mut self.lookahead, self.lexer.next_token());

        token
    }
}
