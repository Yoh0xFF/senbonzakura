use std::rc::Rc;

use crate::ast::{BinaryOperator, Expression, ExpressionNode};
use crate::lexer::TokenType;

use super::Parser;

pub(super) trait ParseExpressions {
    /**
     * Expression
     *  : Literal
     *  ;
     */
    fn expression(&mut self) -> Expression;

    /**
     * AdditiveExpression
     *  : FactorExpression
     *  | AdditiveExpression ADDITIVE_OPERATOR FactorExpression
     *  ;
     */
    fn additive_expression(&mut self) -> Expression;

    /**
     * FactorExpression
     *  : PrimaryExpression
     *  | FactorExpression FACTOR_OPERATOR PrimaryExpression
     *  ;
     */
    fn factor_expression(&mut self) -> Expression;

    /**
     * PrimaryExpression
     *  : Literal
     *  | GroupExpression
     *  ;
     */
    fn primary_expression(&mut self) -> Expression;

    /**
     * GroupExpression
     *  : '(' Expression ')'
     *  ;
     */
    fn group_expression(&mut self) -> Expression;

    /**
     * Literal
     *  : NumericLiteral
     *  | StringLiteral
     *  ;
     */
    fn literal(&mut self) -> Expression;

    /**
     * StringLiteral
     *  : STRING
     *  ;
     */
    fn string_literal(&mut self) -> Expression;

    /**
     * NumericLiteral
     *  : NUMBER
     *  ;
     */
    fn numeric_literal(&mut self) -> Expression;
}

impl<'a> ParseExpressions for Parser<'a> {
    fn expression(&mut self) -> Expression {
        self.additive_expression()
    }

    fn additive_expression(&mut self) -> Expression {
        let mut left = self.factor_expression();

        while self.lookahead.token_type == TokenType::AdditiveOperator {
            let operator_token = self.eat(TokenType::AdditiveOperator);
            let operator_value = &self.source[operator_token.i..operator_token.j];
            let operator = match operator_value {
                "+" => BinaryOperator::Add,
                "-" => BinaryOperator::Subtract,
                _ => panic!("Unknown additive operator {}", operator_value),
            };

            let right = self.factor_expression();

            left = Rc::new(ExpressionNode::Binary {
                operator,
                left,
                right,
            });
        }

        left
    }

    fn factor_expression(&mut self) -> Expression {
        let mut left = self.primary_expression();

        while self.lookahead.token_type == TokenType::FactorOperator {
            let operator_token = self.eat(TokenType::FactorOperator);
            let operator_value = &self.source[operator_token.i..operator_token.j];
            let operator = match operator_value {
                "*" => BinaryOperator::Multiply,
                "/" => BinaryOperator::Divide,
                _ => panic!("Unknown factor operator {}", operator_value),
            };

            let right = self.primary_expression();

            left = Rc::new(ExpressionNode::Binary {
                operator,
                left,
                right,
            });
        }

        left
    }

    fn primary_expression(&mut self) -> Expression {
        match self.lookahead.token_type {
            TokenType::OpeningParenthesis => return self.group_expression(),
            _ => return self.literal(),
        }
    }

    fn group_expression(&mut self) -> Expression {
        self.eat(TokenType::OpeningParenthesis);
        let expression_ref = self.expression();
        self.eat(TokenType::ClosingParenthesis);

        expression_ref
    }

    fn literal(&mut self) -> Expression {
        match self.lookahead.token_type {
            TokenType::String => self.string_literal(),
            TokenType::Number => self.numeric_literal(),
            _ => panic!("Literal: unexpected literal production"),
        }
    }

    fn string_literal(&mut self) -> Expression {
        let token = self.eat(TokenType::String);
        let token_value = &self.source[token.i + 1..token.j - 1];

        Rc::new(ExpressionNode::StringLiteral(String::from(token_value)))
    }

    fn numeric_literal(&mut self) -> Expression {
        let token = self.eat(TokenType::Number);
        let token_value = &self.source[token.i..token.j];
        let token_value = token_value.trim().parse().unwrap();

        Rc::new(ExpressionNode::NumericLiteral(token_value))
    }
}
