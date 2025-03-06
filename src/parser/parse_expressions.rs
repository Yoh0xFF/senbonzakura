use std::rc::Rc;

use crate::lexer::TokenType;

use super::{Expression, ExpressionRef, Parser};

pub(super) trait ParseExpressions {
    /**
     * Expression
     *  : Literal
     *  ;
     */
    fn expression(&mut self) -> ExpressionRef;

    /**
     * Literal
     *  : NumericLiteral
     *  | StringLiteral
     *  ;
     */
    fn literal(&mut self) -> ExpressionRef;

    /**
     * StringLiteral
     *  : STRING
     *  ;
     */
    fn string_literal(&mut self) -> ExpressionRef;

    /**
     * NumericLiteral
     *  : NUMBER
     *  ;
     */
    fn numeric_literal(&mut self) -> ExpressionRef;
}

impl<'a> ParseExpressions for Parser<'a> {
    fn expression(&mut self) -> ExpressionRef {
        self.literal()
    }

    fn literal(&mut self) -> ExpressionRef {
        match self.lookahead.token_type {
            TokenType::String => self.string_literal(),
            TokenType::Number => self.numeric_literal(),
            _ => panic!("Literal: unexpected literal production"),
        }
    }

    fn string_literal(&mut self) -> ExpressionRef {
        let token = self.eat(TokenType::String);
        let token_value = &self.source[token.i + 1..token.j - 1];

        Rc::new(Expression::StringLiteral(String::from(token_value)))
    }

    fn numeric_literal(&mut self) -> ExpressionRef {
        let token = self.eat(TokenType::Number);
        let token_value = &self.source[token.i..token.j];
        let token_value = token_value.trim().parse().unwrap();

        Rc::new(Expression::NumericLiteral(token_value))
    }
}
