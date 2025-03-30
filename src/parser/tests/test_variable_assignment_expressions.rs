use super::parser_tests::execute;

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
