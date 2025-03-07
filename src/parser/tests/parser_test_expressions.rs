use super::parser_tests::execute;

#[test]
fn test_string_literal() {
    execute(r#""Hello";"#, "(program (expr (string \"Hello\")))")
}

#[test]
fn test_number_literal() {
    execute("12;", "(program (expr (number 12)))")
}

#[test]
fn test_multiple_literal() {
    execute(
        r#"
        "Hello";

        17;
        "#,
        "(program (expr (string \"Hello\")) (expr (number 17)))",
    )
}
