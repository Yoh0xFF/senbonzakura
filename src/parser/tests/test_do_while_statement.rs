use super::parser_tests::execute;

#[test]
fn test_simple_do_while_loop() {
    execute(
        r#"
        do {
            i = i + 1;
        } while (i < 10);
        "#,
        r#"
        (program
            (do-while
                (block
                    (expr (assign "=" (id i) (binary "+" (id i) (number 1)))))
                (binary "<" (id i) (number 10))))
        "#,
    )
}

#[test]
fn test_do_while_loop_with_boolean_condition() {
    execute(
        r#"
        do {
            x = x + 1;
        } while (true);
        "#,
        r#"
        (program
            (do-while
                (block
                    (expr (assign "=" (id x) (binary "+" (id x) (number 1)))))
                (boolean true)))
        "#,
    )
}

#[test]
fn test_do_while_loop_with_complex_condition() {
    execute(
        r#"
        do {
            x = x - 1;
            y = y + 1;
        } while (x > 5 && y < 20);
        "#,
        r#"
        (program
            (do-while
                (block
                    (expr (assign "=" (id x) (binary "-" (id x) (number 1))))
                    (expr (assign "=" (id y) (binary "+" (id y) (number 1)))))
                (logical "&&" (binary ">" (id x) (number 5)) (binary "<" (id y) (number 20)))))
        "#,
    )
}

#[test]
fn test_do_while_loop_with_empty_body() {
    execute(
        r#"
        do {
        } while (condition);
        "#,
        r#"
        (program
            (do-while
                (block)
                (id condition)))
        "#,
    )
}

#[test]
fn test_nested_do_while_loops() {
    execute(
        r#"
        do {
            outer = outer - 1;

            do {
                inner = inner + 1;
            } while (inner < 3);

        } while (outer > 0);
        "#,
        r#"
        (program
            (do-while
                (block
                    (expr (assign "=" (id outer) (binary "-" (id outer) (number 1))))
                    (do-while
                        (block
                            (expr (assign "=" (id inner) (binary "+" (id inner) (number 1)))))
                        (binary "<" (id inner) (number 3))))
                (binary ">" (id outer) (number 0))))
        "#,
    )
}

#[test]
fn test_do_while_loop_with_unary_condition() {
    execute(
        r#"
        do {
            x = x + 1;
        } while (!done);
        "#,
        r#"
        (program
            (do-while
                (block
                    (expr (assign "=" (id x) (binary "+" (id x) (number 1)))))
                (unary "!" (id done))))
        "#,
    )
}
