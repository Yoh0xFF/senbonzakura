use super::{parse, parse_clean, parse_first};
use senbonzakura_ast::ExprKind;

fn assert_trees(cases: &[(&str, &str)]) {
    for &(input, tree) in cases {
        assert_eq!(parse_clean(input).tree(), tree, "{input:?}");
    }
}

#[test]
fn calls() {
    assert_trees(&[
        ("f()", "(call f)"),
        ("f(1)", "(call f 1)"),
        ("f(1, 2)", "(call f 1 2)"),
        ("f(1, 2,)", "(call f 1 2)"),
        ("f(a + b, -c)", "(call f (+ a b) (- c))"),
        ("f(g(x))", "(call f (call g x))"),
    ]);
}

#[test]
fn indexing() {
    assert_trees(&[
        ("a[0]", "(index a 0)"),
        ("a[i + 1]", "(index a (+ i 1))"),
        ("a[0..2]", "(index a (.. 0 2))"),
        ("a[b[0]]", "(index a (index b 0))"),
    ]);
}

#[test]
fn field_access() {
    assert_trees(&[("a.b", "(. a b)"), ("a.b.c", "(. (. a b) c)"), ("1.foo", "(. 1 foo)")]);
}

#[test]
fn postfix_operators_chain_left_to_right() {
    assert_trees(&[
        ("a.b(c)[0].d", "(. (index (call (. a b) c) 0) d)"),
        ("f()()", "(call (call f))"),
        ("a[0][1]", "(index (index a 0) 1)"),
        ("(a + b).c", "(. (group (+ a b)) c)"),
        ("(f)(x)", "(call (group f) x)"),
    ]);
}

#[test]
fn postfix_binds_tighter_than_prefix_and_infix() {
    assert_trees(&[
        ("-a.b", "(- (. a b))"),
        ("!f()", "(! (call f))"),
        ("- -a[0]", "(- (- (index a 0)))"),
        ("-a[0] * b.c", "(* (- (index a 0)) (. b c))"),
        ("a.b + f(c)", "(+ (. a b) (call f c))"),
    ]);
}

#[test]
fn fields_and_indexes_can_be_assigned() {
    assert_trees(&[
        ("a.b = 1", "(= (. a b) 1)"),
        ("a[0] += 1", "(+= (index a 0) 1)"),
        ("f().x = 1", "(= (. (call f) x) 1)"),
        ("a.b[i].c = d", "(= (. (index (. a b) i) c) d)"),
    ]);
}

#[test]
fn call_cannot_be_assigned() {
    let parsed = parse("f() = 1");

    assert_eq!(parsed.parser_messages(), ["invalid assignment target"]);
    assert_eq!(parsed.parser_texts(), ["f()"]);
}

#[test]
fn nodes_cover_their_source() {
    let parsed = parse_clean("a.b(c)[0]");
    let ExprKind::Index { target: call, .. } = &parsed.expr.kind else { panic!() };
    let ExprKind::Call { callee: field, .. } = &call.kind else { panic!() };

    assert_eq!(parsed.expr_text(), "a.b(c)[0]");
    assert_eq!(parsed.text(&**call), "a.b(c)");
    assert_eq!(parsed.text(&**field), "a.b");
}

#[test]
fn malformed_postfix_is_reported() {
    let cases = [
        ("f(1 2)", "(call f 1 2)", "expected ',' or ')', found '2'"),
        ("f(1", "(call f 1)", "expected ')', found end of input"),
        ("f(", "(call f)", "expected ')', found end of input"),
        ("f(,)", "(call f <error>)", "expected expression, found ','"),
        ("f(1;", "(call f 1)", "expected ')', found ';'"),
        ("a[]", "(index a <error>)", "expected expression, found ']'"),
        ("f(1 {", "(call f 1)", "expected ')', found '{'"),
        ("f(1 if", "(call f 1)", "expected ')', found 'if'"),
        ("a[0", "(index a 0)", "expected ']', found end of input"),
        ("a.", "(. a <error>)", "expected field name, found end of input"),
    ];
    for (input, tree, message) in cases {
        let parsed = parse_first(input);

        assert_eq!(parsed.tree(), tree, "{input:?}");
        assert_eq!(parsed.parser_messages(), [message], "{input:?}");
    }
}

#[test]
fn increment_and_decrement() {
    assert_trees(&[
        ("i++", "(++ i)"),
        ("i--", "(-- i)"),
        ("a.b++", "(++ (. a b))"),
        ("a[0]--", "(-- (index a 0))"),
        ("-i++", "(- (++ i))"),
        ("i++ + 1", "(+ (++ i) 1)"),
        ("i++ < n", "(< (++ i) n)"),
        ("a+++b", "(+ (++ a) b)"),
        ("a - -b", "(- a (- b))"),
    ]);
}

