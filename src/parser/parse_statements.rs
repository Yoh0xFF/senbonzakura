use std::rc::Rc;

use crate::lexer::TokenType;

use super::{parse_entry_points::ParseEntryPoints, Parser, Statement};

pub(super) trait ParseStatements {
    /**
     * BlockStatement
     *  : '{' OptStatementList '}'
     *  ;
     */
    fn block_statement(&mut self) -> Rc<Statement>;

    /**
     * StatementList
     *  : Statement
     *  | StatementList Statement
     *  ;
     */
    fn statement_list(&mut self, stop_token_type: Option<TokenType>) -> Rc<Vec<Rc<Statement>>>;

    /**
     * Satement
     *  : ExpressionStatement
     *  | BlockStatement
     *  ;
     */
    fn statement(&mut self) -> Rc<Statement>;

    /**
     * EmptyStatement
     *  : ';'
     *  ;
     */
    fn empty_statement(&mut self) -> Rc<Statement>;

    /**
     * ExpressionStatement
     *  : Expression ';'
     *  ;
     */
    fn expression_statement(&mut self) -> Rc<Statement>;
}

impl<'a> ParseStatements for Parser<'a> {
    fn block_statement(&mut self) -> Rc<Statement> {
        self.eat(TokenType::OpeningBrace);

        let block = if self.lookahead.token_type != TokenType::ClosingBrace {
            self.statement_list(Some(TokenType::ClosingBrace))
        } else {
            Rc::new(vec![])
        };

        self.eat(TokenType::ClosingBrace);

        Rc::new(Statement::BlockStatement { body: block })
    }

    fn statement_list(&mut self, stop_token_type: Option<TokenType>) -> Rc<Vec<Rc<Statement>>> {
        let mut statement_list: Vec<Rc<Statement>> = vec![];

        while self.lookahead.token_type != TokenType::End
            && self.lookahead.token_type != stop_token_type.unwrap_or(TokenType::End)
        {
            let statement = self.statement();
            statement_list.push(statement);
        }

        Rc::new(statement_list)
    }

    fn statement(&mut self) -> Rc<Statement> {
        match self.lookahead.token_type {
            TokenType::StatementEnd => return self.empty_statement(),
            TokenType::OpeningBrace => return self.block_statement(),
            _ => return self.expression_statement(),
        }
    }

    fn empty_statement(&mut self) -> Rc<Statement> {
        self.eat(TokenType::StatementEnd);

        Rc::new(Statement::EmptyStatement)
    }

    fn expression_statement(&mut self) -> Rc<Statement>
    where
        Self: ParseEntryPoints,
    {
        let expression = self.expression_root();

        self.eat(TokenType::StatementEnd);

        Rc::new(Statement::ExpressionStatement { expression })
    }
}
