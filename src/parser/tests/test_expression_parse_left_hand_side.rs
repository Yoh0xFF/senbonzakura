use super::utils::execute;

///
/// Test member expression
///

#[test]
fn test_simple_member_access() {
    execute(
        "obj.property;",
        r#"
        (program
            (expr (member "static" (id obj) (id property))))
        "#,
    )
}

#[test]
fn test_computed_member_access() {
    execute(
        "obj[property];",
        r#"
        (program
            (expr (member "computed" (id obj) (id property))))
        "#,
    )
}

#[test]
fn test_computed_member_access_with_literal() {
    execute(
        "obj[42];",
        r#"
        (program
            (expr (member "computed" (id obj) (number 42))))
        "#,
    )
}

#[test]
fn test_computed_member_access_with_string() {
    execute(
        r#"obj["property"];"#,
        r#"
        (program
            (expr (member "computed" (id obj) (string "property"))))
        "#,
    )
}

#[test]
fn test_computed_member_access_with_expression() {
    execute(
        "obj[x + y];",
        r#"
        (program
            (expr (member "computed" (id obj) (binary "+" (id x) (id y)))))
        "#,
    )
}

#[test]
fn test_chained_member_access() {
    execute(
        "obj.inner.property;",
        r#"
        (program
            (expr (member "static" (member "static" (id obj) (id inner)) (id property))))
        "#,
    )
}

#[test]
fn test_chained_computed_member_access() {
    execute(
        "obj[x][y];",
        r#"
        (program
            (expr (member "computed" (member "computed" (id obj) (id x)) (id y))))
        "#,
    )
}

#[test]
fn test_mixed_member_access() {
    execute(
        "obj.inner[index].property;",
        r#"
        (program
            (expr (member "static" (member "computed" (member "static" (id obj) (id inner)) (id index)) (id property))))
        "#,
    )
}

#[test]
fn test_member_access_with_assignment() {
    execute(
        "obj.property = 42;",
        r#"
        (program
            (expr (assign "=" (member "static" (id obj) (id property)) (number 42))))
        "#,
    )
}

#[test]
fn test_computed_member_access_with_assignment() {
    execute(
        "obj[property] = 42;",
        r#"
        (program
            (expr (assign "=" (member "computed" (id obj) (id property)) (number 42))))
        "#,
    )
}

#[test]
fn test_member_access_with_complex_assignment() {
    execute(
        "obj.property += 42;",
        r#"
        (program
            (expr (assign "+=" (member "static" (id obj) (id property)) (number 42))))
        "#,
    )
}

#[test]
fn test_deep_nested_member_access() {
    execute(
        "obj.a.b.c.d.property;",
        r#"
        (program
            (expr (member "static" (member "static" (member "static" (member "static" (member "static" (id obj) (id a)) (id b)) (id c)) (id d)) (id property))))
        "#,
    )
}

#[test]
fn test_member_access_in_binary_expression() {
    execute(
        "obj.value + 10;",
        r#"
        (program
            (expr (binary "+" (member "static" (id obj) (id value)) (number 10))))
        "#,
    )
}

#[test]
fn test_member_access_in_logical_expression() {
    execute(
        "obj.valid && hasPermission;",
        r#"
        (program
            (expr (logical "&&" (member "static" (id obj) (id valid)) (id hasPermission))))
        "#,
    )
}

#[test]
fn test_variable_declaration_with_member_access() {
    execute(
        "let value = obj.property;",
        r#"
        (program
            (let
                (init
                    (id value)
                    (member "static" (id obj) (id property)))))
        "#,
    )
}

#[test]
fn test_conditional_with_member_access() {
    execute(
        r#"
        if (user.isAdmin) {
            permission.granted = true;
        }
        "#,
        r#"
        (program
            (if
                (member "static" (id user) (id isAdmin))
                (block
                    (expr (assign "=" (member "static" (id permission) (id granted)) (boolean true))))))
        "#,
    )
}

#[test]
fn test_conditional_with_computed_member_access() {
    execute(
        r#"
        if (permissions[role]) {
            access[level] = true;
        }
        "#,
        r#"
        (program
            (if
                (member "computed" (id permissions) (id role))
                (block
                    (expr (assign "=" (member "computed" (id access) (id level)) (boolean true))))))
        "#,
    )
}

#[test]
fn test_complex_computed_member_access() {
    execute(
        "matrix[i * 10 + j];",
        r#"
        (program
            (expr (member "computed" (id matrix) (binary "+" (binary "*" (id i) (number 10)) (id j)))))
        "#,
    )
}

