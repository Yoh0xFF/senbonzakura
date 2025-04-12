use super::utils::execute;

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