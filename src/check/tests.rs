use super::{Ty, analyze, check};
use crate::ast::{ExprKind, PrimitiveKind, Stmt};
use crate::lexer::{Diagnostic, Lexer, SourceFile};
use crate::parser::Parser;

fn parse_ignoring_problems(source: &str) -> Vec<Stmt> {
    let source = SourceFile::new("test", source);
    Parser::new(Lexer::new(&source)).parse_file()
}

/// Checks a program that must lex and parse cleanly.
fn check_clean_parse(source: &str) -> Vec<Diagnostic> {
    let file = SourceFile::new("test", source);
    let mut parser = Parser::new(Lexer::new(&file));
    let statements = parser.parse_file();
    assert_eq!(parser.lexer().diagnostics(), [], "lexer diagnostics for {source:?}");
    assert_eq!(parser.diagnostics(), [], "parser diagnostics for {source:?}");
    check(&statements)
}

fn messages(diagnostics: &[Diagnostic]) -> Vec<&str> {
    diagnostics.iter().map(|d| d.message.as_str()).collect()
}

fn texts<'s>(source: &'s str, diagnostics: &[Diagnostic]) -> Vec<&'s str> {
    diagnostics.iter().map(|d| &source[d.start..d.end]).collect()
}

/// Each source has exactly one problem: `message`, at the source text `at`.
fn assert_reported_once(cases: &[(&str, &str, &str)]) {
    for &(source, message, at) in cases {
        let diagnostics = check_clean_parse(source);

        assert_eq!(messages(&diagnostics), [message], "{source:?}");
        assert_eq!(texts(source, &diagnostics), [at], "{source:?}");
    }
}

fn assert_clean(sources: &[&str]) {
    for source in sources {
        assert_eq!(messages(&check_clean_parse(source)), Vec::<&str>::new(), "{source:?}");
    }
}

#[test]
fn clean_programs_have_no_problems() {
    assert_clean(&[
        "fn f(a: int): int { return a; } f(1);",
        "f(1); fn f(a: int) {}",
        "fn even(n: int): bool { return odd(n); } fn odd(n: int): bool { return even(n); }",
        "var x = 1; { var x = 2; x; } x;",
        "var a = 1, b = a;",
        "var x = 1; x = 2; x++; x += 3;",
        "if true { var t = 1; t; } else { var t = 2; t; }",
        "if true {} else if false {} else {}",
        "loop { break; }",
        "loop var i = 0; i < 10; i++ { continue if i == 2; break if i == 5; }",
        "var xs = 0..3; loop var x in xs { x; }",
        "loop var i = 0; i < 3; i++ {} loop var i = 0; i < 3; i++ {}",
        "fn f() { return; } fn g() { if true { return; } }",
        "fn f(): int, bool { return 1, true; } fn g(): int { return 1 if true; return 2; }",
        "fn f(i: int, j: int) {} f(1, 2); f(i: 1, j: 2); f(j: 1, i: 2);",
        "fn outer(a: int) { fn inner(): int { return a; } inner(); }",
        "fn f() { loop { fn g() { loop { break; } } break; } }",
        "fn f(a: int) { { var a = 1; } }",
        "var f = 1; { fn f() {} f(); }",
    ]);
}

