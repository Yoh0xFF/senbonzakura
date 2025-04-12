use super::utils::execute;

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
fn test_double_quote_string_literal() {
    execute(
        r#""Hello";"#,
        r#"
        (program
            (expr (string "Hello")))
        "#,
    )
}

#[test]
fn test_single_quote_string_literal() {
    execute(
        r#"'Hello';"#,
        r#"
        (program
            (expr (string "Hello")))
        "#,
    )
}

#[test]
fn test_multiple_string_literals() {
    execute(
        r#"  'Hello';    "World";"#,
        r#"
        (program
            (expr (string "Hello"))
            (expr (string "World")))
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