#[test]
fn test_member_access_with_unary_operation() {
    execute(
        "!obj.enabled;",
        r#"
        (program
            (expr (unary "!" (member "static" (id obj) (id enabled)))))
        "#,
    )
}

///
/// Test call expression
///

#[test]
fn test_simple_call_expression() {
    execute(
        "foo();",
        r#"
        (program
            (expr (call (id foo))))
        "#,
    )
}

#[test]
fn test_call_with_single_argument() {
    execute(
        "foo(42);",
        r#"
        (program
            (expr (call (id foo) (args (number 42)))))
        "#,
    )
}

#[test]
fn test_call_with_multiple_arguments() {
    execute(
        "foo(x, y, 42);",
        r#"
        (program
            (expr (call (id foo) (args (id x) (id y) (number 42)))))
        "#,
    )
}

#[test]
fn test_call_with_expression_argument() {
    execute(
        "foo(x + y);",
        r#"
        (program
            (expr (call (id foo) (args (binary "+" (id x) (id y))))))
        "#,
    )
}

#[test]
fn test_chained_calls() {
    execute(
        "foo()();",
        r#"
        (program
            (expr (call (call (id foo)))))
        "#,
    )
}

#[test]
fn test_call_with_member_expression() {
    execute(
        "obj.method();",
        r#"
        (program
            (expr (call (member "static" (id obj) (id method)))))
        "#,
    )
}

#[test]
fn test_call_with_computed_member_expression() {
    execute(
        "obj['method']();",
        r#"
        (program
            (expr (call (member "computed" (id obj) (string "method")))))
        "#,
    )
}

// #[test]
// fn test_member_expression_on_call_result() {
//     execute(
//         "foo().property;",
//         r#"
//         (program
//             (expr (member "static" (call (id foo)) (id property))))
//         "#,
//     )
// }
//
// #[test]
// fn test_computed_member_expression_on_call_result() {
//     execute(
//         "foo()[index];",
//         r#"
//         (program
//             (expr (member "computed" (call (id foo)) (id index))))
//         "#,
//     )
// }

#[test]
fn test_complex_nested_call_expressions() {
    execute(
        "foo(bar(), baz(42));",
        r#"
        (program
            (expr (call (id foo) (args (call (id bar)) (call (id baz) (args (number 42)))))))
        "#,
    )
}

#[test]
fn test_call_in_binary_expression() {
    execute(
        "foo() + bar();",
        r#"
        (program
            (expr (binary "+" (call (id foo)) (call (id bar)))))
        "#,
    )
}

#[test]
fn test_call_in_logical_expression() {
    execute(
        "isValid() && hasPermission();",
        r#"
        (program
            (expr (logical "&&" (call (id isValid)) (call (id hasPermission)))))
        "#,
    )
}

#[test]
fn test_call_in_assignment() {
    execute(
        "result = getValue();",
        r#"
        (program
            (expr (assign "=" (id result) (call (id getValue)))))
        "#,
    )
}

#[test]
fn test_call_with_nested_expressions() {
    execute(
        "calculate(x + y, z * 2, obj.property);",
        r#"
        (program
            (expr (call (id calculate) (args (binary "+" (id x) (id y)) (binary "*" (id z) (number 2)) (member "static" (id obj) (id property))))))
        "#,
    )
}

#[test]
fn test_variable_declaration_with_call() {
    execute(
        "let result = getValue();",
        r#"
        (program
            (let
                (init
                    (id result)
                    (call (id getValue)))))
        "#,
    )
}

#[test]
fn test_if_with_call_condition() {
    execute(
        r#"
        if (isAdmin()) {
            grantAccess();
        }
        "#,
        r#"
        (program
            (if
                (call (id isAdmin))
                (block
                    (expr (call (id grantAccess))))))
        "#,
    )
}

// #[test]
// fn test_member_access_chain_with_call() {
//     execute(
//         "obj.getInner().property;",
//         r#"
//         (program
//             (expr (member "static" (call (member "static" (id obj) (id getInner))) (id property))))
//         "#,
//     )
// }
//
// #[test]
// fn test_complex_call_chain() {
//     execute(
//         "a().b().c();",
//         r#"
//         (program
//             (expr (call (member "static" (call (member "static" (call (id a)) (id b))) (id c)))))
//         "#,
//     )
// }

#[test]
fn test_call_with_boolean_argument() {
    execute(
        "setEnabled(true);",
        r#"
        (program
            (expr (call (id setEnabled) (args (boolean true)))))
        "#,
    )
}

#[test]
fn test_call_with_string_argument() {
    execute(
        r#"log("Hello, world!");"#,
        r#"
        (program
            (expr (call (id log) (args (string "Hello, world!")))))
        "#,
    )
}
