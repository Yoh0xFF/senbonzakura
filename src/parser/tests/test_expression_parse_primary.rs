use super::utils::execute;

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
