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
