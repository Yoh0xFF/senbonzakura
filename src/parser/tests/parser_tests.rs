use crate::{Parser, ToSExpression};

pub(super) fn execute(source: &str, expected_sexpression: &str) {
    let mut parser = Parser::new(source);

    let ast = parser.parse();
    let actual_sexp = ast
        .to_sexpression()
        .expect("Failed to convert AST to s-expression");

    assert_eq!(actual_sexp, expected_sexpression);
}
