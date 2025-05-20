use super::internal_util::execute;

#[test]
fn test_binary_expression_with_additive_operator() {
    execute(
        "1 + 2;",
        r#"
        (program
            (expr (binary "+" (number 1) (number 2))))
        "#,
    )
}

#[test]
fn test_binary_expression_with_multiple_additive_operators() {
    execute(
        "1 + 2 + 3;",
        r#"
        (program
            (expr (binary "+" (binary "+" (number 1) (number 2)) (number 3))))
        "#,
    )
}

#[test]
fn test_binary_expression_with_multiple_additive_operators_and_parentheses() {
    execute(
        "1 + (2 + 3);",
        r#"
        (program
            (expr (binary "+" (number 1) (binary "+" (number 2) (number 3)))))
        "#,
    )
}

#[test]
fn test_binary_expression_with_multiple_additive_and_factor_operators() {
    execute(
        "1 + 2 * 3;",
        r#"
        (program
            (expr (binary "+" (number 1) (binary "*" (number 2) (number 3)))))
        "#,
    )
}

#[test]
fn test_binary_expression_with_multiple_additive_and_factor_operators_and_parentheses() {
    execute(
        "(1 + 2) * 3;",
        r#"
        (program
            (expr (binary "*" (binary "+" (number 1) (number 2)) (number 3))))
        "#,
    )
}
