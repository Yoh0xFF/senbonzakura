use std::rc::Rc;

use super::{
    parse_literals::ParseLiterals, parse_statements::ParseStatements, ExpressionRef, Parser,
    Statement, StatementRef,
};

pub(super) trait ParseEntryPoints {
    /**
     * Main entry point
     *
     * Program
     *  : StatementList
     *  ;
     */
    fn program_root(&mut self) -> StatementRef;

    /**
     * ExpressionStatement
     *  : Literal
     *  ;
     */
    fn expression_root(&mut self) -> ExpressionRef;
}

impl<'a> ParseEntryPoints for Parser<'a> {
    fn program_root(&mut self) -> StatementRef
    where
        Self: ParseLiterals,
    {
        let statement_list = self.statement_list(None);
        Rc::new(Statement::Program {
            body: statement_list,
        })
    }

    fn expression_root(&mut self) -> ExpressionRef
    where
        Self: ParseLiterals,
    {
        self.literal()
    }
}
