//! Parses types. So far only the primitive types, each spelled by a keyword of its own.

use super::cursor::TokenCursor;
use crate::ast::{PrimitiveKind, Type, TypeKind};
use crate::lexer::Kind;

/// Parses the type at the cursor. Anything else is reported, and consumed unless it can follow a
/// type, so `var x: foo = 1` reports `foo` once and still parses the initializer.
pub(crate) fn ty(c: &mut TokenCursor) -> Type {
    if let Some(kind) = primitive(c.peek().kind) {
        let token = c.advance();
        return Type::new(TypeKind::Primitive(kind), token.start, token.end);
    }

    let token = c.peek();
    c.report_found("expected type");
    if can_follow_type(c) {
        return Type::new(TypeKind::Error, token.start, token.start);
    }
    c.advance();
    Type::new(TypeKind::Error, token.start, token.end)
}

/// Whether the cursor is on a token that starts a type.
pub(crate) fn at_type(c: &TokenCursor) -> bool {
    primitive(c.peek().kind).is_some()
}

fn primitive(kind: Kind) -> Option<PrimitiveKind> {
    Some(match kind {
        Kind::Byte => PrimitiveKind::Byte,
        Kind::Short => PrimitiveKind::Short,
        Kind::Int => PrimitiveKind::Int,
        Kind::Size => PrimitiveKind::Size,
        Kind::Long => PrimitiveKind::Long,
        Kind::Ubyte => PrimitiveKind::Ubyte,
        Kind::Ushort => PrimitiveKind::Ushort,
        Kind::Uint => PrimitiveKind::Uint,
        Kind::Usize => PrimitiveKind::Usize,
        Kind::Ulong => PrimitiveKind::Ulong,
        Kind::Float => PrimitiveKind::Float,
        Kind::Double => PrimitiveKind::Double,
        Kind::Char => PrimitiveKind::Char,
        Kind::Bool => PrimitiveKind::Bool,
        Kind::String => PrimitiveKind::String,
        _ => return None,
    })
}

fn can_follow_type(c: &TokenCursor) -> bool {
    matches!(
        c.peek().kind,
        Kind::Eq
            | Kind::Comma
            | Kind::Semicolon
            | Kind::In
            | Kind::LeftBrace
            | Kind::RightBrace
            | Kind::RightParen
            | Kind::Eof
    )
}
