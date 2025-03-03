use crate::{Expression, Parser};

#[test]
fn test() {
    let parser = Parser::new();

    let source = "12";

    let ast = parser.parse(source);

    assert_eq!(ast, Expression::NumericLiteral(12))
}
