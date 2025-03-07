use super::parser_tests::execute;

#[test]
fn test_block_statement() {
    execute(
        r#"
        {
            "Hello";
            17;
        }
        "#,
        "(program (block (expr (string \"Hello\")) (expr (number 17))))",
    )
}

#[test]
fn test_empty_block_statement() {
    execute(
        r#"
        {
        }
        "#,
        "(program (block ))",
    )
}

#[test]
fn test_nested_block_statement() {
    execute(
        r#"
        {
            "Hello";
            {
                17;
            }
        }
        "#,
        "(program (block (expr (string \"Hello\")) (block (expr (number 17)))))",
    )
}

#[test]
fn test_empty_statement() {
    execute(
        r#"
        {
            ;
        }
        "#,
        "(program (block (empty)))",
    )
}
