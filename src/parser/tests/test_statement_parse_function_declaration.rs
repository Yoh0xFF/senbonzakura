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
                (return_type "void")
                (block
                    (expr (assign "=" (id x) (number 10))))))
        "#,
    )
}

#[test]
fn test_function_declaration_with_single_parameter() {
    execute(
        r#"
        def add(x: number): number {
            return x + 1;
        }
        "#,
        r#"
        (program
            (def
                (id add)
                (params
                    (param (id x) (type Number)))
                (return_type Number)
                (block
                    (return (binary "+" (id x) (number 1))))))
        "#,
    )
}

#[test]
fn test_function_declaration_with_multiple_parameters() {
    execute(
        r#"
        def add(x: number, y: number, z: number): number {
            return x + y + z;
        }
        "#,
        r#"
        (program
            (def
                (id add)
                (params
                    (param (id x) (type Number))
                    (param (id y) (type Number))
                    (param (id z) (type Number)))
                (return_type Number)
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
                (return_type "void")
                (block)))
        "#,
    )
}

#[test]
fn test_function_declaration_with_variable_declaration() {
    execute(
        r#"
        def initFunction(): number {
            let x: number = 10;
            let y: number = 20;
            return x + y;
        }
        "#,
        r#"
        (program
            (def
                (id initFunction)
                (return_type Number)
                (block
                    (let
                        (init
                            (id x)
                            (type Number)
                            (number 10)))
                    (let
                        (init
                            (id y)
                            (type Number)
                            (number 20)))
                    (return (binary "+" (id x) (id y))))))
        "#,
    )
}

#[test]
fn test_function_declaration_with_if_statement() {
    execute(
        r#"
        def max(a: number, b: number): number {
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
                    (param (id a) (type Number))
                    (param (id b) (type Number)))
                (return_type Number)
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
        def factorial(n: number): number {
            let result: number = 1;
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
                    (param (id n) (type Number)))
                (return_type Number)
                (block
                    (let
                        (init
                            (id result)
                            (type Number)
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
        def earlyReturn(x: number): number {
            if (x <= 0) {
                return 0;
            }
            return x;
        }
        "#,
        r#"
        (program
            (def
                (id earlyReturn)
                (params
                    (param (id x) (type Number)))
                (return_type Number)
                (block
                    (if
                        (binary "<=" (id x) (number 0))
                        (block
                            (return (number 0))))
                    (return (id x)))))
        "#,
    )
}

#[test]
fn test_function_declaration_with_complex_expression() {
    execute(
        r#"
        def evaluate(a: number, b: number, c: number): number {
            return a * b + c * (a + b);
        }
        "#,
        r#"
        (program
            (def
                (id evaluate)
                (params
                    (param (id a) (type Number))
                    (param (id b) (type Number))
                    (param (id c) (type Number)))
                (return_type Number)
                (block
                    (return (binary "+" (binary "*" (id a) (id b)) (binary "*" (id c) (binary "+" (id a) (id b))))))))
        "#,
    )
}

#[test]
fn test_multiple_function_declarations() {
    execute(
        r#"
        def add(a: number, b: number): number {
            return a + b;
        }

        def subtract(a: number, b: number): number {
            return a - b;
        }
        "#,
        r#"
        (program
            (def
                (id add)
                (params
                    (param (id a) (type Number))
                    (param (id b) (type Number)))
                (return_type Number)
                (block
                    (return (binary "+" (id a) (id b)))))
            (def
                (id subtract)
                (params
                    (param (id a) (type Number))
                    (param (id b) (type Number)))
                (return_type Number)
                (block
                    (return (binary "-" (id a) (id b))))))
        "#,
    )
}
