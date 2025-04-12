use super::utils::execute;

#[test]
fn test_unary_plus_operator() {
    execute(
        "+42;",
        r#"
        (program
          (expr (unary "+" (number 42))))
        "#,
    )
}

#[test]
fn test_unary_minus_operator() {
    execute(
        "-42;",
        r#"
        (program
            (expr (unary "-" (number 42))))
        "#,
    )
}

#[test]
fn test_unary_not_operator() {
    execute(
        "!true;",
        r#"
        (program
            (expr (unary "!" (boolean true))))
        "#,
    )
}

#[test]
fn test_unary_not_with_identifier() {
    execute(
        "!isValid;",
        r#"
        (program
            (expr (unary "!" (id isValid))))
        "#,
    )
}

#[test]
fn test_chained_unary_operators() {
    execute(
        "+-42;",
        r#"
        (program
            (expr (unary "+" (unary "-" (number 42)))))
        "#,
    )
}

#[test]
fn test_multiple_chained_unary_operators() {
    execute(
        "+-!x;",
        r#"
        (program
            (expr (unary "+" (unary "-" (unary "!" (id x))))))
        "#,
    )
}

#[test]
fn test_unary_operator_with_parentheses() {
    execute(
        "-(x + y);",
        r#"
        (program
            (expr (unary "-" (binary "+" (id x) (id y)))))
        "#,
    )
}

#[test]
fn test_unary_operators_precedence_with_binary_operators() {
    execute(
        "-x + y;",
        r#"
        (program
            (expr (binary "+" (unary "-" (id x)) (id y))))
        "#,
    )
}

#[test]
fn test_unary_operators_with_assignment() {
    execute(
        "result = -x;",
        r#"
        (program
            (expr (assign "=" (id result) (unary "-" (id x)))))
        "#,
    )
}

#[test]
fn test_unary_minus_with_numeric_literal() {
    execute(
        "let x = -5;",
        r#"
        (program
            (let
                (init
                    (id x)
                    (unary "-" (number 5)))))
        "#,
    )
}

#[test]
fn test_unary_operators_in_binary_expressions() {
    execute(
        "x * -y;",
        r#"
        (program
            (expr (binary "*" (id x) (unary "-" (id y)))))
        "#,
    )
}

#[test]
fn test_unary_not_with_equality_expressions() {
    execute(
        "!(x == y);",
        r#"
        (program
            (expr (unary "!" (binary "==" (id x) (id y)))))
        "#,
    )
}

#[test]
fn test_unary_operators_with_function_calls() {
    // Once you implement function calls, this test would be valuable
    // For now, focusing on existing functionality
    execute(
        "-x + -y;",
        r#"
        (program
            (expr (binary "+" (unary "-" (id x)) (unary "-" (id y)))))
        "#,
    )
}

#[test]
fn test_unary_operators_in_conditional_expressions() {
    execute(
        r#"
        if (!condition) {
            x = -y;
        }
        "#,
        r#"
        (program
            (if
                (unary "!" (id condition))
                (block
                    (expr (assign "=" (id x) (unary "-" (id y)))))))
        "#,
    )
}

#[test]
fn test_unary_not_with_logical_expressions() {
    execute(
        "!(x && y);",
        r#"
        (program
            (expr (unary "!" (logical "&&" (id x) (id y)))))
        "#,
    )
}

#[test]
fn test_unary_operators_precedence_with_multiplication() {
    execute(
        "-x * y;",
        r#"
        (program
            (expr
                (binary "*"
                    (unary "-" (id x))
                    (id y)
                )
            )
        )
        "#,
    )
}

#[test]
fn test_unary_not_with_variable_declaration() {
    execute(
        "let isNotValid = !isValid;",
        r#"
        (program
            (let
                (init
                    (id isNotValid)
                    (unary "!" (id isValid))
                )
            )
        )
        "#,
    )
}

#[test]
fn test_complex_expression_with_unary_operators() {
    execute(
        "result = -x * y + !z && w;",
        r#"
        (program
            (expr
                (assign "="
                    (id result)
                    (logical "&&"
                        (binary "+"
                            (binary "*"
                                (unary "-" (id x))
                                (id y)
                            )
                            (unary "!" (id z))
                        )
                        (id w)
                    )
                )
            )
        )
        "#,
    )
}