#[test]
fn problem_is_reported_once() {
    assert_reported_once(&[
        ("x;", "'x' is not defined", "x"),
        ("var x = x;", "'x' is not defined", "x"),
        ("{ var t = 1; } t;", "'t' is not defined", "t"),
        ("loop var i in 0..3 {} i;", "'i' is not defined", "i"),
        ("loop var i = 0; i < 3; i++ {} i;", "'i' is not defined", "i"),
        ("fn f(a: int) {} a;", "'a' is not defined", "a"),
        ("var a = b, b = 1;", "'b' is not defined", "b"),
        ("fn f() { g(); } { fn g() {} }", "'g' is not defined", "g"),
        ("var x = 1; var x = 2;", "'x' is already declared in this scope", "x = 2"),
        ("var a = 1, a: int;", "'a' is already declared in this scope", "a: int"),
        ("fn f(a: int, a: bool) {}", "'a' is already declared in this scope", "a: bool"),
        ("fn f(a: int) { var a = 1; }", "'a' is already declared in this scope", "a = 1"),
        ("var f = 1; fn f() {}", "'f' is already declared in this scope", "f = 1"),
        ("break;", "'break' outside a loop", "break;"),
        ("continue if true;", "'continue' outside a loop", "continue if true;"),
        ("loop { fn g() { break; } }", "'break' outside a loop", "break;"),
        ("return;", "'return' outside a function", "return;"),
        ("fn f(): int { return; }", "expected 1 return value, found none", "return;"),
        ("fn f() { return 1; }", "expected no return value, found 1", "return 1;"),
        ("fn f(): int, int { return 1; }", "expected 2 return values, found 1", "return 1;"),
        ("fn f(a: int) {} f();", "'f' expects 1 argument, found 0", "f()"),
        ("fn f() {} f(1, 2);", "'f' expects 0 arguments, found 2", "f(1, 2)"),
        ("fn f(i: int, j: int) {} f(j: 1);", "missing argument 'i' for 'f'", "f(j: 1)"),
        ("fn f(i: int) {} f(i: 1, k: 2);", "'f' has no parameter named 'k'", "k: 2"),
        ("fn f() {} f = 1;", "cannot assign to function 'f'", "f"),
        ("fn f() {} f++;", "cannot assign to function 'f'", "f"),
    ]);
}

#[test]
fn each_problem_is_reported() {
    let cases: [(&str, &[&str]); 4] = [
        (
            "fn f(i: int) {} f(k: 1);",
            &["'f' has no parameter named 'k'", "missing argument 'i' for 'f'"],
        ),
        ("fn f(i: int, j: int, k: int) {} f(j: 1);", &["missing arguments 'i', 'k' for 'f'"]),
        ("x; x;", &["'x' is not defined", "'x' is not defined"]),
        (
            "fn f(): int { return; } fn g() { return 1; }",
            &["expected 1 return value, found none", "expected no return value, found 1"],
        ),
    ];
    for (source, expected) in cases {
        assert_eq!(messages(&check_clean_parse(source)), expected, "{source:?}");
    }
}

#[test]
fn calls_the_checker_cannot_match_are_left_alone() {
    // A mix of positional and named arguments was reported by the parser.
    let statements = parse_ignoring_problems("fn f(i: int) {} f(1, i: 2);");

    assert_eq!(check(&statements), []);
}

#[test]
fn well_typed_programs_have_no_problems() {
    assert_clean(&[
        "var f: float = 1; var d: double = 1.5; var b: byte = -128; var u: ubyte = 255;",
        "var s: size = 1; var u: usize = 2; var l: ulong = 0xFFFF_FFFF; var d = 1.5 * 2;",
        "var s = `a` + `b`; print(s); print(1); print(true); print(1.5); print('c');",
        "var c = 65 as char; var n = 'a' as int; var l = 1 as long + 2; var b = 300 as byte;",
        "var i: int = 1; var d = i as double / 2.0; var u = -i as uint;",
        "var x: long = 1; var y = x + 1; var z: long = y;",
        "var b: byte = 1; b += 1; b++; b = b * 2 - 1;",
        "var x = 1; x = 2 + x * 3; var even = x % 2 == 0;",
        "var a = 1 == 1 && 2 < 3 || `a` == `b` && 'x' < 'y' && !false;",
        "fn f(): int { loop { return 1; } }",
        "fn f(x: bool): int { if x { return 1; } else { return 2; } }",
        "fn f(x: bool): int { if x { return 1; } else if !x { return 2; } else { return 3; } }",
        "fn f(): int { { return 1; } }",
        "fn f(): int { loop { loop { break; } return 1; } }",
        "loop var i: long in 0..3 { var y: long = i; }",
        "var r = 0..10; loop var i in r { var j: int = i; }",
        "fn print(s: string) {} print(`x`);",
        "fn f(a: int, b: bool): long { return a as long if b; return 0; } var x: long = f(b: true, a: 1);",
        "fn pair(): int, bool { return 1, true; } pair();",
        "var d: double = 1; var f = d * 3 + 0.5; var g: double = f;",
    ]);
}

