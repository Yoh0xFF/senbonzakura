use crate::{parser::StatementRef, Parser};

pub(super) fn execute(source: &str, expected: StatementRef) {
    let mut parser = Parser::new(source);

    let ast = parser.parse();

    assert_eq!(ast, expected);
}
