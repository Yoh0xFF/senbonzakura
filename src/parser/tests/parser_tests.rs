use std::rc::Rc;

use crate::{parser::Statement, Parser};

pub(super) fn execute(source: &str, expected: Rc<Statement>) {
    let mut parser = Parser::new(source);

    let ast = parser.parse();

    assert_eq!(ast, expected);
}
