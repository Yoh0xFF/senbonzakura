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
    fn statement_list(&mut self) -> Rc<Vec<Rc<Expression>>>;

    /**
     * Satement
     *  : ExpressionStatement
     *  ;
     */
    fn statement(&mut self) -> Rc<Expression>;

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
    fn statement_list(&mut self) -> Rc<Vec<Rc<Expression>>> {
        let mut statement_list: Vec<Rc<Expression>> = vec![];

        while self.lookahead.token_type != TokenType::End {
            let statement = self.statement();
            statement_list.push(statement);
        }

        Rc::new(statement_list)
    }

    fn statement(&mut self) -> Rc<Expression> {
        self.expression_statement()
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
