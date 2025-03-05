use std::rc::Rc;

use crate::lexer::TokenType;

use super::{parse_literals::ParseLiterals, Expression, Parser};

pub(super) trait ParseRootExpression {
    /**
     * StatementList
     *  : Statement
     *  | StatementList Statement
     *  ;
     */
    fn statement_list(&mut self, stop_token_type: Option<TokenType>) -> Rc<Vec<Rc<Expression>>>;

    /**
     * Satement
     *  : ExpressionStatement
     *  | BlockStatement
     *  ;
     */
    fn statement(&mut self) -> Rc<Expression>;

    /**
     * BlockStatement
     *  : '{' OptStatementList '}'
     *  ;
     */
    fn block_statement(&mut self) -> Rc<Expression>;

    /**
     * EmptyStatement
     *  : ';'
     *  ;
     */
    fn empty_statement(&mut self) -> Rc<Expression>;

    /**
     * ExpressionStatement
     *  : Expression ';'
     *  ;
     */
    fn expression_statement(&mut self) -> Rc<Expression>;

    /**
     * ExpressionStatement
     *  : Literal
     *  ;
     */
    fn expression(&mut self) -> Rc<Expression>;
}

impl<'a> ParseRootExpression for Parser<'a> {
    fn statement_list(&mut self, stop_token_type: Option<TokenType>) -> Rc<Vec<Rc<Expression>>> {
        let mut statement_list: Vec<Rc<Expression>> = vec![];

        while self.lookahead.token_type != TokenType::End
            && self.lookahead.token_type != stop_token_type.unwrap_or(TokenType::End)
        {
            let statement = self.statement();
            statement_list.push(statement);
        }

        Rc::new(statement_list)
    }

    fn statement(&mut self) -> Rc<Expression> {
        match self.lookahead.token_type {
            TokenType::StatementEnd => return self.empty_statement(),
            TokenType::OpeningBrace => return self.block_statement(),
            _ => return self.expression_statement(),
        }
    }

    fn block_statement(&mut self) -> Rc<Expression> {
        self.eat(TokenType::OpeningBrace);

        let block = if self.lookahead.token_type != TokenType::ClosingBrace {
            self.statement_list(Some(TokenType::ClosingBrace))
        } else {
            Rc::new(vec![])
        };

        self.eat(TokenType::ClosingBrace);

        Rc::new(Expression::BlockStatement { body: block })
    }

    fn empty_statement(&mut self) -> Rc<Expression> {
        self.eat(TokenType::StatementEnd);

        Rc::new(Expression::EmptyStatement)
    }

    fn expression_statement(&mut self) -> Rc<Expression> {
        let expression = self.expression();

        self.eat(TokenType::StatementEnd);

        Rc::new(Expression::ExpressionStatement { expression })
    }

    fn expression(&mut self) -> Rc<Expression>
    where
        Self: ParseLiterals,
    {
        self.literal()
    }
}
