use super::{parse, parse_clean};
use senbonzakura_ast::{Expr, ExprKind};

#[test]
fn parentheses_override_precedence() {
    let cases = [
        ("(1)", "(group 1)"),
        ("(1 + 2) * 3", "(* (group (+ 1 2)) 3)"),
        ("1 - (2 - 3)", "(- 1 (group (- 2 3)))"),
        ("-(1 + 2)", "(- (group (+ 1 2)))"),
        ("((1))", "(group (group 1))"),
        ("(1 < 2) == true", "(== (group (< 1 2)) true)"),
    ];
    for (input, tree) in cases {
        assert_eq!(parse_clean(input).tree(), tree, "{input:?}");
    }
}

#[test]
fn missing_operand_before_close_paren_leaves_the_paren_for_the_group() {
    let parsed = parse("(1 + )");

    assert_eq!(parsed.tree(), "(group (+ 1 <error>))");
    assert_eq!(parsed.parser_messages(), ["expected expression, found ')'"]);
    assert_eq!(parsed.expr_text(), "(1 + )");
    let ExprKind::Group(inner) = &parsed.expr.kind else { panic!("not a group") };
    let ExprKind::Binary { right, .. } = &inner.kind else { panic!("not a binary") };
    assert_eq!(**right, Expr::new(ExprKind::Error, 5, 5));
}

#[test]
fn unclosed_paren_is_reported() {
    let parsed = parse("(1 + 2");

    assert_eq!(parsed.tree(), "(group (+ 1 2))");
    assert_eq!(parsed.parser_messages(), ["expected ')', found end of input"]);
    assert_eq!(parsed.expr_text(), "(1 + 2");
}
