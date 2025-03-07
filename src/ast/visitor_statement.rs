use super::{BaseVisitor, ExpressionRef, StatementList, StatementRef};

pub trait StatementVisitor: BaseVisitor {
    fn visit_statement(&mut self, statement: StatementRef) -> Result<Self::Output, Self::Error> {
        match statement.as_ref() {
            super::Statement::Program { body } => {
                return self.visit_program_statement(body.clone());
            }
            super::Statement::Block { body } => {
                return self.visit_block_statement(body.clone());
            }
            super::Statement::Empty => return self.visit_empty_statement(),
            super::Statement::Expression { expression } => {
                return self.visit_expression_statement(expression.clone());
            }
        }
    }

    fn visit_program_statement(&mut self, body: StatementList)
        -> Result<Self::Output, Self::Error>;

    fn visit_block_statement(&mut self, body: StatementList) -> Result<Self::Output, Self::Error>;

    fn visit_empty_statement(&mut self) -> Result<Self::Output, Self::Error>;

    fn visit_expression_statement(
        &mut self,
        expression: ExpressionRef,
    ) -> Result<Self::Output, Self::Error>;
}
