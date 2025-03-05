use std::rc::Rc;

use crate::lexer::TokenType;

use super::{Expression, Parser};

pub(super) trait ParseLiterals {
    /**
     * Literal
     *  : NumericLiteral
     *  | StringLiteral
     *  ;
     */
    fn literal(&mut self) -> Rc<Expression>;

    /**
     * StringLiteral
     *  : STRING
     *  ;
     */
    fn string_literal(&mut self) -> Rc<Expression>;

    /**
     * NumericLiteral
     *  : NUMBER
     *  ;
     */
    fn numeric_literal(&mut self) -> Rc<Expression>;
}

impl<'a> ParseLiterals for Parser<'a> {
    fn literal(&mut self) -> Rc<Expression> {
        match self.lookahead.token_type {
            TokenType::String => self.string_literal(),
            TokenType::Number => self.numeric_literal(),
            _ => panic!("Literal: unexpected literal production"),
        }
    }

    fn string_literal(&mut self) -> Rc<Expression> {
        let token = self.eat(TokenType::String);
        let token_value = &self.source[token.i + 1..token.j - 1];

        Rc::new(Expression::StringLiteral(String::from(token_value)))
    }

    fn numeric_literal(&mut self) -> Rc<Expression> {
        let token = self.eat(TokenType::Number);
        let token_value = &self.source[token.i..token.j];
        let token_value = token_value.trim().parse().unwrap();

        Rc::new(Expression::NumericLiteral(token_value))
    }
}
