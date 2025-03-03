use std::rc::Rc;

use crate::{Expression, Parser};

#[test]
fn test_string_literal() {
    let mut parser = Parser::new();

    let source = "\"Hello\"";

    let ast = parser.parse(source);

    assert_eq!(
        ast,
        Expression::Program {
            body: Rc::new(Expression::StringLiteral("Hello".to_string()))
        }
    )
}

#[test]
fn test_number_literal() {
    let mut parser = Parser::new();

    let source = "12";

    let ast = parser.parse(source);

    assert_eq!(
        ast,
        Expression::Program {
            body: Rc::new(Expression::NumericLiteral(12))
        }
    )
}
