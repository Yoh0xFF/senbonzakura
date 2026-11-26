use super::{Interpreter, RuntimeError};
use senbonzakura_check as check;
use senbonzakura_lexer::{Lexer, SourceFile};
use senbonzakura_parser::Parser;

/// Runs a program that must check cleanly, on a large stack, and returns what it printed and how
/// it ended.
fn execute(program: &str) -> (String, Result<(), RuntimeError>) {
    let program = program.to_string();
    crate::with_large_stack(move || {
        let source = SourceFile::new("test", program.as_str());
        let mut parser = Parser::new(Lexer::new(&source));
        let statements = parser.parse_file();
        let analysis = check::analyze(&statements);
        assert_eq!(parser.lexer().diagnostics(), [], "lexer diagnostics for {program:?}");
        assert_eq!(parser.diagnostics(), [], "parser diagnostics for {program:?}");
        assert_eq!(analysis.diagnostics(), [], "checker diagnostics for {program:?}");
        let mut out = Vec::new();
        let result = Interpreter::new(&analysis, &mut out).run(&statements);
        (String::from_utf8(out).expect("UTF-8 output"), result)
    })
}

/// What a program that runs without failing prints, with `|` for each line break.
fn run(program: &str) -> String {
    let (out, result) = execute(program);
    if let Err(error) = result {
        panic!("{program:?} failed: {error}");
    }
    out.strip_suffix('\n').unwrap_or(&out).replace('\n', "|")
}

/// Each program prints what is paired with it.
fn assert_prints(cases: &[(&str, &str)]) {
    for &(program, output) in cases {
        assert_eq!(run(program), output, "{program:?}");
    }
}

#[test]
fn expressions() {
    assert_prints(&[
        ("print(1);", "1"),
        ("print(1 + 2 * 3); print((1 + 2) * 3);", "7|9"),
        ("print(`a` + `b`); print(`x` < `y`);", "ab|true"),
        ("print(true && !false); print(false || 1 > 2);", "true|false"),
        ("print('x'); print('😀'); print('\\n' as int);", "x|😀|10"),
        ("print(7 / 2); print(-7 / 2); print(-7 % 3);", "3|-3|-1"),
        ("print(7.0 / 2); print(1 / 2.0);", "3.5|0.5"),
        ("var f: float = 0.1; print(f); print(f * 2);", "0.1|0.2"),
        ("print(0..3); var r = 2..5; print(r);", "0..3|2..5"),
        (
            "var i: int; var s: string; var b: bool; var d: double; print(i); print(s); print(b); print(d);",
            "0||false|0.0",
        ),
        ("var x = 1; print(x++); print(x); print(x--); print(x);", "1|2|2|1"),
        ("var x = 5; x *= 2; x -= 1; x /= 3; x %= 2; print(x);", "1"),
        ("var s = `a`; s += `b`; s += `c`; print(s);", "abc"),
        ("var a = 1; var b = 2; a = b = 7; print(a + b);", "14"),
        ("var a = 1, b = a + 1; print(b);", "2"),
    ]);
}

#[test]
fn integers_wrap_around_their_width() {
    assert_prints(&[
        ("var b: byte = 127; b++; print(b);", "-128"),
        ("var u: ubyte = 0; u--; print(u);", "255"),
        ("var s: short = 32767; s += 1; print(s);", "-32768"),
        ("var i: int = 2147483647; i = i + 1; print(i);", "-2147483648"),
        (
            "var x: uint = 4_000_000_000; print(x); x += 300_000_000; print(x);",
            "4000000000|5032704",
        ),
        (
            "var m: ulong = 0; m--; print(m); print(m / 2); print(m > 1);",
            "18446744073709551615|9223372036854775807|true",
        ),
        ("var m: ulong = 0; m--; print(m % 10);", "5"),
    ]);
}

#[test]
fn casts() {
    assert_prints(&[
        ("print(300 as byte); print(-1 as ubyte);", "44|255"),
        ("print(65 as char); print('a' as int);", "A|97"),
        ("print(3.9 as int); print(-3.9 as int);", "3|-3"),
        ("print(1e20 as int); print(-1e20 as long);", "2147483647|-9223372036854775808"),
        ("print(-1.0 as uint); print(1e30 as ulong);", "0|18446744073709551615"),
        ("var i: int = -1; print(i as ulong); print(i as long);", "18446744073709551615|-1"),
        ("var m: ulong = 0; m--; print(m as double);", "1.8446744073709552E19"),
        ("var i: int = 7; print(i as double / 2);", "3.5"),
        ("var d: double = 0.1; print(d as float);", "0.1"),
    ]);
}

