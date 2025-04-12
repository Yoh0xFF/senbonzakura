use super::Parser;
use crate::ast::{
    AssignmentOperator, BinaryOperator, Expression, ExpressionRef, LogicalOperator, UnaryOperator,
};
use crate::lexer::TokenType;
use crate::parser::parsers::{
    eat, eat_any_of, is_any_of_token, is_assignment_operator_token, is_literal_token,
    is_valid_assignment_target, parse_binary_expression, parse_logical_expression,
};

pub(super) trait ParseExpressions {
    /**
     * Expression
     *  : Literal
     *  ;
     */
    fn expression(&mut self) -> ExpressionRef;

    /**
     * VariableInitializationExpression
     *  : Identifier ['=' Initializer]
     *  ;
     */
    fn variable_initialization_expression(&mut self) -> ExpressionRef;

    /**
     * AssignmentExpression
     *  : LogicalOrExpression
     *  | LeftHandSideExpression ASSIGNMENT_OPERATOR AssignmentExpression
     *  ;
     */
    fn assignment_expression(&mut self) -> ExpressionRef;

    /**
     * LogicalOrExpression
     *  : LogicalAndExpression LOGICAL_OR_OPERATOR LogicalOrExpression
     *  | LogicalAndExpression
     *  ;
     */
    fn logical_or_expression(&mut self) -> ExpressionRef;

    /**
     * LogicalAndExpression
     *  : EqualityExpression LOGICAL_AND_OPERATOR LogicalAndExpression
     *  | EqualityExpression
     *  ;
     */
    fn logical_and_expression(&mut self) -> ExpressionRef;

    /**
     * EqualityExpression
     *  : RelationalExpression EQUALITY_OPERATOR EqualityExpression
     *  | RelationalExpression
     *  ;
     */
    fn equality_expression(&mut self) -> ExpressionRef;

    /**
     * RelationalExpression
     *  : AdditiveExpression
     *  | AdditiveExpression RELATIONAL_OPERATOR AdditiveExpression
     *  ;
     */
    fn relational_expression(&mut self) -> ExpressionRef;

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
     * UnaryExpression
     *  : LeftHandSideExpression
     *  | ADDITIVE_OPERATOR UnaryExpression
     *  | LOGICAL_NOT_OPERATOR UnaryExpression
     *  ;
     */
    fn unary_expression(&mut self) -> ExpressionRef;

    /**
     * LeftHandSideExpression
     *  : PrimaryExpression
     *  ;
     */
    fn left_hand_side_expression(&mut self) -> ExpressionRef;

    /**
     * PrimaryExpression
     *  : LiteralExpression
     *  | GroupExpression
     *  | IdentifierExpression
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
     * IdentifierExpression
     *  : IDENTIFIER
     *  ;
     */
    fn identifier_expression(&mut self) -> ExpressionRef;

    /**
     * Literal
     *  : BooleanLiteral
     *  : NilLiteral
     *  : NumericLiteral
     *  | StringLiteral
     *  ;
     */
    fn literal_expression(&mut self) -> ExpressionRef;

    /**
     * BooleanLiteral
     *  : BOOLEAN
     *  ;
     */
    fn boolean_literal_expression(&mut self) -> ExpressionRef;

    /**
     * NilLiteral
     *  : NIL
     *  ;
     */
    fn nil_literal_expression(&mut self) -> ExpressionRef;

    /**
     * NumericLiteral
     *  : NUMBER
     *  ;
     */
    fn numeric_literal_expression(&mut self) -> ExpressionRef;

    /**
     * StringLiteral
     *  : STRING
     *  ;
     */
    fn string_literal_expression(&mut self) -> ExpressionRef;
}

impl<'a> ParseExpressions for Parser<'a> {
    fn expression(&mut self) -> ExpressionRef {
        self.assignment_expression()
    }

    fn variable_initialization_expression(&mut self) -> ExpressionRef {
        let identifier = self.identifier_expression();

        let initializer: Option<ExpressionRef> =
            if is_any_of_token(self, &[TokenType::StatementEnd, TokenType::Comma]) {
                None
            } else {
                eat(self, TokenType::SimpleAssignmentOperator);
                let initializer = self.assignment_expression();
                Some(initializer)
            };

        Box::new(Expression::VariableIntialization {
            identifier,
            initializer,
        })
    }

    fn assignment_expression(&mut self) -> ExpressionRef {
        let left = self.logical_or_expression();

        if !is_assignment_operator_token(self) {
            return left;
        }

        let assignment_operator_token = eat_any_of(
            self,
            &[
                TokenType::SimpleAssignmentOperator,
                TokenType::ComplexAssignmentOperator,
            ],
        );
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

        if !is_valid_assignment_target(&left) {
            panic!("Invalid left-hand side in the assignment expression");
        }

        Box::new(Expression::Assignment {
            operator: assignment_operator,
            left,
            right: self.assignment_expression(),
        })
    }

    fn logical_or_expression(&mut self) -> ExpressionRef {
        parse_logical_expression(
            self,
            TokenType::LogicalOrOperator,
            |parser| parser.logical_and_expression(),
            |op| match op {
                "||" => LogicalOperator::Or,
                _ => panic!("Unknown logical operator {}", op),
            },
        )
    }

