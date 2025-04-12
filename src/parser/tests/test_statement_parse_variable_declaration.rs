use super::utils::execute;

#[test]
fn test_simple_variable_declaration() {
    execute(
        "let x = 5;",
        r#"
        (program
            (let
                (init
                    (id x)
                    (number 5))))
        "#,
    )
}

#[test]
fn test_variable_declaration_without_initializer() {
    execute(
        "let x;",
        r#"
        (program
            (let
                (init
                    (id x))))
        "#,
    )
}

#[test]
fn test_multiple_variable_declarations() {
    execute(
        "let x = 5, y = 10;",
        r#"
        (program
            (let
                (init
                    (id x)
                    (number 5))
                (init
                    (id y)
                    (number 10))))
        "#,
    )
}

#[test]
fn test_mixed_variable_declarations() {
    execute(
        "let x = 5, y;",
        r#"
        (program
            (let
                (init
                    (id x)
                    (number 5))
                (init
                    (id y))))
        "#,
    )
}

#[test]
fn test_variable_initialization_with_expression() {
    execute(
        "let x = 2 + 3;",
        r#"
        (program
            (let
                (init
                    (id x)
                    (binary "+" (number 2) (number 3)))))
        "#,
    )
}

#[test]
fn test_variable_initialization_with_complex_expression() {
    execute(
        "let x = (2 + 3) * 4;",
        r#"
        (program
            (let
                (init
                    (id x)
                    (binary "*" (binary "+" (number 2) (number 3)) (number 4)))))
        "#,
    )
}

#[test]
fn test_variable_initialization_with_string() {
    execute(
        r#"let message = "Hello, world!";"#,
        r#"
        (program
            (let
                (init
                    (id message)
                    (string "Hello, world!"))))
        "#,
    )
}

#[test]
fn test_variable_declarations_with_multiple_types() {
    execute(
        r#"let count = 42, name = "John";"#,
        r#"
        (program
            (let
                (init
                    (id count)
                    (number 42))
                (init
                    (id name)
                    (string "John"))))
        "#,
    )
}

#[test]
fn test_multiple_variable_declarations_with_complex_expressions() {
    execute(
        "let x = 2 + 3, y = x * 4;",
        r#"
        (program
            (let
                (init
                    (id x)
                    (binary "+" (number 2) (number 3)))
                (init
                    (id y)
                    (binary "*" (id x) (number 4)))))
        "#,
    )
}