#[test]
fn update_covers_its_source() {
    let parsed = parse_clean("a.b[0]++");

    assert_eq!(parsed.expr_text(), "a.b[0]++");
}

#[test]
fn update_of_something_not_assignable_is_reported() {
    let cases = [
        ("f()++", "(++ (call f))", "invalid target for '++'"),
        ("1--", "(-- 1)", "invalid target for '--'"),
        ("i++++", "(++ (++ i))", "invalid target for '++'"),
        ("(i)++", "(++ (group i))", "invalid target for '++'"),
    ];
    for (input, tree, message) in cases {
        let parsed = parse(input);

        assert_eq!(parsed.tree(), tree, "{input:?}");
        assert_eq!(parsed.parser_messages(), [message]);
    }
}

#[test]
fn there_is_no_prefix_increment() {
    let parsed = parse_first("--i");

    assert_eq!(parsed.parser_messages(), ["expected expression, found '--'"]);
}

#[test]
fn named_arguments() {
    assert_trees(&[
        ("test(i: 1, j: 2)", "(call test i: 1 j: 2)"),
        ("test(j: 4, i: 8)", "(call test j: 4 i: 8)"),
        ("f(i: 1,)", "(call f i: 1)"),
        ("f(i: i)", "(call f i: i)"),
        ("f(x: a + b, y: g(z: 1))", "(call f x: (+ a b) y: (call g z: 1))"),
        ("f(x: g(1, 2))", "(call f x: (call g 1 2))"),
    ]);
}

#[test]
fn named_argument_covers_its_source() {
    let parsed = parse_clean("f(i: 1 + 2, j: x)");
    let ExprKind::Call { arguments, .. } = &parsed.expr.kind else { panic!() };
    let i = &arguments[0];

    assert_eq!(i.name.as_deref(), Some("i"));
    assert_eq!(parsed.text(&i.value), "1 + 2");
    assert_eq!(parsed.text(i), "i: 1 + 2");
}

#[test]
fn positional_arguments_have_no_name() {
    let parsed = parse_clean("f(1, 2)");
    let ExprKind::Call { arguments, .. } = &parsed.expr.kind else { panic!() };

    assert!(!arguments[0].is_named());
    assert!(!arguments[1].is_named());
}

#[test]
fn mixing_positional_and_named_arguments_is_reported_once() {
    let cases = [
        ("f(1, j: 2)", "(call f 1 j: 2)", "j: 2"),
        ("f(i: 1, 2)", "(call f i: 1 2)", "2"),
        ("f(1, i: 2, j: 3)", "(call f 1 i: 2 j: 3)", "i: 2"),
        ("f(i: 1, 2, 3)", "(call f i: 1 2 3)", "2"),
    ];
    for (input, tree, at) in cases {
        let parsed = parse(input);

        assert_eq!(parsed.tree(), tree, "{input:?}");
        assert_eq!(parsed.parser_messages(), ["positional and named arguments cannot be mixed"]);
        assert_eq!(parsed.parser_texts(), [at]);
    }
}

#[test]
fn malformed_named_argument_is_reported_once() {
    let cases = [
        ("f(i: 1, i: 2)", "(call f i: 1 i: 2)", "argument 'i' is given twice"),
        ("f(a.b: 1)", "(call f <error>: 1)", "argument name must be an identifier"),
        ("f(1: 2)", "(call f <error>: 2)", "argument name must be an identifier"),
        ("f(i:)", "(call f i: <error>)", "expected expression, found ')'"),
        ("f(i: , j: 2)", "(call f i: <error> j: 2)", "expected expression, found ','"),
        ("f(i: 1 j: 2)", "(call f i: 1 j: 2)", "expected ',' or ')', found 'j'"),
    ];
    for (input, tree, message) in cases {
        let parsed = parse(input);

        assert_eq!(parsed.tree(), tree, "{input:?}");
        assert_eq!(parsed.parser_messages(), [message], "{input:?}");
    }
}

#[test]
fn unclosed_call_ends_at_its_last_argument() {
    let parsed = parse("f(1, 2");

    assert_eq!(parsed.expr_text(), "f(1, 2");
}

#[test]
fn keyword_is_not_a_field_name() {
    let parsed = parse_first("a.fn");

    assert_eq!(parsed.tree(), "(. a <error>)");
    assert_eq!(parsed.parser_messages(), ["expected field name, found 'fn'"]);
    assert_eq!(parsed.expr_text(), "a.");
}
