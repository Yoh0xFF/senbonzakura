use crate::{Parser, ToSExpression};

pub(super) fn execute(source: &str, expected_sexpression: &str) {
    let mut parser = Parser::new(source);

    let ast = parser.parse();
    let actual_sexp = ast
        .to_sexpression()
        .expect("Failed to convert AST to s-expression");

    // Normalize expected s-expression by removing indentation
    let normalized_expected_sexpression = expected_sexpression
        .lines()
        .map(|line| line.trim())
        .filter(|line| !line.is_empty())
        .collect::<Vec<&str>>()
        .join(" ");

    assert_eq!(actual_sexp, normalized_expected_sexpression);
}
