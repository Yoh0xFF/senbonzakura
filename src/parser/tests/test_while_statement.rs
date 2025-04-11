use super::parser_tests::execute;

#[test]
fn test_simple_while_loop() {
    execute(
        r#"
        while (i < 10) {
            i = i + 1;
        }
        "#,
        r#"
        (program
            (while
                (binary "<" (id i) (number 10))
                (block
                    (expr (assign "=" (id i) (binary "+" (id i) (number 1)))))))
        "#,
    )
}

#[test]
fn test_while_loop_with_boolean_condition() {
    execute(
        r#"
        while (true) {
            x = x + 1;
        }
        "#,
        r#"
        (program
            (while
                (boolean true)
                (block
                    (expr (assign "=" (id x) (binary "+" (id x) (number 1)))))))
        "#,
    )
}

#[test]
fn test_while_loop_with_complex_condition() {
    execute(
        r#"
        while (x > 5 && y < 20) {
            x = x - 1;
            y = y + 1;
        }
        "#,
        r#"
        (program
            (while
                (logical "&&" (binary ">" (id x) (number 5)) (binary "<" (id y) (number 20)))
                (block
                    (expr (assign "=" (id x) (binary "-" (id x) (number 1))))
                    (expr (assign "=" (id y) (binary "+" (id y) (number 1)))))))
        "#,
    )
}

#[test]
fn test_while_loop_with_empty_body() {
    execute(
        r#"
        while (condition) {
        }
        "#,
        r#"
        (program
            (while
                (id condition)
                (block)))
        "#,
    )
}

#[test]
fn test_nested_while_loops() {
    execute(
        r#"
        while (outer > 0) {
            outer = outer - 1;

            while (inner < 3) {
                inner = inner + 1;
            }
        }
        "#,
        r#"
        (program
            (while
                (binary ">" (id outer) (number 0))
                (block
                    (expr (assign "=" (id outer) (binary "-" (id outer) (number 1))))
                    (while
                        (binary "<" (id inner) (number 3))
                        (block
                            (expr (assign "=" (id inner) (binary "+" (id inner) (number 1)))))))))
        "#,
    )
}

#[test]
fn test_while_loop_with_unary_condition() {
    execute(
        r#"
        while (!done) {
            x = x + 1;
        }
        "#,
        r#"
        (program
            (while
                (unary "!" (id done))
                (block
                    (expr (assign "=" (id x) (binary "+" (id x) (number 1)))))))
        "#,
    )
}
