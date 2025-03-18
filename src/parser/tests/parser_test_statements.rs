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
        r#"
        (program
            (block
                (expr (string "Hello"))
                (expr (number 17))))"#,
    )
}

#[test]
fn test_empty_block_statement() {
    execute(
        r#"
        {
        }
        "#,
        r#"
        (program
            (block))
        "#,
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
        r#"
        (program
            (block
                (expr (string "Hello"))
                (block
                    (expr (number 17)))))
        "#,
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
        r#"
        (program
            (block (empty)))"#,
    )
}

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
fn test_greater_than_operator() {
    execute(
        "x > 5;",
        r#"
        (program
            (expr (binary ">" (id x) (number 5))))
        "#,
    )
}

#[test]
fn test_greater_than_or_equal_to_operator() {
    execute(
        "x >= 10;",
        r#"
        (program
            (expr (binary ">=" (id x) (number 10))))
        "#,
    )
}

#[test]
fn test_less_than_operator() {
    execute(
        "y < 20;",
        r#"
        (program
            (expr (binary "<" (id y) (number 20))))
        "#,
    )
}

#[test]
fn test_less_than_or_equal_to_operator() {
    execute(
        "y <= 30;",
        r#"
        (program
            (expr (binary "<=" (id y) (number 30))))
        "#,
    )
}

#[test]
fn test_relational_operators_with_variables() {
    execute(
        "x > y;",
        r#"
        (program
            (expr (binary ">" (id x) (id y))))
        "#,
    )
}

#[test]
fn test_relational_operators_with_expressions() {
    execute(
        "x + 5 > y * 2;",
        r#"
        (program
            (expr (binary ">" (binary "+" (id x) (number 5)) (binary "*" (id y) (number 2)))))
        "#,
    )
}

#[test]
fn test_chained_relational_expressions() {
    execute(
        "x > 5 > y;", // Note: This is parsed as (x > 5) > y, which might not be what users expect
        r#"
        (program
            (expr (binary ">" (binary ">" (id x) (number 5)) (id y))))
        "#,
    )
}

#[test]
fn test_relational_operators_in_conditions() {
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
fn test_relational_operators_with_parenthesized_expressions() {
    execute(
        "(x + 10) > (y - 5);",
        r#"
        (program
            (expr (binary ">" (binary "+" (id x) (number 10)) (binary "-" (id y) (number 5)))))
        "#,
    )
}

#[test]
fn test_relational_operators_precedence_with_assignment() {
    execute(
        "result = x > y;",
        r#"
        (program
            (expr (assign "=" (id result) (binary ">" (id x) (id y)))))
        "#,
    )
}
