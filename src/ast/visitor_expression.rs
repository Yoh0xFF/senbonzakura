use super::{BaseVisitor, Expression, ExpressionRef};

pub trait ExpressionVisitor: BaseVisitor {
    fn visit_expression(&mut self, expression: ExpressionRef) -> Result<Self::Output, Self::Error> {
        match expression.as_ref() {
            Expression::Binary {
                operator,
                left,
                right,
            } => return self.visit_binary_expression(operator, left.clone(), right.clone()),
            Expression::StringLiteral(value) => return self.visit_string_literal_expression(value),
            Expression::NumericLiteral(value) => {
                return self.visit_numeric_literal_expression(*value)
            }
        }
    }

    fn visit_binary_expression(
        &mut self,
        operator: &str,
        left: ExpressionRef,
        right: ExpressionRef,
    ) -> Result<Self::Output, Self::Error>;

    fn visit_string_literal_expression(&mut self, value: &str)
        -> Result<Self::Output, Self::Error>;

    fn visit_numeric_literal_expression(&mut self, value: i32)
        -> Result<Self::Output, Self::Error>;
}
