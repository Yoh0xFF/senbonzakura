use super::{parse_stmt, parse_stmt_clean};
use crate::ast::{PrimitiveKind, Stmt, TypeKind};

#[test]
fn every_primitive_type_parses() {
    for &kind in PrimitiveKind::ALL {
        let parsed = parse_stmt_clean(&format!("var x: {};", kind.spelling()));
        let Stmt::Var(var) = &parsed.stmt else { panic!("not a var: {:?}", parsed.stmt) };
        let ty = var.declarators[0].ty.as_ref().expect("a type");

        assert_eq!(ty.kind, TypeKind::Primitive(kind));
        assert_eq!(parsed.text(ty), kind.spelling());
    }
}

#[test]
fn primitive_type_names_are_not_variable_names() {
    for &kind in PrimitiveKind::ALL {
        let parsed = parse_stmt(&format!("var {} = 1;", kind.spelling()));

        assert_eq!(parsed.tree(), "(var <error> 1)");
        assert_eq!(
            parsed.parser_messages(),
            [format!("expected variable name, found '{}'", kind.spelling())]
        );
    }
}
