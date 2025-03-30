use super::parser_tests::execute;

#[test]
fn test_greater_than_operator() {
    execute(
        "x > 5;",
        r#"
        (program
            (expr (binary ">" (id x) (number 5))))
        "#,
    )
}

#[test]
fn test_greater_than_or_equal_to_operator() {
    execute(
        "x >= 10;",
        r#"
        (program
            (expr (binary ">=" (id x) (number 10))))
        "#,
    )
}

#[test]
fn test_less_than_operator() {
    execute(
        "y < 20;",
        r#"
        (program
            (expr (binary "<" (id y) (number 20))))
        "#,
    )
}

#[test]
fn test_less_than_or_equal_to_operator() {
    execute(
        "y <= 30;",
        r#"
        (program
            (expr (binary "<=" (id y) (number 30))))
        "#,
    )
}

#[test]
fn test_relational_operators_with_variables() {
    execute(
        "x > y;",
        r#"
        (program
            (expr (binary ">" (id x) (id y))))
        "#,
    )
}

#[test]
fn test_relational_operators_with_expressions() {
    execute(
        "x + 5 > y * 2;",
        r#"
        (program
            (expr (binary ">" (binary "+" (id x) (number 5)) (binary "*" (id y) (number 2)))))
        "#,
    )
}

#[test]
fn test_chained_relational_expressions() {
    execute(
        "x > 5 > y;", // Note: This is parsed as (x > 5) > y, which might not be what users expect
        r#"
        (program
            (expr (binary ">" (binary ">" (id x) (number 5)) (id y))))
        "#,
    )
}

#[test]
fn test_relational_operators_with_parenthesized_expressions() {
    execute(
        "(x + 10) > (y - 5);",
        r#"
        (program
            (expr (binary ">" (binary "+" (id x) (number 10)) (binary "-" (id y) (number 5)))))
        "#,
    )
}

#[test]
fn test_relational_operators_precedence_with_assignment() {
    execute(
        "result = x > y;",
        r#"
        (program
            (expr (assign "=" (id result) (binary ">" (id x) (id y)))))
        "#,
    )
}
