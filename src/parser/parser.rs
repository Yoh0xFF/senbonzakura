use crate::{ast::Expression, lexer::TokenType, Lexer, Token};

use super::{ParserError, ParserResult};

///
/// Recursive descent parser
///
#[derive(Debug, Clone)]
pub struct Parser<'a> {
    pub(super) source: &'a str,
    pub(super) lexer: Lexer<'a>,
    pub(super) lookahead: Token,
}

#[allow(dead_code)]
impl<'a> Parser<'a> {
    pub fn new(source: &'a str) -> ParserResult<Self> {
        let mut lexer = Lexer::new(&source);
        let lookahead = Self::next_token(&mut lexer)?;

        Ok(Parser {
            source,
            lexer,
            lookahead,
        })
    }

    ///
    /// Reads the next token from the lexer
    ///
    fn next_token(lexer: &mut Lexer<'a>) -> ParserResult<Token> {
        lexer
            .next_token()
            .map_err(|error| ParserError::LexicalError {
                message: format!("Could not parse token: {:?}", error),
            })
    }

    ///
    /// Advances to the next token, returning the current one
    ///
    fn advance(&mut self) -> ParserResult<Token> {
        let next = Self::next_token(&mut self.lexer)?;
        Ok(std::mem::replace(&mut self.lookahead, next))
    }

    ///
    /// Expects a token of a given type
    ///
    pub(super) fn eat_token(&mut self, token_type: TokenType) -> ParserResult<Token> {
        if self.lookahead.token_type != token_type {
            return Err(ParserError::LexicalError {
                message: format!(
                    "Unexpected token: {}, expected token: '{}'",
                    self.lookahead.token_type, token_type
                ),
            });
        }

        self.advance()
    }

    ///
    /// Expects a token of a given types
    ///
    pub(super) fn eat_any_of_token(&mut self, token_types: &[TokenType]) -> ParserResult<Token> {
        if self.is_next_token_any_of_type(token_types) {
            return self.advance();
        }

        Err(ParserError::LexicalError {
            message: format!(
                "Unexpected token: {}, expected tokens: '{:?}'",
                self.lookahead.token_type, token_types
            ),
        })
    }

    ///
    /// Check the current token type
    ///
    pub(super) fn is_next_token_of_type(&self, token_type: TokenType) -> bool {
        self.lookahead.token_type == token_type
    }

    ///
    /// Check the current token type
    ///
    pub(super) fn is_next_token_any_of_type(&self, token_types: &[TokenType]) -> bool {
        for token_type in token_types {
            if self.lookahead.token_type == *token_type {
                return true;
            }
        }

        false
    }

    ///
    /// Check if the expression is valid assignment target
    ///
    pub(super) fn is_expression_valid_assignment_target(&self, expression: &Expression) -> bool {
        matches!(
            expression,
            Expression::Identifier { .. } | Expression::Member { .. }
        )
    }

    ///
    /// Check if the current token is literal
    ///
    pub(super) fn is_next_token_literal(&self) -> bool {
        self.lookahead.is_literal()
    }

    ///
    /// Check if the current token is assignment operator
    ///
    pub(super) fn is_next_token_assignment_operator(&self) -> bool {
        self.lookahead.is_assignment_operator()
    }
}
