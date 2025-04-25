use super::utils::execute;

#[test]
fn test_simple_variable_declaration() {
    execute(
        "let x: number = 5;",
        r#"
        (program
            (let
                (init
                    (id x)
                    (type Number)
                    (number 5))))
        "#,
    )
}

#[test]
fn test_variable_declaration_without_initializer() {
    execute(
        "let x: string;",
        r#"
        (program
            (let
                (init
                    (id x)
                    (type String))))
        "#,
    )
}

#[test]
fn test_multiple_variable_declarations() {
    execute(
        "let x: number = 5, y: number = 10;",
        r#"
        (program
            (let
                (init
                    (id x)
                    (type Number)
                    (number 5))
                (init
                    (id y)
                    (type Number)
                    (number 10))))
        "#,
    )
}

#[test]
fn test_mixed_variable_declarations() {
    execute(
        "let x: number = 5, y: number;",
        r#"
        (program
            (let
                (init
                    (id x)
                    (type Number)
                    (number 5))
                (init
                    (id y)
                    (type Number))))
        "#,
    )
}

#[test]
fn test_variable_initialization_with_expression() {
    execute(
        "let x: number = 2 + 3;",
        r#"
        (program
            (let
                (init
                    (id x)
                    (type Number)
                    (binary "+" (number 2) (number 3)))))
        "#,
    )
}

#[test]
fn test_variable_initialization_with_complex_expression() {
    execute(
        "let x: number = (2 + 3) * 4;",
        r#"
        (program
            (let
                (init
                    (id x)
                    (type Number)
                    (binary "*" (binary "+" (number 2) (number 3)) (number 4)))))
        "#,
    )
}

#[test]
fn test_variable_initialization_with_string() {
    execute(
        r#"let message: string = "Hello, world!";"#,
        r#"
        (program
            (let
                (init
                    (id message)
                    (type String)
                    (string "Hello, world!"))))
        "#,
    )
}

#[test]
fn test_variable_declarations_with_multiple_types() {
    execute(
        r#"let count: number = 42, name: string = "John";"#,
        r#"
        (program
            (let
                (init
                    (id count)
                    (type Number)
                    (number 42))
                (init
                    (id name)
                    (type String)
                    (string "John"))))
        "#,
    )
}

#[test]
fn test_multiple_variable_declarations_with_complex_expressions() {
    execute(
        "let x: number = 2 + 3, y: number = x * 4;",
        r#"
        (program
            (let
                (init
                    (id x)
                    (type Number)
                    (binary "+" (number 2) (number 3)))
                (init
                    (id y)
                    (type Number)
                    (binary "*" (id x) (number 4)))))
        "#,
    )
}
