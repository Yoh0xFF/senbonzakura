use super::parser_tests::execute;

#[test]
fn test_simple_if_statement() {
    execute(
        r#"
        if (x > 5) {
            y = 10;
        }
        "#,
        r#"
        (program
            (if
                (binary ">" (id x) (number 5))
                (block
                    (expr (assign "=" (id y) (number 10))))))
        "#,
    )
}

#[test]
fn test_if_else_statement() {
    execute(
        r#"
        if (x > 5) {
            y = 10;
        } else {
            y = 0;
        }
        "#,
        r#"
        (program
            (if
                (binary ">" (id x) (number 5))
                (block
                    (expr (assign "=" (id y) (number 10))))
                (block
                    (expr (assign "=" (id y) (number 0))))))
        "#,
    )
}

#[test]
fn test_relational_operators_in_if_conditions() {
    execute(
        r#"
        if (x >= 10) {
            y = 20;
        }
        "#,
        r#"
        (program
            (if
                (binary ">=" (id x) (number 10))
                (block
                    (expr (assign "=" (id y) (number 20))))))
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
