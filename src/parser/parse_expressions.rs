use std::rc::Rc;

use crate::ast::{AssignmentOperator, BinaryOperator, Expression, ExpressionNode};
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
     * VariableInitializationExpression
     *  : Identifier ['=' Initializer]
     *  ;
     */
    fn variable_initialization_expression(&mut self) -> Expression;

    /**
     * AssignmentExpression
     *  : AdditiveExpression
     *  | LeftHandSideExpression ASSIGNMENT_OPERATOR AssignmentExpression
     *  ;
     */
    fn assignment_expression(&mut self) -> Expression;

    /**
     * LeftHandSideExpression
     *  : IdentifierExpression
     *  ;
     */
    fn left_hand_side_expression(&mut self) -> Expression;

    /**
     * IdentifierExpression
     *  : IDENTIFIER
     *  ;
     */
    fn identifier_expression(&mut self) -> Expression;

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
     *  : LiteralExpression
     *  | GroupExpression
     *  | LeftHandSideExpression
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
    fn literal_expression(&mut self) -> Expression;

    /**
     * StringLiteral
     *  : STRING
     *  ;
     */
    fn string_literal_expression(&mut self) -> Expression;

    /**
     * NumericLiteral
     *  : NUMBER
     *  ;
     */
    fn numeric_literal_expression(&mut self) -> Expression;
}

impl<'a> ParseExpressions for Parser<'a> {
    fn expression(&mut self) -> Expression {
        self.assignment_expression()
    }

    fn variable_initialization_expression(&mut self) -> Expression {
        let identifier = self.identifier_expression();

        let initializer: Option<Expression> =
            if self.is_any_of_token(&[TokenType::StatementEnd, TokenType::Comma]) {
                None
            } else {
                self.eat(TokenType::SimpleAssignmentOperator);
                let initializer = self.assignment_expression();
                Some(initializer)
            };

        Rc::new(ExpressionNode::VariableIntialization {
            identifier,
            initializer,
        })
    }

    fn assignment_expression(&mut self) -> Expression {
        let left = self.additive_expression();

        if !self.is_assignment_operator_token() {
            return left;
        }

        let assignment_operator_token = self.eat_any_of(&[
            TokenType::SimpleAssignmentOperator,
            TokenType::ComplexAssignmentOperator,
        ]);
        let assignment_operator_value =
            &self.source[assignment_operator_token.i..assignment_operator_token.j];
        let assignment_operator = match assignment_operator_value {
            "=" => AssignmentOperator::Assign,
            "+=" => AssignmentOperator::AssignAdd,
            "-=" => AssignmentOperator::AssignSubtract,
            "*=" => AssignmentOperator::AssignMultiply,
            "/=" => AssignmentOperator::AssignDivide,
            _ => panic!("Unknown assignment operator {}", assignment_operator_value),
        };

        if !self.is_valid_assignment_target(&left) {
            panic!("Invalid left-hand side in the assignment expression");
        }

        Rc::new(ExpressionNode::Assignment {
            operator: assignment_operator,
            left: left,
            right: self.assignment_expression(),
        })
    }

    fn left_hand_side_expression(&mut self) -> Expression {
        self.identifier_expression()
    }

    fn identifier_expression(&mut self) -> Expression {
        let identifier_token = self.eat(TokenType::Identifier);
        let identifier_value = &self.source[identifier_token.i..identifier_token.j];

        Rc::new(ExpressionNode::Identifier(String::from(identifier_value)))
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
        if self.is_literal_token() {
            return self.literal_expression();
        }

        match self.lookahead.token_type {
            TokenType::OpeningParenthesis => return self.group_expression(),
            _ => return self.left_hand_side_expression(),
        }
    }

    fn group_expression(&mut self) -> Expression {
        self.eat(TokenType::OpeningParenthesis);
        let expression_ref = self.expression();
        self.eat(TokenType::ClosingParenthesis);

        expression_ref
    }

    fn literal_expression(&mut self) -> Expression {
        match self.lookahead.token_type {
            TokenType::String => self.string_literal_expression(),
            TokenType::Number => self.numeric_literal_expression(),
            _ => panic!("Literal: unexpected literal production"),
        }
    }

    fn string_literal_expression(&mut self) -> Expression {
        let token = self.eat(TokenType::String);
        let token_value = &self.source[token.i + 1..token.j - 1];

        Rc::new(ExpressionNode::StringLiteral(String::from(token_value)))
    }

    fn numeric_literal_expression(&mut self) -> Expression {
        let token = self.eat(TokenType::Number);
        let token_value = &self.source[token.i..token.j];
        let token_value = token_value.trim().parse().unwrap();

        Rc::new(ExpressionNode::NumericLiteral(token_value))
    }
}