#[test]
fn type_problem_is_reported_once() {
    assert_reported_once(&[
        ("var x: int = true;", "expected 'int', found 'bool'", "true"),
        ("var i: int = 1.5;", "expected 'int', found float constant", "1.5"),
        ("var s: string = 'c';", "expected 'string', found 'char'", "'c'"),
        ("var x: int = 1; var y: long = x;", "expected 'long', found 'int'", "x"),
        ("var b: byte = 300;", "300 does not fit in 'byte'", "300"),
        ("var u: uint = -1;", "-1 does not fit in 'uint'", "-1"),
        ("var u: ubyte = (256);", "256 does not fit in 'ubyte'", "(256)"),
        ("var x = 3_000_000_000;", "3000000000 does not fit in 'int'", "3_000_000_000"),
        ("var b: byte = 1; var c = b + 200;", "200 does not fit in 'byte'", "200"),
        ("var x;", "'x' needs a type or an initializer", "x"),
        ("fn g() {} var x = g();", "expected a value, found no value", "g()"),
        (
            "fn g(): int, int { return 1, 2; } var x = g();",
            "expected a value, found 2 values",
            "g()",
        ),
        ("fn g() {} print(g());", "expected a value, found no value", "g()"),
        ("if 1 {}", "expected 'bool', found integer constant", "1"),
        ("loop `forever` {}", "expected 'bool', found 'string'", "`forever`"),
        ("var x = 1; loop var i = 0; x; i++ {}", "expected 'bool', found 'int'", "x"),
        ("loop { break if 1; }", "expected 'bool', found integer constant", "1"),
        ("fn f() { return if 2; }", "expected 'bool', found integer constant", "2"),
        ("var a = 1 + true;", "cannot apply '+' to integer constant and 'bool'", "1 + true"),
        ("var s = `a` + 1;", "cannot apply '+' to 'string' and integer constant", "`a` + 1"),
        ("var s = `a` - `b`;", "cannot apply '-' to 'string' and 'string'", "`a` - `b`"),
        (
            "var i: int = 1; var l: long = 2; var x = i * l;",
            "cannot apply '*' to 'int' and 'long'",
            "i * l",
        ),
        (
            "var x: int = 1; var y = x + 1.5;",
            "cannot apply '+' to 'int' and float constant",
            "x + 1.5",
        ),
        ("var a = 1 < true;", "cannot apply '<' to integer constant and 'bool'", "1 < true"),
        ("var a = true < false;", "cannot apply '<' to 'bool' and 'bool'", "true < false"),
        ("var a = true && 1;", "cannot apply '&&' to 'bool' and integer constant", "true && 1"),
        ("var b = !1;", "cannot apply '!' to integer constant", "!1"),
        ("var u: uint = 1; var n = -u;", "cannot negate 'uint'", "-u"),
        ("var r = 0..1.5;", "range bounds must be integers, found float constant", "0..1.5"),
        (
            "var i: int = 1; var l: long = 2; var r = i..l;",
            "range bounds have different types, 'int' and 'long'",
            "i..l",
        ),
        ("var n = 5; loop var x in n {}", "cannot loop over 'int'", "n"),
        (
            "loop var x: bool in 0..3 {}",
            "the loop variable is 'bool', but the range holds 'int'",
            "bool",
        ),
        ("loop var x: long in 0..3 { var y: int = x; }", "expected 'int', found 'long'", "x"),
        ("var x = 1; x = true;", "expected 'int', found 'bool'", "true"),
        ("var s = `a`; s -= `b`;", "cannot apply '-=' to 'string' and 'string'", "s -= `b`"),
        ("var b = true; b++;", "cannot apply '++' to 'bool'", "b++"),
        ("fn f(a: int) {} f(true);", "expected 'int', found 'bool'", "true"),
        ("fn f(a: int, b: bool) {} f(b: 1, a: 2);", "expected 'bool', found integer constant", "1"),
        ("fn f(): int { return true; }", "expected 'int', found 'bool'", "true"),
        ("fn f(): int, bool { return 1, 2; }", "expected 'bool', found integer constant", "2"),
        (
            "fn f(): int { if true { return 1; } }",
            "'f' does not return on every path",
            "fn f(): int",
        ),
        ("fn f(): int { loop { break; } }", "'f' does not return on every path", "fn f(): int"),
        (
            "fn f(): int { loop { if true { break; } return 1; } }",
            "'f' does not return on every path",
            "fn f(): int",
        ),
        ("fn f(): int { return 1 if true; }", "'f' does not return on every path", "fn f(): int"),
        (
            "fn f(x: int): int { loop var i in 0..x { return i; } }",
            "'f' does not return on every path",
            "fn f(x: int): int",
        ),
        ("var x = 1; x();", "cannot call 'int'", "x"),
        ("fn f(i: int) {} { var f = 1; f(); }", "cannot call 'int'", "f"),
        ("fn f() {} var g = f;", "function 'f' cannot be used as a value", "f"),
        ("var x = 1; x[0];", "cannot index 'int'", "x"),
        ("var x = 1; x.y;", "'int' has no field 'y'", "x.y"),
        ("var c = `a` as int;", "cannot convert 'string' to 'int'", "`a` as int"),
        ("var b = true as int;", "cannot convert 'bool' to 'int'", "true as int"),
        ("var c = 1.5 as char;", "cannot convert float constant to 'char'", "1.5 as char"),
        ("print();", "'print' expects 1 argument, found 0", "print()"),
        ("print(1, 2);", "'print' expects 1 argument, found 2", "print(1, 2)"),
    ]);
}

