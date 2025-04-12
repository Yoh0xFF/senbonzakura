use crate::ast::{Expression, Statement, StatementList, StatementRef};
use crate::lexer::TokenType;
use crate::parser::parsers::{eat, is_token};
use super::{parse_expressions::ParseExpressions, Parser};

pub(super) trait ParseStatements {
    /**
     * Main entry point
     *
     * Program
     *  : StatementList
     *  ;
     */
    fn program(&mut self) -> StatementRef;

    /**
     * BlockStatement
     *  : '{' OptStatementList '}'
     *  ;
     */
    fn block_statement(&mut self) -> StatementRef;

    /**
     * StatementList
     *  : Statement
     *  | StatementList Statement
     *  ;
     */
    fn statement_list(&mut self, stop_token_type: Option<TokenType>) -> StatementList;

    /**
     * Statement
     *  : ExpressionStatement
     *  | BlockStatement
     *  | EmptyStatement
     *  | VariableStatement
     *  | ConditionalStatement
     *  | IterationStatement
     *  ;
     */
    fn statement(&mut self) -> StatementRef;

    /**
     * VariableDeclarationStatement
     *  : 'let' VariableInitializationList ';'
     *
     * VariableInitializationList
     *  : VariableInitialization
     *  | VariableInitializationList ',' VariableInitialization
     *  ;
     */
    fn variable_declaration_statement(&mut self, consume_statement_end: bool) -> StatementRef;

    /**
     * ConditionalStatement
     *  : if '(' Expression ')' Statement [else Statement]
     */
    fn if_statement(&mut self) -> StatementRef;

    /**
     * WhileStatement
     *  : while '(' Expression ')' Statement ';'
     *  ;
     */
    fn while_statement(&mut self) -> StatementRef;

    /**
     * WhileStatement
     *  : do Statement while '(' Expression ')' ';'
     *  ;
     */
    fn do_while_statement(&mut self) -> StatementRef;

    /**
     * ForStatement
     *  : for '(' [InitExpression] ';' [Expression] ';' [Expression] ')' Statement
     *  ;
     */
    fn for_statement(&mut self) -> StatementRef;

    /**
     * ForStatementInit
     *  : VariableDeclarationStatement
     *  | Expression
     *  ;
     */
    fn for_statement_init(&mut self) -> StatementRef;

    /**
     * EmptyStatement
     *  : ';'
     *  ;
     */
    fn empty_statement(&mut self) -> StatementRef;

    /**
     * ExpressionStatement
     *  : Expression ';'
     *  ;
     */
    fn expression_statement(&mut self, consume_statement_end: bool) -> StatementRef;
}

impl<'a> ParseStatements for Parser<'a> {
    fn program(&mut self) -> StatementRef {
        let statement_list = self.statement_list(None);
        Box::new(Statement::Program {
            body: statement_list,
        })
    }

    fn block_statement(&mut self) -> StatementRef {
        eat(self, TokenType::OpeningBrace);

        let block = if self.lookahead.token_type != TokenType::ClosingBrace {
            self.statement_list(Some(TokenType::ClosingBrace))
        } else {
            vec![]
        };

        eat(self, TokenType::ClosingBrace);

        Box::new(Statement::Block { body: block })
    }

    fn statement_list(&mut self, stop_token_type: Option<TokenType>) -> StatementList {
        let mut statement_list: Vec<Statement> = vec![];

        while self.lookahead.token_type != TokenType::End
            && self.lookahead.token_type != stop_token_type.unwrap_or(TokenType::End)
        {
            let statement = self.statement();
            statement_list.push(*statement);
        }

        statement_list
    }

    fn statement(&mut self) -> StatementRef {
        match self.lookahead.token_type {
            TokenType::StatementEnd => self.empty_statement(),
            TokenType::OpeningBrace => self.block_statement(),
            TokenType::LetKeyword => self.variable_declaration_statement(true),
            TokenType::IfKeyword => self.if_statement(),
            TokenType::WhileKeyword => self.while_statement(),
            TokenType::DoKeyword => self.do_while_statement(),
            TokenType::ForKeyword => self.for_statement(),
            _ => self.expression_statement(true),
        }
    }

    fn variable_declaration_statement(&mut self, consume_statement_end: bool) -> StatementRef {
        let mut variables: Vec<Expression> = vec![];

        eat(self, TokenType::LetKeyword);
        loop {
            variables.push(*self.variable_initialization_expression());

            if !is_token(self, TokenType::Comma) {
                break;
            }

            eat(self, TokenType::Comma);
        }
        if consume_statement_end {
            eat(self, TokenType::StatementEnd);
        }

        Box::new(Statement::VariableDeclaration { variables })
    }

    fn if_statement(&mut self) -> StatementRef {
        eat(self, TokenType::IfKeyword);

        eat(self, TokenType::OpeningParenthesis);
        let condition = self.expression();
        eat(self, TokenType::ClosingParenthesis);

        let consequent = self.statement();

        let alternative = if is_token(self, TokenType::ElseKeyword) {
            eat(self, TokenType::ElseKeyword);
            Some(self.statement())
        } else {
            None
        };

        Box::new(Statement::Conditional {
            condition,
            consequent,
            alternative,
        })
    }

    fn while_statement(&mut self) -> StatementRef {
        eat(self, TokenType::WhileKeyword);

        eat(self, TokenType::OpeningParenthesis);
        let condition = self.expression();
        eat(self, TokenType::ClosingParenthesis);

        let body = self.statement();

        Box::new(Statement::While { condition, body })
    }

    fn do_while_statement(&mut self) -> StatementRef {
        eat(self, TokenType::DoKeyword);

        let body = self.statement();

        eat(self, TokenType::WhileKeyword);

        eat(self, TokenType::OpeningParenthesis);
        let condition = self.expression();
        eat(self, TokenType::ClosingParenthesis);

        eat(self, TokenType::StatementEnd);

        Box::new(Statement::DoWhile { body, condition })
    }

    fn for_statement(&mut self) -> StatementRef {
        eat(self, TokenType::ForKeyword);
        eat(self, TokenType::OpeningParenthesis);

        let initializer = if is_token(self, TokenType::StatementEnd) {
            None
        } else {
            Some(self.for_statement_init())
        };
        eat(self, TokenType::StatementEnd);

        let condition = if is_token(self, TokenType::StatementEnd) {
            None
        } else {
            Some(self.expression())
        };
        eat(self, TokenType::StatementEnd);

        let increment = if is_token(self, TokenType::ClosingParenthesis) {
            None
        } else {
            Some(self.expression())
        };
        eat(self, TokenType::ClosingParenthesis);

        let body = self.statement();

        Box::new(Statement::For {
            initializer,
            condition,
            increment,
            body,
        })
    }

    fn for_statement_init(&mut self) -> StatementRef {
        if is_token(self, TokenType::LetKeyword) {
            return self.variable_declaration_statement(false);
        }
        self.expression_statement(false)
    }

    fn empty_statement(&mut self) -> StatementRef {
        eat(self, TokenType::StatementEnd);

        Box::new(Statement::Empty)
    }

    fn expression_statement(&mut self, consume_statement_end: bool) -> StatementRef {
        let expression = self.expression();

        if consume_statement_end {
            eat(self, TokenType::StatementEnd);
        }

        Box::new(Statement::Expression { expression })
    }
}
