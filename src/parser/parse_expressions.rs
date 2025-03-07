use std::rc::Rc;

use crate::ast::{Expression, ExpressionRef};
use crate::lexer::TokenType;

use super::Parser;

pub(super) trait ParseExpressions {
    /**
     * Expression
     *  : Literal
     *  ;
     */
    fn expression(&mut self) -> ExpressionRef;

    /**
     * AdditiveExpression
     *  : FactorExpression
     *  | AdditiveExpression ADDITIVE_OPERATOR FactorExpression
     *  ;
     */
    fn additive_expression(&mut self) -> ExpressionRef;

    /**
     * FactorExpression
     *  : PrimaryExpression
     *  | FactorExpression FACTOR_OPERATOR PrimaryExpression
     *  ;
     */
    fn factor_expression(&mut self) -> ExpressionRef;

    /**
     * PrimaryExpression
     *  : Literal
     *  | GroupExpression
     *  ;
     */
    fn primary_expression(&mut self) -> ExpressionRef;

    /**
     * GroupExpression
     *  : '(' Expression ')'
     *  ;
     */
    fn group_expression(&mut self) -> ExpressionRef;

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
        self.additive_expression()
    }

    fn additive_expression(&mut self) -> ExpressionRef {
        let mut left = self.factor_expression();

        while self.lookahead.token_type == TokenType::AdditiveOperator {
            let operator_token = self.eat(TokenType::AdditiveOperator);
            let operator = &self.source[operator_token.i..operator_token.j];

            let right = self.factor_expression();

            left = Rc::new(Expression::BinaryExpression {
                operator: String::from(operator),
                left,
                right,
            });
        }

        left
    }

    fn factor_expression(&mut self) -> ExpressionRef {
        let mut left = self.primary_expression();

        while self.lookahead.token_type == TokenType::FactorOperator {
            let operator_token = self.eat(TokenType::FactorOperator);
            let operator = &self.source[operator_token.i..operator_token.j];

            let right = self.primary_expression();

            left = Rc::new(Expression::BinaryExpression {
                operator: String::from(operator),
                left,
                right,
            });
        }

        left
    }

    fn primary_expression(&mut self) -> ExpressionRef {
        match self.lookahead.token_type {
            TokenType::OpeningParenthesis => return self.group_expression(),
            _ => return self.literal(),
        }
    }

    fn group_expression(&mut self) -> ExpressionRef {
        self.eat(TokenType::OpeningParenthesis);
        let expression_ref = self.expression();
        self.eat(TokenType::ClosingParenthesis);

        expression_ref
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
