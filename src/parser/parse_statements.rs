use std::rc::Rc;

use crate::ast::{Statement, StatementList, StatementNode};
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
    fn program(&mut self) -> Statement;

    /**
     * BlockStatement
     *  : '{' OptStatementList '}'
     *  ;
     */
    fn block_statement(&mut self) -> Statement;

    /**
     * StatementList
     *  : Statement
     *  | StatementList Statement
     *  ;
     */
    fn statement_list(&mut self, stop_token_type: Option<TokenType>) -> StatementList;

    /**
     * Satement
     *  : ExpressionStatement
     *  | BlockStatement
     *  ;
     */
    fn statement(&mut self) -> Statement;

    /**
     * EmptyStatement
     *  : ';'
     *  ;
     */
    fn empty_statement(&mut self) -> Statement;

    /**
     * ExpressionStatement
     *  : Expression ';'
     *  ;
     */
    fn expression_statement(&mut self) -> Statement;
}

impl<'a> ParseStatements for Parser<'a> {
    fn program(&mut self) -> Statement {
        let statement_list = self.statement_list(None);
        Rc::new(StatementNode::Program {
            body: statement_list,
        })
    }

    fn block_statement(&mut self) -> Statement {
        self.eat(TokenType::OpeningBrace);

        let block = if self.lookahead.token_type != TokenType::ClosingBrace {
            self.statement_list(Some(TokenType::ClosingBrace))
        } else {
            Rc::new(vec![])
        };

        self.eat(TokenType::ClosingBrace);

        Rc::new(StatementNode::Block { body: block })
    }

    fn statement_list(&mut self, stop_token_type: Option<TokenType>) -> StatementList {
        let mut statement_list: Vec<Statement> = vec![];

        while self.lookahead.token_type != TokenType::End
            && self.lookahead.token_type != stop_token_type.unwrap_or(TokenType::End)
        {
            let statement = self.statement();
            statement_list.push(statement);
        }

        Rc::new(statement_list)
    }

    fn statement(&mut self) -> Statement {
        match self.lookahead.token_type {
            TokenType::StatementEnd => return self.empty_statement(),
            TokenType::OpeningBrace => return self.block_statement(),
            _ => return self.expression_statement(),
        }
    }

    fn empty_statement(&mut self) -> Statement {
        self.eat(TokenType::StatementEnd);

        Rc::new(StatementNode::Empty)
    }

    fn expression_statement(&mut self) -> Statement {
        let expression = self.expression();

        self.eat(TokenType::StatementEnd);

        Rc::new(StatementNode::Expression { expression })
    }
}