#[test]
fn undefined_name_is_reported_once_per_use_and_not_again_by_what_uses_it() {
    let cases = [
        ("var y: int = x + 1 * 2;", 1),
        ("var a = b..c;", 2),
        ("print(-z);", 1),
        ("loop var i in nope { i + 1; }", 1),
        ("var x = 1; x = nope; x += nope;", 2),
        ("f(x: g());", 2),
    ];
    for (source, uses) in cases {
        let diagnostics = check_clean_parse(source);
        let messages = messages(&diagnostics);

        assert_eq!(messages.len(), uses, "{source:?}: {messages:?}");
        assert!(messages.iter().all(|m| m.ends_with("is not defined")), "{source:?}: {messages:?}");
    }
}

#[test]
fn literals_record_the_type_they_settled_into() {
    let statements = parse_ignoring_problems(
        "var b: byte = 1 + 2; var d: double = 1; var c = 1 < 1.5; print(7); loop var i: long in 0..3 {}",
    );
    let analysis = analyze(&statements);
    assert_eq!(analysis.diagnostics(), []);

    let initializer = |index: usize| {
        let Stmt::Var(var) = &statements[index] else { panic!("not a var") };
        var.declarators[0].initializer.as_ref().expect("an initializer")
    };
    let byte = Ty::Primitive(PrimitiveKind::Byte);

    let sum = initializer(0);
    let ExprKind::Binary { left, .. } = &sum.kind else { panic!("not a binary") };
    assert_eq!(*analysis.type_of(sum), byte);
    assert_eq!(*analysis.type_of(left), byte);
    assert_eq!(*analysis.type_of(initializer(1)), Ty::DOUBLE);

    let comparison = initializer(2);
    let ExprKind::Binary { left, .. } = &comparison.kind else { panic!("not a binary") };
    assert_eq!(*analysis.type_of(comparison), Ty::BOOL);
    assert_eq!(*analysis.type_of(left), Ty::FloatConstant);

    let Stmt::Expression(print) = &statements[3] else { panic!("not an expression") };
    let ExprKind::Call { arguments, .. } = &print.expr.kind else { panic!("not a call") };
    assert_eq!(*analysis.type_of(&arguments[0].value), Ty::INT);

    let Stmt::LoopIn(loop_in) = &statements[4] else { panic!("not a loop") };
    let ExprKind::Range { upper, .. } = &loop_in.iterable.kind else { panic!("not a range") };
    let long = Ty::Primitive(PrimitiveKind::Long);
    assert_eq!(*analysis.type_of(&loop_in.iterable), Ty::range(long.clone()));
    assert_eq!(*analysis.type_of(upper), long);
}
