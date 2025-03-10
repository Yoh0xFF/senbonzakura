use super::parser_tests::execute;

#[test]
fn test_string_literal() {
    execute(
        r#""Hello";"#,
        r#"
        (program
            (expr (string "Hello")))
        "#,
    )
}

#[test]
fn test_number_literal() {
    execute(
        "12;",
        r#"
        (program
            (expr (number 12)))
        "#,
    )
}

#[test]
fn test_multiple_literal() {
    execute(
        r#"
        "Hello";

        17;
        "#,
        r#"
        (program
            (expr (string "Hello"))
            (expr (number 17)))
        "#,
    )
}

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

#[test]
fn test_simple_assignment_expression() {
    execute(
        "x = 5;",
        r#"
        (program
            (expr (assign "=" (id x) (number 5))))
        "#,
    )
}

#[test]
fn test_complex_assignment_expression() {
    execute(
        "x += 10;",
        r#"
        (program
            (expr (assign "+=" (id x) (number 10))))
        "#,
    )
}

#[test]
fn test_assignment_with_binary_expression() {
    execute(
        "x = 5 + 10;",
        r#"
        (program
            (expr (assign "=" (id x) (binary "+" (number 5) (number 10)))))
        "#,
    )
}

#[test]
fn test_chained_assignment() {
    execute(
        "x = y = 5;",
        r#"
        (program
            (expr (assign "=" (id x) (assign "=" (id y) (number 5)))))
        "#,
    )
}
