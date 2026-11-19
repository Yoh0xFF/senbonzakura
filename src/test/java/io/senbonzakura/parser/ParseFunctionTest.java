package io.senbonzakura.parser;

import static io.senbonzakura.parser.ParsedStmt.parse;
import static io.senbonzakura.parser.ParsedStmt.parseAll;
import static io.senbonzakura.parser.ParsedStmt.parseClean;
import static org.junit.jupiter.api.Assertions.assertEquals;

import io.senbonzakura.ast.Stmt;
import java.util.List;
import org.junit.jupiter.api.Test;
import org.junit.jupiter.params.ParameterizedTest;
import org.junit.jupiter.params.provider.CsvSource;

class ParseFunctionTest {

  @ParameterizedTest
  @CsvSource(
      delimiterString = "=>",
      value = {
        "fn test1(i: int, j: int): int { x; }     => (fn test1 (i: int, j: int): int {x;})",
        "fn test2(i: int, j: int): int, int {}    => (fn test2 (i: int, j: int): int, int {})",
        "fn test3(i: int) {}                      => (fn test3 (i: int) {})",
        "fn test4() {}                            => (fn test4 () {})",
        "fn f(a: int,) {}                         => (fn f (a: int) {})",
        "fn f(s: string, c: char): ulong {}       => (fn f (s: string, c: char): ulong {})",
        "fn f(): bool { return x if y; }          => (fn f (): bool {(return x if y)})",
        "fn f() { fn g() {} }                     => (fn f () {(fn g () {})})",
        "fn f(): int, bool { return 1, true; }    => (fn f (): int, bool {(return 1, true)})",
      })
  void functions(String input, String tree) {
    assertEquals(tree, parseClean(input).tree());
  }

  @Test
  void functionCoversItsSource() {
    var parsed = parseClean("fn add(a: int, b: int): int, bool { x; }");
    var fn = parsed.as(Stmt.Function.class);

    assertEquals("fn add(a: int, b: int): int, bool { x; }", parsed.text(fn));
    assertEquals("add", fn.name());
    assertEquals("a: int", parsed.text(fn.parameters().get(0)));
    assertEquals("int", parsed.text(fn.parameters().get(1).type()));
    assertEquals("bool", parsed.text(fn.returnTypes().get(1)));
    assertEquals("{ x; }", parsed.text(fn.body()));
  }

  @Test
  void functionNeedsNoSemicolon() {
    var all = parseAll("fn f() {} fn g() {} x;");

    assertEquals(List.of("(fn f () {})", "(fn g () {})", "x;"), all.trees());
    assertEquals(List.of(), all.messages());
  }

  @ParameterizedTest
  @CsvSource(
      delimiterString = "=>",
      value = {
        "fn (a: int) {}           => (fn <error> (a: int) {})          => expected function name, found '('",
        "fn 1() {}                => (fn <error> () {})                => expected function name, found '1'",
        "fn f {}                  => (fn f () {})                      => expected '(' after function name, found '{'",
        "fn f(a: int {}           => (fn f (a: int) {})                => expected ')', found '{'",
        "fn f(a: int              => (fn f (a: int) {})                => expected ')', found end of input",
        "fn f(a int) {}           => (fn f (a: int) {})                => expected ':' after parameter name, found 'int'",
        "fn f(a) {}               => (fn f (a: <error>) {})            => expected ':' after parameter name, found ')'",
        "fn f(a:) {}              => (fn f (a: <error>) {})            => expected type, found ')'",
        "fn f(a: foo) {}          => (fn f (a: <error>) {})            => expected type, found 'foo'",
        "fn f(a: int b: int) {}   => (fn f (a: int, b: int) {})        => expected ',' or ')', found 'b'",
        "fn f(, a: int) {}        => (fn f (<error>: <error>, a: int) {}) => expected parameter name, found ','",
        "fn f(): {}               => (fn f (): <error> {})             => expected type, found '{'",
        "fn f(): int, {}          => (fn f (): int, <error> {})        => expected type, found '{'",
        "fn f() int {}            => (fn f (): int {})                 => expected ':' before return type, found 'int'",
        "fn f()                   => (fn f () {})                      => expected '{' after function signature, found end of input",
        "fn f(): int              => (fn f (): int {})                 => expected '{' after function signature, found end of input",
        "fn f() { x;              => (fn f () {x;})                    => expected '}', found end of input",
      })
  void malformedFunctionIsReportedOnce(String input, String tree, String message) {
    var parsed = parse(input);

    assertEquals(tree, parsed.tree());
    assertEquals(List.of(message), parsed.parserMessages());
  }

  @Test
  void missingBodyLeavesTheRestForTheNextStatement() {
    var all = parseAll("fn f() x; y;");

    assertEquals(List.of("(fn f () {})", "x;", "y;"), all.trees());
    assertEquals(List.of("expected '{' after function signature, found 'x'"), all.messages());
  }

  @Test
  void fnIsNotAnExpression() {
    var parsed = parse("{ x = fn; }");

    assertEquals(List.of("expected expression, found 'fn'"), parsed.parserMessages());
  }
}
