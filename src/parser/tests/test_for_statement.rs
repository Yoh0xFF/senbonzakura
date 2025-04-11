use super::parser_tests::execute;

#[test]
fn test_simple_for_loop() {
    execute(
        r#"
        for (let i = 0; i < 10; i = i + 1) {
            sum = sum + i;
        }
        "#,
        r#"
        (program
            (for
                (let
                    (init
                        (id i)
                        (number 0)))
                (binary "<" (id i) (number 10))
                (assign "=" (id i) (binary "+" (id i) (number 1)))
                (block
                    (expr (assign "=" (id sum) (binary "+" (id sum) (id i)))))))
        "#,
    )
}

#[test]
fn test_for_loop_with_expression_initializer() {
    execute(
        r#"
        for (i = 0; i < 10; i = i + 1) {
            sum = sum + i;
        }
        "#,
        r#"
        (program
            (for
                (expr (assign "=" (id i) (number 0)))
                (binary "<" (id i) (number 10))
                (assign "=" (id i) (binary "+" (id i) (number 1)))
                (block
                    (expr (assign "=" (id sum) (binary "+" (id sum) (id i)))))))
        "#,
    )
}

#[test]
fn test_for_loop_with_multiple_initializations() {
    execute(
        r#"
        for (let i = 0, j = 10; i < j; i = i + 1) {
            sum = sum + i;
        }
        "#,
        r#"
        (program
            (for
                (let
                    (init
                        (id i)
                        (number 0))
                    (init
                        (id j)
                        (number 10)))
                (binary "<" (id i) (id j))
                (assign "=" (id i) (binary "+" (id i) (number 1)))
                (block
                    (expr (assign "=" (id sum) (binary "+" (id sum) (id i)))))))
        "#,
    )
}

#[test]
fn test_for_loop_without_initializer() {
    execute(
        r#"
        for (; i < 10; i = i + 1) {
            sum = sum + i;
        }
        "#,
        r#"
        (program
            (for
                (binary "<" (id i) (number 10))
                (assign "=" (id i) (binary "+" (id i) (number 1)))
                (block
                    (expr (assign "=" (id sum) (binary "+" (id sum) (id i)))))))
        "#,
    )
}

#[test]
fn test_for_loop_without_condition() {
    execute(
        r#"
        for (let i = 0; ; i = i + 1) {
            if (i >= 10) {
                i = 0;
            }
            sum = sum + i;
        }
        "#,
        r#"
        (program
            (for
                (let
                    (init
                        (id i)
                        (number 0)))
                (assign "=" (id i) (binary "+" (id i) (number 1)))
                (block
                    (if
                        (binary ">=" (id i) (number 10))
                        (block
                            (expr (assign "=" (id i) (number 0)))))
                    (expr (assign "=" (id sum) (binary "+" (id sum) (id i)))))))
        "#,
    )
}

#[test]
fn test_for_loop_without_increment() {
    execute(
        r#"
        for (let i = 0; i < 10;) {
            sum = sum + i;
            i = i + 1;
        }
        "#,
        r#"
        (program
            (for
                (let
                    (init
                        (id i)
                        (number 0)))
                (binary "<" (id i) (number 10))
                (block
                    (expr (assign "=" (id sum) (binary "+" (id sum) (id i))))
                    (expr (assign "=" (id i) (binary "+" (id i) (number 1)))))))
        "#,
    )
}

#[test]
fn test_empty_for_loop() {
    execute(
        r#"
        for (;;) {
            if (condition) {
                done = true;
            }
        }
        "#,
        r#"
        (program
            (for
                (block
                    (if
                        (id condition)
                        (block
                            (expr (assign "=" (id done) (boolean true))))))))
        "#,
    )
}

#[test]
fn test_nested_for_loops() {
    execute(
        r#"
        for (let i = 0; i < 3; i = i + 1) {
            for (let j = 0; j < 3; j = j + 1) {
                result = i * j;
            }
        }
        "#,
        r#"
        (program
            (for
                (let
                    (init
                        (id i)
                        (number 0)))
                (binary "<" (id i) (number 3))
                (assign "=" (id i) (binary "+" (id i) (number 1)))
                (block
                    (for
                        (let
                            (init
                                (id j)
                                (number 0)))
                        (binary "<" (id j) (number 3))
                        (assign "=" (id j) (binary "+" (id j) (number 1)))
                        (block
                            (expr (assign "=" (id result) (binary "*" (id i) (id j)))))))))
        "#,
    )
}

#[test]
fn test_for_loop_with_complex_expressions() {
    execute(
        r#"
        for (let i = a * b + c; i < max && !done; i = i + 1) {
            sum = sum + i;
        }
        "#,
        r#"
        (program
            (for
                (let
                    (init
                        (id i)
                        (binary "+" (binary "*" (id a) (id b)) (id c))))
                (logical "&&" (binary "<" (id i) (id max)) (unary "!" (id done)))
                (assign "=" (id i) (binary "+" (id i) (number 1)))
                (block
                    (expr (assign "=" (id sum) (binary "+" (id sum) (id i)))))))
        "#,
    )
}
