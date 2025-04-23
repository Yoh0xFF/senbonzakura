use super::utils::execute;

#[test]
fn test_simple_function_declaration() {
    execute(
        r#"
        def myFunction() {
            x = 10;
        }
        "#,
        r#"
        (program
            (def
                (id myFunction)
                (block
                    (expr (assign "=" (id x) (number 10))))))
        "#,
    )
}

#[test]
fn test_function_declaration_with_single_parameter() {
    execute(
        r#"
        def add(x) {
            return x + 1;
        }
        "#,
        r#"
        (program
            (def
                (id add)
                (params
                    (id x))
                (block
                    (return (binary "+" (id x) (number 1))))))
        "#,
    )
}

#[test]
fn test_function_declaration_with_multiple_parameters() {
    execute(
        r#"
        def add(x, y, z) {
            return x + y + z;
        }
        "#,
        r#"
        (program
            (def
                (id add)
                (params
                    (id x)
                    (id y)
                    (id z))
                (block
                    (return (binary "+" (binary "+" (id x) (id y)) (id z))))))
        "#,
    )
}

#[test]
fn test_function_declaration_with_empty_body() {
    execute(
        r#"
        def emptyFunction() {
        }
        "#,
        r#"
        (program
            (def
                (id emptyFunction)
                (block)))
        "#,
    )
}

#[test]
fn test_function_declaration_with_variable_declaration() {
    execute(
        r#"
        def initFunction() {
            let x = 10;
            let y = 20;
            return x + y;
        }
        "#,
        r#"
        (program
            (def
                (id initFunction)
                (block
                    (let
                        (init
                            (id x)
                            (number 10)))
                    (let
                        (init
                            (id y)
                            (number 20)))
                    (return (binary "+" (id x) (id y))))))
        "#,
    )
}

#[test]
fn test_function_declaration_with_if_statement() {
    execute(
        r#"
        def max(a, b) {
            if (a > b) {
                return a;
            } else {
                return b;
            }
        }
        "#,
        r#"
        (program
            (def
                (id max)
                (params
                    (id a)
                    (id b))
                (block
                    (if
                        (binary ">" (id a) (id b))
                        (block
                            (return (id a)))
                        (block
                            (return (id b)))))))
        "#,
    )
}

#[test]
fn test_function_declaration_with_while_loop() {
    execute(
        r#"
        def factorial(n) {
            let result = 1;
            while (n > 1) {
                result = result * n;
                n = n - 1;
            }
            return result;
        }
        "#,
        r#"
        (program
            (def
                (id factorial)
                (params
                    (id n))
                (block
                    (let
                        (init
                            (id result)
                            (number 1)))
                    (while
                        (binary ">" (id n) (number 1))
                        (block
                            (expr (assign "=" (id result) (binary "*" (id result) (id n))))
                            (expr (assign "=" (id n) (binary "-" (id n) (number 1))))))
                    (return (id result)))))
        "#,
    )
}

#[test]
fn test_function_declaration_with_return_no_argument() {
    execute(
        r#"
        def earlyReturn(x) {
            if (x <= 0) {
                return;
            }
            return x;
        }
        "#,
        r#"
        (program
            (def
                (id earlyReturn)
                (params
                    (id x))
                (block
                    (if
                        (binary "<=" (id x) (number 0))
                        (block
                            (return)))
                    (return (id x)))))
        "#,
    )
}

#[test]
fn test_function_declaration_with_complex_expression() {
    execute(
        r#"
        def evaluate(a, b, c) {
            return a * b + c * (a + b);
        }
        "#,
        r#"
        (program
            (def
                (id evaluate)
                (params
                    (id a)
                    (id b)
                    (id c))
                (block
                    (return (binary "+" (binary "*" (id a) (id b)) (binary "*" (id c) (binary "+" (id a) (id b))))))))
        "#,
    )
}

#[test]
fn test_multiple_function_declarations() {
    execute(
        r#"
        def add(a, b) {
            return a + b;
        }

        def subtract(a, b) {
            return a - b;
        }
        "#,
        r#"
        (program
            (def
                (id add)
                (params
                    (id a)
                    (id b))
                (block
                    (return (binary "+" (id a) (id b)))))
            (def
                (id subtract)
                (params
                    (id a)
                    (id b))
                (block
                    (return (binary "-" (id a) (id b))))))
        "#,
    )
}
