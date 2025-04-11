use crate::ast::{Expression, Statement, StatementList, StatementRef};
use crate::lexer::TokenType;

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
    fn variable_declaration_statement(&mut self) -> StatementRef;

    /**
     * ConditionalStatement
     *  : if '(' Expression ')' Statement [else Statement]
     */
    fn if_statement(&mut self) -> StatementRef;

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
    fn expression_statement(&mut self) -> StatementRef;
}

impl<'a> ParseStatements for Parser<'a> {
    fn program(&mut self) -> StatementRef {
        let statement_list = self.statement_list(None);
        Box::new(Statement::Program {
            body: statement_list,
        })
    }

    fn block_statement(&mut self) -> StatementRef {
        self.eat(TokenType::OpeningBrace);

        let block = if self.lookahead.token_type != TokenType::ClosingBrace {
            self.statement_list(Some(TokenType::ClosingBrace))
        } else {
            vec![]
        };

        self.eat(TokenType::ClosingBrace);

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
            TokenType::LetKeyword => self.variable_declaration_statement(),
            TokenType::IfKeyword => self.if_statement(),
            _ => self.expression_statement(),
        }
    }

    fn variable_declaration_statement(&mut self) -> StatementRef {
        let mut variables: Vec<Expression> = vec![];

        self.eat(TokenType::LetKeyword);
        loop {
            variables.push(*self.variable_initialization_expression());

            if !self.is_token(TokenType::Comma) {
                break;
            }

            self.eat(TokenType::Comma);
        }
        self.eat(TokenType::StatementEnd);

        Box::new(Statement::VariableDeclaration {
            variables,
        })
    }

    fn if_statement(&mut self) -> StatementRef {
        self.eat(TokenType::IfKeyword);

        self.eat(TokenType::OpeningParenthesis);
        let condition = self.expression();
        self.eat(TokenType::ClosingParenthesis);

        let consequent = self.statement();

        let alternative = if self.is_token(TokenType::ElseKeyword) {
            self.eat(TokenType::ElseKeyword);
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

    fn empty_statement(&mut self) -> StatementRef {
        self.eat(TokenType::StatementEnd);

        Box::new(Statement::Empty)
    }

    fn expression_statement(&mut self) -> StatementRef {
        let expression = self.expression();

        self.eat(TokenType::StatementEnd);

        Box::new(Statement::Expression { expression })
    }
}
