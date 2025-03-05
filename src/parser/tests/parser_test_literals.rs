use std::rc::Rc;

use crate::Expression;

use super::parser_tests::execute;

#[test]
fn test_string_literal() {
    execute(
        r#""Hello""#,
        Rc::new(Expression::Program {
            body: Rc::new(Expression::StringLiteral("Hello".to_string())),
        }),
    )
}

#[test]
fn test_number_literal() {
    execute(
        "12",
        Rc::new(Expression::Program {
            body: Rc::new(Expression::NumericLiteral(12)),
        }),
    )
}