    fn logical_and_expression(&mut self) -> ExpressionRef {
        parse_logical_expression(
            self,
            TokenType::LogicalAndOperator,
            |parser| parser.equality_expression(),
            |op| match op {
                "&&" => LogicalOperator::And,
                _ => panic!("Unknown logical operator {}", op),
            },
        )
    }

    fn equality_expression(&mut self) -> ExpressionRef {
        parse_binary_expression(
            self,
            TokenType::EqualityOperator,
            |parser| parser.relational_expression(),
            |op| match op {
                "==" => BinaryOperator::Equal,
                "!=" => BinaryOperator::NotEqual,
                _ => panic!("Unknown relational operator {}", op),
            },
        )
    }

    fn relational_expression(&mut self) -> ExpressionRef {
        parse_binary_expression(
            self,
            TokenType::RelationalOperator,
            |parser| parser.additive_expression(),
            |op| match op {
                ">" => BinaryOperator::GreaterThan,
                ">=" => BinaryOperator::GreaterThanOrEqualTo,
                "<" => BinaryOperator::LessThan,
                "<=" => BinaryOperator::LessThanOrEqualTo,
                _ => panic!("Unknown relational operator {}", op),
            },
        )
    }

    fn additive_expression(&mut self) -> ExpressionRef {
        parse_binary_expression(
            self,
            TokenType::AdditiveOperator,
            |parser| parser.factor_expression(),
            |op| match op {
                "+" => BinaryOperator::Add,
                "-" => BinaryOperator::Subtract,
                _ => panic!("Unknown additive operator {}", op),
            },
        )
    }

    fn factor_expression(&mut self) -> ExpressionRef {
        parse_binary_expression(
            self,
            TokenType::FactorOperator,
            |parser| parser.unary_expression(),
            |op| match op {
                "*" => BinaryOperator::Multiply,
                "/" => BinaryOperator::Divide,
                _ => panic!("Unknown factor operator {}", op),
            },
        )
    }

    fn unary_expression(&mut self) -> ExpressionRef {
        let mut operator: Option<UnaryOperator> = None;

        if is_any_of_token(
            self,
            &[TokenType::AdditiveOperator, TokenType::LogicalNotOperator],
        ) {
            let token = eat_any_of(
                self,
                &[TokenType::AdditiveOperator, TokenType::LogicalNotOperator],
            );
            let token_value = &self.source[token.i..token.j];

            operator = Some(match token_value {
                "+" => UnaryOperator::Plus,
                "-" => UnaryOperator::Minus,
                "!" => UnaryOperator::Not,
                _ => panic!("Unknown unary operator {}", token_value),
            });
        }

        if operator.is_some() {
            return Box::new(Expression::Unary {
                operator: operator.unwrap(),
                right: self.unary_expression(),
            });
        }

        self.left_hand_side_expression()
    }

    fn left_hand_side_expression(&mut self) -> ExpressionRef {
        self.primary_expression()
    }

    fn primary_expression(&mut self) -> ExpressionRef {
        if is_literal_token(self) {
            return self.literal_expression();
        }

        match self.lookahead.token_type {
            TokenType::OpeningParenthesis => self.group_expression(),
            TokenType::Identifier => self.identifier_expression(),
            _ => self.left_hand_side_expression(),
        }
    }

    fn group_expression(&mut self) -> ExpressionRef {
        eat(self, TokenType::OpeningParenthesis);
        let expression_ref = self.expression();
        eat(self, TokenType::ClosingParenthesis);

        expression_ref
    }

    fn identifier_expression(&mut self) -> ExpressionRef {
        let identifier_token = eat(self, TokenType::Identifier);
        let identifier_value = &self.source[identifier_token.i..identifier_token.j];

        Box::new(Expression::Identifier(String::from(identifier_value)))
    }

    fn literal_expression(&mut self) -> ExpressionRef {
        match self.lookahead.token_type {
            TokenType::Boolean => self.boolean_literal_expression(),
            TokenType::Nil => self.nil_literal_expression(),
            TokenType::Number => self.numeric_literal_expression(),
            TokenType::String => self.string_literal_expression(),
            _ => panic!("Literal: unexpected literal production"),
        }
    }

    fn boolean_literal_expression(&mut self) -> ExpressionRef {
        let token = eat(self, TokenType::Boolean);
        let token_value = &self.source[token.i..token.j];
        let bool_value = token_value == "true";

        Box::new(Expression::BooleanLiteral(bool_value))
    }

    fn nil_literal_expression(&mut self) -> ExpressionRef {
        eat(self, TokenType::Nil);

        Box::new(Expression::NilLiteral)
    }

    fn numeric_literal_expression(&mut self) -> ExpressionRef {
        let token = eat(self, TokenType::Number);
        let token_value = &self.source[token.i..token.j];
        let token_value = token_value.trim().parse().unwrap();

        Box::new(Expression::NumericLiteral(token_value))
    }

    fn string_literal_expression(&mut self) -> ExpressionRef {
        let token = eat(self, TokenType::String);
        let token_value = &self.source[token.i + 1..token.j - 1];

        Box::new(Expression::StringLiteral(String::from(token_value)))
    }
}
