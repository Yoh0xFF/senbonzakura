use std::rc::Rc;

use crate::{Expression, Parser};

pub(super) fn execute(source: &str, expected: Rc<Expression>) {
    let mut parser = Parser::new(source);

    let ast = parser.parse();

    assert_eq!(ast, *expected);
}
