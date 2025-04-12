use crate::parser::root_statement;
use crate::{Parser, ToSExpression};

pub(super) fn execute(source: &str, expected_s_expression: &str) {
    let mut parser = Parser::new(source);

    let ast = root_statement(&mut parser);
    let actual_s_expression = ast
        .to_s_expression()
        .expect("Failed to convert AST to s-expression");

    // Normalize expected s-expression by removing indentation
    let mut normalized_expected_s_expression = expected_s_expression
        .replace('\n', " ") // Replace newlines with spaces
        .split_whitespace() // Split by whitespace
        .collect::<Vec<&str>>()
        .join(" "); // Join with single spaces
    normalized_expected_s_expression = normalized_expected_s_expression.replace(" )", ")");

    assert_eq!(actual_s_expression, normalized_expected_s_expression);
}
