use super::parser_tests::execute;

#[test]
fn test_logical_and_operator() {
    execute(
        "x && y;",
        r#"
        (program
            (expr (logical "&&" (id x) (id y))))
        "#,
    )
}

#[test]
fn test_logical_or_operator() {
    execute(
        "x || y;",
        r#"
        (program
            (expr (logical "||" (id x) (id y))))
        "#,
    )
}

#[test]
fn test_chained_logical_and_operators() {
    execute(
        "a && b && c;",
        r#"
        (program
            (expr (logical "&&" (logical "&&" (id a) (id b)) (id c))))
        "#,
    )
}

#[test]
fn test_chained_logical_or_operators() {
    execute(
        "a || b || c;",
        r#"
        (program
            (expr (logical "||" (logical "||" (id a) (id b)) (id c))))
        "#,
    )
}

#[test]
fn test_mixed_logical_operators() {
    execute(
        "a && b || c;",
        r#"
        (program
            (expr (logical "||" (logical "&&" (id a) (id b)) (id c))))
        "#,
    )
}

#[test]
fn test_logical_operators_with_parentheses() {
    execute(
        "a && (b || c);",
        r#"
        (program
            (expr (logical "&&" (id a) (logical "||" (id b) (id c)))))
        "#,
    )
}

#[test]
fn test_logical_operators_with_boolean_literals() {
    execute(
        "true && false;",
        r#"
        (program
            (expr (logical "&&" (boolean true) (boolean false))))
        "#,
    )
}

#[test]
fn test_logical_operators_with_comparison_expressions() {
    execute(
        "x > 5 && y < 10;",
        r#"
        (program
            (expr (logical "&&" (binary ">" (id x) (number 5)) (binary "<" (id y) (number 10)))))
        "#,
    )
}

#[test]
fn test_logical_operators_with_equality_expressions() {
    execute(
        "x == 5 || y != 10;",
        r#"
        (program
            (expr (logical "||" (binary "==" (id x) (number 5)) (binary "!=" (id y) (number 10)))))
        "#,
    )
}

#[test]
fn test_complex_logical_expression() {
    execute(
        "(a > b && c < d) || (e == f && g != h);",
        r#"
        (program
            (expr (logical "||"
                (logical "&&" (binary ">" (id a) (id b)) (binary "<" (id c) (id d)))
                (logical "&&" (binary "==" (id e) (id f)) (binary "!=" (id g) (id h))))))
        "#,
    )
}

#[test]
fn test_assignment_with_logical_expression() {
    execute(
        "result = a && b || c;",
        r#"
        (program
            (expr (assign "=" (id result) (logical "||" (logical "&&" (id a) (id b)) (id c)))))
        "#,
    )
}

#[test]
fn test_variable_initialization_with_logical_expression() {
    execute(
        "let isValid = x > 0 && x < 100;",
        r#"
        (program
            (let
                (init
                    (id isValid)
                    (logical "&&" (binary ">" (id x) (number 0)) (binary "<" (id x) (number 100))))))
        "#,
    )
}

#[test]
fn test_logical_expression_in_if_condition() {
    execute(
        r#"
        if (x > 0 && x < 100) {
            y = 1;
        } else {
            y = 0;
        }
        "#,
        r#"
        (program
            (if
                (logical "&&" (binary ">" (id x) (number 0)) (binary "<" (id x) (number 100)))
                (block
                    (expr (assign "=" (id y) (number 1))))
                (block
                    (expr (assign "=" (id y) (number 0))))))
        "#,
    )
}
