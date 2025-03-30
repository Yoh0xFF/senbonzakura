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

#[test]
fn test_boolean_true_literal() {
    execute(
        "true;",
        r#"
        (program
            (expr (boolean true)))
        "#,
    )
}

#[test]
fn test_boolean_false_literal() {
    execute(
        "false;",
        r#"
        (program
            (expr (boolean false)))
        "#,
    )
}

#[test]
fn test_nil_literal() {
    execute(
        "nil;",
        r#"
        (program
            (expr (nil)))
        "#,
    )
}

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
fn test_boolean_in_if_condition() {
    execute(
        r#"
        if (true) {
            x = 10;
        }
        "#,
        r#"
        (program
            (if
                (boolean true)
                (block
                    (expr (assign "=" (id x) (number 10))))))
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
