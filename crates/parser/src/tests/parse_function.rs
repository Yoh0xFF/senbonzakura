use super::{parse_all, parse_stmt, parse_stmt_clean};
use senbonzakura_ast::Stmt;

#[test]
fn functions() {
  let cases = [
    ("fn test1(i: int, j: int): int { x; }", "(fn test1 (i: int, j: int): int {x;})"),
    ("fn test2(i: int, j: int): int, int {}", "(fn test2 (i: int, j: int): int, int {})"),
    ("fn test3(i: int) {}", "(fn test3 (i: int) {})"),
    ("fn test4() {}", "(fn test4 () {})"),
    ("fn f(a: int,) {}", "(fn f (a: int) {})"),
    ("fn f(s: string, c: char): ulong {}", "(fn f (s: string, c: char): ulong {})"),
    ("fn f(): bool { return x if y; }", "(fn f (): bool {(return x if y)})"),
    ("fn f() { fn g() {} }", "(fn f () {(fn g () {})})"),
    ("fn f(): int, bool { return 1, true; }", "(fn f (): int, bool {(return 1, true)})"),
  ];
  for (input, tree) in cases {
    assert_eq!(parse_stmt_clean(input).tree(), tree, "{input:?}");
  }
}

#[test]
fn function_covers_its_source() {
  let parsed = parse_stmt_clean("fn add(a: int, b: int): int, bool { x; }");
  let Stmt::Function(f) = &parsed.stmt else { panic!("not a function") };

  assert_eq!(parsed.text(&parsed.stmt), "fn add(a: int, b: int): int, bool { x; }");
  assert_eq!(f.name, "add");
  assert_eq!(parsed.text(&f.parameters[0]), "a: int");
  assert_eq!(parsed.text(&f.parameters[1].ty), "int");
  assert_eq!(parsed.text(&f.return_types[1]), "bool");
  assert_eq!(parsed.text(&f.body), "{ x; }");
}

#[test]
fn function_needs_no_semicolon() {
  let all = parse_all("fn f() {} fn g() {} x;");

  assert_eq!(all.trees, ["(fn f () {})", "(fn g () {})", "x;"]);
  assert_eq!(all.messages, Vec::<String>::new());
}

#[test]
fn malformed_function_is_reported_once() {
  let cases = [
    ("fn (a: int) {}", "(fn <error> (a: int) {})", "expected function name, found '('"),
    ("fn 1() {}", "(fn <error> () {})", "expected function name, found '1'"),
    ("fn f {}", "(fn f () {})", "expected '(' after function name, found '{'"),
    ("fn f(a: int {}", "(fn f (a: int) {})", "expected ')', found '{'"),
    ("fn f(a: int", "(fn f (a: int) {})", "expected ')', found end of input"),
    ("fn f(a int) {}", "(fn f (a: int) {})", "expected ':' after parameter name, found 'int'"),
    ("fn f(a) {}", "(fn f (a: <error>) {})", "expected ':' after parameter name, found ')'"),
    ("fn f(a:) {}", "(fn f (a: <error>) {})", "expected type, found ')'"),
    ("fn f(a: foo) {}", "(fn f (a: <error>) {})", "expected type, found 'foo'"),
    ("fn f(a: int b: int) {}", "(fn f (a: int, b: int) {})", "expected ',' or ')', found 'b'"),
    (
      "fn f(, a: int) {}",
      "(fn f (<error>: <error>, a: int) {})",
      "expected parameter name, found ','",
    ),
    ("fn f(): {}", "(fn f (): <error> {})", "expected type, found '{'"),
    ("fn f(): int, {}", "(fn f (): int, <error> {})", "expected type, found '{'"),
    ("fn f() int {}", "(fn f (): int {})", "expected ':' before return type, found 'int'"),
    ("fn f()", "(fn f () {})", "expected '{' after function signature, found end of input"),
    (
      "fn f(): int",
      "(fn f (): int {})",
      "expected '{' after function signature, found end of input",
    ),
    ("fn f() { x;", "(fn f () {x;})", "expected '}', found end of input"),
  ];
  for (input, tree, message) in cases {
    let parsed = parse_stmt(input);

    assert_eq!(parsed.tree(), tree, "{input:?}");
    assert_eq!(parsed.parser_messages(), [message], "{input:?}");
  }
}

#[test]
fn missing_body_leaves_the_rest_for_the_next_statement() {
  let all = parse_all("fn f() x; y;");

  assert_eq!(all.trees, ["(fn f () {})", "x;", "y;"]);
  assert_eq!(all.messages, ["expected '{' after function signature, found 'x'"]);
}

#[test]
fn fn_is_not_an_expression() {
  let parsed = parse_stmt("{ x = fn; }");

  assert_eq!(parsed.parser_messages(), ["expected expression, found 'fn'"]);
}
