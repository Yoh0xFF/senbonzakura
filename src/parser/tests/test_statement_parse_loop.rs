use super::internal_util::execute;

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

#[test]
fn test_simple_for_loop() {
    execute(
        r#"
        for (let i: number = 0; i < 10; i = i + 1) {
            sum = sum + i;
        }
        "#,
        r#"
        (program
            (for
                (let
                    (init
                        (id i)
                        (type Number)
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
        for (let i: number = 0, j: number = 10; i < j; i = i + 1) {
            sum = sum + i;
        }
        "#,
        r#"
        (program
            (for
                (let
                    (init
                        (id i)
                        (type Number)
                        (number 0))
                    (init
                        (id j)
                        (type Number)
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
        for (let i: number = 0; ; i = i + 1) {
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
                        (type Number)
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
        for (let i: number = 0; i < 10;) {
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
                        (type Number)
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
        for (let i: number = 0; i < 3; i = i + 1) {
            for (let j: number = 0; j < 3; j = j + 1) {
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
                        (type Number)
                        (number 0)))
                (binary "<" (id i) (number 3))
                (assign "=" (id i) (binary "+" (id i) (number 1)))
                (block
                    (for
                        (let
                            (init
                                (id j)
                                (type Number)
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
        for (let i: number = a * b + c; i < max && !done; i = i + 1) {
            sum = sum + i;
        }
        "#,
        r#"
        (program
            (for
                (let
                    (init
                        (id i)
                        (type Number)
                        (binary "+" (binary "*" (id a) (id b)) (id c))))
                (logical "&&" (binary "<" (id i) (id max)) (unary "!" (id done)))
                (assign "=" (id i) (binary "+" (id i) (number 1)))
                (block
                    (expr (assign "=" (id sum) (binary "+" (id sum) (id i)))))))
        "#,
    )
}
