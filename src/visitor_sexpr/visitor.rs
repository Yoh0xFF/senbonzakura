pub struct SExpVisitor;

impl BaseVisitor for SExpVisitor {
    type Output = String;
    type Error = anyhow::Error;
}

/**
 * Implement Acceptor for Statement
 */
impl SExpAcceptor for StatementRef {
    fn accept(&self, visitor: &SExpVisitor) -> Result<String, anyhow::Error> {
        visitor.visit_statement(self)
    }
}

/**
 * Implement Acceptor for Expression
 */
impl SExpAcceptor for ExpressionRef {
    fn accept(&self, visitor: &SExpVisitor) -> Result<String, anyhow::Error> {
        visitor.visit_expression(self)
    }
}
