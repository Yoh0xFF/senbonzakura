use crate::{Parser, ToSExpression};
use crate::parser::root_statement;

pub(super) fn execute(source: &str, expected_sexpression: &str) {
    let mut parser = Parser::new(source);

    let ast = root_statement(&mut parser);
    let actual_sexp = ast
        .to_s_expression()
        .expect("Failed to convert AST to s-expression");

    // Normalize expected s-expression by removing indentation
    let mut normalized_expected_sexpression = expected_sexpression
        .replace('\n', " ") // Replace newlines with spaces
        .split_whitespace() // Split by whitespace
        .collect::<Vec<&str>>()
        .join(" "); // Join with single spaces
    normalized_expected_sexpression = normalized_expected_sexpression.replace(" )", ")");

    assert_eq!(actual_sexp, normalized_expected_sexpression);
}
