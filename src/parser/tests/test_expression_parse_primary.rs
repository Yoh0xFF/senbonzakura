use super::internal_util::execute;

#[test]
fn test_identifier_expression() {
    execute(
        "variableName;",
        r#"
        (program
            (expr (id variableName)))
        "#,
    )
}

#[test]
fn test_multiple_identifiers() {
    execute(
        r#"
        first;
        second;
        "#,
        r#"
        (program
            (expr (id first))
            (expr (id second)))
        "#,
    )
}

#[test]
fn test_identifier_starting_with_keyword() {
    execute(
        "letVariable;",
        r#"
        (program
            (expr (id letVariable)))
        "#,
    )
}

#[test]
fn test_identifier_with_numbers() {
    execute(
        "user123;",
        r#"
        (program
            (expr (id user123)))
        "#,
    )
}

#[test]
fn test_identifier_with_underscore() {
    execute(
        "user_name;",
        r#"
        (program
            (expr (id user_name)))
        "#,
    )
}

#[test]
fn test_parenthesized_expression() {
    execute(
        "(42);",
        r#"
        (program
            (expr (number 42)))
        "#,
    )
}

#[test]
fn test_nested_parenthesized_expression() {
    execute(
        "((42));",
        r#"
        (program
            (expr (number 42)))
        "#,
    )
}

#[test]
fn test_parenthesized_identifier() {
    execute(
        "(x);",
        r#"
        (program
            (expr (id x)))
        "#,
    )
}

#[test]
fn test_parenthesized_binary_expression() {
    execute(
        "(x + y);",
        r#"
        (program
            (expr (binary "+" (id x) (id y))))
        "#,
    )
}

#[test]
fn test_parenthesized_complex_expression() {
    execute(
        "(a * b + c / d);",
        r#"
        (program
            (expr (binary "+" (binary "*" (id a) (id b)) (binary "/" (id c) (id d)))))
        "#,
    )
}

#[test]
fn test_this_expression() {
    execute(
        "this;",
        r#"
        (program
            (expr (this)))
        "#,
    )
}

#[test]
fn test_this_with_member_access() {
    execute(
        "this.property;",
        r#"
        (program
            (expr (member "static" (this) (id property))))
        "#,
    )
}

#[test]
fn test_this_with_computed_member_access() {
    execute(
        "this[property];",
        r#"
        (program
            (expr (member "computed" (this) (id property))))
        "#,
    )
}

#[test]
fn test_this_with_method_call() {
    execute(
        "this.method();",
        r#"
        (program
            (expr (call (member "static" (this) (id method)))))
        "#,
    )
}

#[test]
fn test_super_expression() {
    execute(
        "super;",
        r#"
        (program
            (expr (super)))
        "#,
    )
}

#[test]
fn test_super_with_member_access() {
    execute(
        "super.property;",
        r#"
        (program
            (expr (member "static" (super) (id property))))
        "#,
    )
}

#[test]
fn test_super_with_method_call() {
    execute(
        "super.method();",
        r#"
        (program
            (expr (call (member "static" (super) (id method)))))
        "#,
    )
}

#[test]
fn test_super_with_arguments() {
    execute(
        "super(arg1, arg2);",
        r#"
        (program
            (expr (call (super) (args (id arg1) (id arg2)))))
        "#,
    )
}

#[test]
fn test_new_expression_no_arguments() {
    execute(
        "new Object();",
        r#"
        (program
            (expr (new (id Object))))
        "#,
    )
}

#[test]
fn test_new_expression_with_arguments() {
    execute(
        "new Point(10, 20);",
        r#"
        (program
            (expr (new (id Point) (args (number 10) (number 20)))))
        "#,
    )
}

#[test]
fn test_new_expression_with_member_access() {
    execute(
        "new namespace.Class();",
        r#"
        (program
            (expr (new (member "static" (id namespace) (id Class)))))
        "#,
    )
}

#[test]
fn test_new_expression_with_computed_member_access() {
    execute(
        "new objects[member]();",
        r#"
        (program
            (expr (new (member "computed" (id objects) (id member)))))
        "#,
    )
}

#[test]
fn test_new_expression_with_complex_arguments() {
    execute(
        "new Constructor(x + y, obj.method(), 42);",
        r#"
        (program
            (expr (new (id Constructor) (args (binary "+" (id x) (id y)) (call (member "static" (id obj) (id method))) (number 42)))))
        "#,
    )
}

#[test]
fn test_primary_expression_in_binary_context() {
    execute(
        "42 + (x * y);",
        r#"
        (program
            (expr (binary "+" (number 42) (binary "*" (id x) (id y)))))
        "#,
    )
}

#[test]
fn test_primary_expression_in_logical_context() {
    execute(
        "(a > b) && this.isValid();",
        r#"
        (program
            (expr (logical "&&" (binary ">" (id a) (id b)) (call (member "static" (this) (id isValid))))))
        "#,
    )
}

#[test]
fn test_primary_expression_in_assignment() {
    execute(
        "result = (a + b) * c;",
        r#"
        (program
            (expr (assign "=" (id result) (binary "*" (binary "+" (id a) (id b)) (id c)))))
        "#,
    )
}

#[test]
fn test_primary_expression_in_initialization() {
    execute(
        "let value: Vector = new Vector(1, 2, 3);",
        r#"
        (program
            (let
                (init
                    (id value)
                    (type(class Vector))
                    (new (id Vector) (args (number 1) (number 2) (number 3))))))
        "#,
    )
}

#[test]
fn test_primary_expression_in_conditional() {
    execute(
        r#"
        if (this.isAdmin()) {
            return true;
        }
        "#,
        r#"
        (program
            (if
                (call (member "static" (this) (id isAdmin)))
                (block
                    (return (boolean true)))))
        "#,
    )
}

#[test]
fn test_primary_expression_in_loop() {
    execute(
        r#"
        while (this.hasNext()) {
            this.process();
        }
        "#,
        r#"
        (program
            (while
                (call (member "static" (this) (id hasNext)))
                (block
                    (expr (call (member "static" (this) (id process)))))))
        "#,
    )
}

#[test]
fn test_mixed_primary_expressions() {
    execute(
        r#"
        let result: number = (this.calculate(x, y) + super.getValue()) * new Factor(42).apply();
        "#,
        r#"
        (program
            (let
                (init
                    (id result)
                    (type Number)
                    (binary "*"
                        (binary "+"
                            (call (member "static" (this) (id calculate)) (args (id x) (id y)))
                            (call (member "static" (super) (id getValue)))
                        )
                        (call (member "static" (new (id Factor) (args (number 42))) (id apply)))
                    )
                )
            ))
        "#,
    )
}
