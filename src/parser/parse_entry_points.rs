use std::rc::Rc;

use super::{
    parse_expressions::ParseExpressions, parse_statements::ParseStatements, Parser, Statement,
    StatementRef,
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
}

impl<'a> ParseEntryPoints for Parser<'a> {
    fn program_root(&mut self) -> StatementRef
    where
        Self: ParseExpressions,
    {
        let statement_list = self.statement_list(None);
        Rc::new(Statement::Program {
            body: statement_list,
        })
    }
}
