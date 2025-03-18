use std::rc::Rc;

use crate::ast::{BinaryOperator, Expression, ExpressionNode, Statement};
use crate::{lexer::TokenType, Lexer, Token};

use super::parse_statements::ParseStatements;

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
    pub fn parse(&mut self) -> Statement {
        self.program()
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

    /**
     * Expects a token of a given types
     */
    pub(super) fn eat_any_of(&mut self, token_types: &[TokenType]) -> Token {
        for token_type in token_types {
            if self.lookahead.token_type == *token_type {
                let pre_token = self.lookahead;
                self.lookahead = self.lexer.next_token();
                return pre_token;
            }
        }

        panic!(
            "Unexpected token: {}, expected tokens: '{:?}'",
            self.lookahead.token_type, token_types
        );
    }

    /**
     * Check the current token type
     */
    #[allow(dead_code)]
    pub(super) fn is_token(&mut self, token_type: TokenType) -> bool {
        self.lookahead.token_type == token_type
    }

    /**
     * Check the current token type
     */
    #[allow(dead_code)]
    pub(super) fn is_any_of_token(&mut self, token_types: &[TokenType]) -> bool {
        for token_type in token_types {
            if self.lookahead.token_type == *token_type {
                return true;
            }
        }

        false
    }

    /**
     * Check if the expression is valid assignment target
     */
    #[allow(dead_code)]
    pub(super) fn is_valid_assignment_target(&mut self, expression: &Expression) -> bool {
        match expression.as_ref() {
            ExpressionNode::Identifier(_) => return true,
            _ => {
                return false;
            }
        }
    }

    /**
     * Check if the current token is literal
     */
    #[allow(dead_code)]
    pub(super) fn is_literal_token(&mut self) -> bool {
        self.is_any_of_token(&[TokenType::Number, TokenType::String])
    }

    /**
     * Check if the current token is assignment operator
     */
    #[allow(dead_code)]
    pub(super) fn is_assignment_operator_token(&mut self) -> bool {
        self.is_any_of_token(&[
            TokenType::SimpleAssignmentOperator,
            TokenType::ComplexAssignmentOperator,
        ])
    }

    /**
     * Parse generic binary expression
     */
    pub(super) fn parse_binary_expression<OperandParserFnType, OperatorMapperFnType>(
        &mut self,
        token_type: TokenType,
        operand_parser: OperandParserFnType,
        operator_mapper: OperatorMapperFnType,
    ) -> Expression
    where
        OperandParserFnType: Fn(&mut Self) -> Expression,
        OperatorMapperFnType: Fn(&str) -> BinaryOperator,
    {
        let mut left = operand_parser(self);

        while self.lookahead.token_type == token_type {
            let operator_token = self.eat(token_type);
            let operator_value = &self.source[operator_token.i..operator_token.j];
            let operator = operator_mapper(operator_value);

            let right = operand_parser(self);

            left = Rc::new(ExpressionNode::Binary {
                operator,
                left,
                right,
            });
        }

        left
    }
}
