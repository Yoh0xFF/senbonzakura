use super::parser_tests::execute;

#[test]
fn test_equality_operator_equal() {
    execute(
        "x == y;",
        r#"
        (program
            (expr (binary "==" (id x) (id y))))
        "#,
    )
}

#[test]
fn test_equality_operator_not_equal() {
    execute(
        "x != y;",
        r#"
        (program
            (expr (binary "!=" (id x) (id y))))
        "#,
    )
}

#[test]
fn test_equality_operator_with_literals() {
    execute(
        "42 == 42;",
        r#"
        (program
            (expr (binary "==" (number 42) (number 42))))
        "#,
    )
}

#[test]
fn test_equality_operator_with_boolean_literals() {
    execute(
        "true == false;",
        r#"
        (program
            (expr (binary "==" (boolean true) (boolean false))))
        "#,
    )
}

#[test]
fn test_equality_operator_with_nil() {
    execute(
        "x == nil;",
        r#"
        (program
            (expr (binary "==" (id x) (nil))))
        "#,
    )
}

#[test]
fn test_complex_equality_expression() {
    execute(
        "x + 5 == y * 2;",
        r#"
        (program
            (expr (binary "==" (binary "+" (id x) (number 5)) (binary "*" (id y) (number 2)))))
        "#,
    )
}

#[test]
fn test_chained_equality_expressions() {
    execute(
        "x == y == true;", // Note: This is parsed as (x == y) == true, following precedence rules
        r#"
        (program
            (expr (binary "==" (binary "==" (id x) (id y)) (boolean true))))
        "#,
    )
}

#[test]
fn test_equality_with_assignment() {
    execute(
        "result = x == y;",
        r#"
        (program
            (expr (assign "=" (id result) (binary "==" (id x) (id y)))))
        "#,
    )
}

#[test]
fn test_complex_boolean_expressions() {
    execute(
        r#"
        isValid = (count > 0) == true;
        "#,
        r#"
        (program
            (expr (assign "=" (id isValid) (binary "==" (binary ">" (id count) (number 0)) (boolean true)))))
        "#,
    )
}
