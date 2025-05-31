use crate::{
    ast::{Expression, ExpressionRef},
    lexer::TokenType,
    Lexer, Token,
};

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
    pub fn new(source: &'a str) -> Self {
        let mut lexer = Lexer::new(&source);
        let lookahead = match lexer.next_token() {
            Ok(token) => token,
            Err(error) => panic!("Could not parse token: {:?}", error),
        };

        Parser {
            source,
            lexer,
            lookahead,
        }
    }

    ///
    /// Expects a token of a given type
    ///
    pub(super) fn eat_token(&mut self, token_type: TokenType) -> Token {
        if self.lookahead.token_type != token_type {
            panic!(
                "Unexpected token: {}, expected token: '{}'",
                self.lookahead.token_type, token_type
            );
        }

        let pre_token = self.lookahead;
        self.lookahead = match self.lexer.next_token() {
            Ok(token) => token,
            Err(error) => panic!("Could not parse token: {:?}", error),
        };
        pre_token
    }

    ///
    /// Expects a token of a given types
    ///
    pub(super) fn eat_any_of_token(&mut self, token_types: &[TokenType]) -> Token {
        for token_type in token_types {
            if self.lookahead.token_type == *token_type {
                let pre_token = self.lookahead;
                self.lookahead = match self.lexer.next_token() {
                    Ok(token) => token,
                    Err(error) => panic!("Could not parse token: {:?}", error),
                };
                return pre_token;
            }
        }

        panic!(
            "Unexpected token: {}, expected tokens: '{:?}'",
            self.lookahead.token_type, token_types
        );
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
    pub(super) fn is_expression_valid_assignment_target(&self, expression: &ExpressionRef) -> bool {
        matches!(
            expression.as_ref(),
            Expression::Identifier { .. } | Expression::Member { .. }
        )
    }

    ///
    /// Check if the current token is literal
    ///
    pub(super) fn is_next_token_literal(&self) -> bool {
        self.is_next_token_any_of_type(&[
            TokenType::BooleanTrue,
            TokenType::BooleanFalse,
            TokenType::Nil,
            TokenType::Number,
            TokenType::String,
        ])
    }

    ///
    /// Check if the current token is assignment operator
    ///
    pub(super) fn is_next_token_assignment_operator(&self) -> bool {
        self.is_next_token_any_of_type(&[
            TokenType::SimpleAssignmentOperator,
            TokenType::ComplexPlusAssignmentOperator,
            TokenType::ComplexMinusAssignmentOperator,
            TokenType::ComplexMultiplyAssignmentOperator,
            TokenType::ComplexDivideAssignmentOperator,
        ])
    }
}