#[test]
fn control_flow() {
    assert_prints(&[
        ("if 1 < 2 { print(1); } else { print(2); }", "1"),
        ("var x = 3; if x == 1 { print(1); } else if x == 3 { print(3); } else { print(0); }", "3"),
        ("var i = 0; loop i < 3 { print(i); i++; }", "0|1|2"),
        ("var i = 0; loop { i++; break if i == 3; } print(i);", "3"),
        ("loop var i in 0..4 { continue if i == 1; print(i); }", "0|2|3"),
        ("loop var i = 0; i < 6; i += 2 { print(i); }", "0|2|4"),
        ("loop var i = 0; i < 5; i++ { continue if i % 2 == 0; print(i); }", "1|3"),
        (
            "loop var i in 0..3 { loop var j in 0..3 { break if j > i; print(i * 10 + j); } }",
            "0|10|11|20|21|22",
        ),
        ("loop var i: long in 0..2 { print(i); }", "0|1"),
        ("var u: ubyte = 253; loop var i in u..255 { print(i); }", "253|254"),
        ("var n = 0; loop var i in 0..0 { n++; } print(n);", "0"),
        ("var s = 0; loop var i in 1..101 { s += i; } print(s);", "5050"),
    ]);
}

#[test]
fn functions() {
    assert_prints(&[
        ("fn add(a: int, b: int): int { return a + b; } print(add(2, 3));", "5"),
        ("fn sub(a: int, b: int): int { return a - b; } print(sub(b: 1, a: 10));", "9"),
        ("print(f()); fn f(): int { return 1; }", "1"),
        (
            "fn fib(n: int): int { return n if n < 2; return fib(n - 1) + fib(n - 2); } print(fib(20));",
            "6765",
        ),
        (
            "fn even(n: int): bool { return true if n == 0; return odd(n - 1); } fn odd(n: int): bool { return false if n == 0; return even(n - 1); } print(even(10)); print(odd(7));",
            "true|true",
        ),
        ("fn hello() { print(`hi`); return; print(`unreachable`); } hello();", "hi"),
        ("fn pair(): int, int { return 1, 2; } pair(); print(0);", "0"),
        (
            "fn find(n: int): int { loop var i in 0..n { return i if i * i > n; } return -1; } print(find(20));",
            "5",
        ),
        ("fn f(): int { loop { return 7; } } print(f());", "7"),
        ("var x = 1; fn show() { print(x); } show(); x = 2; show();", "1|2"),
        (
            "fn outer(a: int): int { fn inner(b: int): int { return a + b; } return inner(10); } print(outer(1));",
            "11",
        ),
        ("var x = 1; { var x = 2; print(x); } print(x);", "2|1"),
        ("fn show(n: int) { return if n > 0; print(n); } show(1); show(0);", "0"),
        ("fn twice(f: int): int { return f * 2; } print(twice(twice(3)));", "12"),
    ]);
}

#[test]
fn runtime_errors_point_at_the_failing_code() {
    let cases = [
        ("print(1 / 0);", "division by zero", "1 / 0"),
        ("var x = 0; print(5 % x);", "division by zero", "5 % x"),
        ("var i = 0x110000; print(i as char);", "1114112 is not a valid char", "i as char"),
        ("var i = 0xD800; print(i as char);", "55296 is not a valid char", "i as char"),
        ("var x = 10; x /= 0;", "division by zero", "x /= 0"),
    ];
    for (program, message, at) in cases {
        let (_, result) = execute(program);
        let error = result.expect_err(program);

        assert_eq!(error.message, message, "{program:?}");
        let (start, end) = error.location.expect("a location");
        assert_eq!(&program[start..end], at, "{program:?}");
    }
}

#[test]
fn floats_divide_by_zero_without_failing() {
    assert_eq!(run("var x = 1.0; print(x / 0); print(0.0 / 0.0);"), "Infinity|NaN");
}

#[test]
fn output_before_a_failure_is_kept() {
    let (out, result) = execute("print(1); print(2 / 0); print(3);");

    assert!(result.is_err());
    assert_eq!(out, "1\n");
}

#[test]
fn runaway_recursion_is_a_stack_overflow() {
    let (_, result) = execute("fn f(): int { return f() + 1; } f();");
    let error = result.expect_err("a stack overflow");

    assert_eq!(error.message, "stack overflow: the calls nest too deeply");
    assert_eq!(error.location, None);
}

#[test]
fn recursion_just_inside_the_limit_runs() {
    let program = format!(
        "fn down(n: int): int {{ return 0 if n == 0; return down(n - 1) + 1; }} print(down({}));",
        super::MAX_CALL_DEPTH - 1
    );

    assert_eq!(run(&program), (super::MAX_CALL_DEPTH - 1).to_string());
}
