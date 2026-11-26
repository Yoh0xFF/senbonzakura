use super::{parse, parse_clean};
use senbonzakura_ast::{Expr, ExprKind};

#[test]
fn identifiers() {
    for input in ["x", "_", "_x1", "camelCase", "snake_case", "ünïcödé", "日本語"] {
        let parsed = parse_clean(input);

        let expected = Expr::new(ExprKind::Identifier(input.to_string()), 0, input.len());
        assert_eq!(parsed.expr, expected);
    }
}

#[test]
fn keyword_is_not_an_identifier() {
    let parsed = parse("fn");

    assert_eq!(parsed.expr, Expr::new(ExprKind::Error, 0, 2));
    assert_eq!(parsed.parser_messages(), ["expected expression, found 'fn'"]);
}
