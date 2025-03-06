use std::rc::Rc;

use super::{
    parse_literals::ParseLiterals, parse_statements::ParseStatements, Expression, Parser, Statement,
};

pub(super) trait ParseEntryPoints {
    /**
     * Main entry point
     *
     * Program
     *  : StatementList
     *  ;
     */
    fn program_root(&mut self) -> Rc<Statement>;

    /**
     * ExpressionStatement
     *  : Literal
     *  ;
     */
    fn expression_root(&mut self) -> Rc<Expression>;
}

impl<'a> ParseEntryPoints for Parser<'a> {
    fn program_root(&mut self) -> Rc<Statement>
    where
        Self: ParseLiterals,
    {
        let statement_list = self.statement_list(None);
        Rc::new(Statement::Program {
            body: statement_list,
        })
    }

    fn expression_root(&mut self) -> Rc<Expression>
    where
        Self: ParseLiterals,
    {
        self.literal()
    }
}
