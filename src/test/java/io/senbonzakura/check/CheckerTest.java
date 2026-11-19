package io.senbonzakura.check;

import static org.junit.jupiter.api.Assertions.assertEquals;

import io.senbonzakura.ast.Stmt;
import io.senbonzakura.lexer.Lexer;
import io.senbonzakura.lexer.SourceFile;
import io.senbonzakura.lexer.Token.Diagnostic;
import io.senbonzakura.parser.Parser;
import java.util.List;
import org.junit.jupiter.params.ParameterizedTest;
import org.junit.jupiter.params.provider.CsvSource;
import org.junit.jupiter.params.provider.ValueSource;

class CheckerTest {

  @ParameterizedTest
  @ValueSource(
      strings = {
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
      })
  void cleanProgramsHaveNoProblems(String source) {
    assertEquals(List.of(), messages(check(source)));
  }

  @ParameterizedTest
  @CsvSource(
      quoteCharacter = '"',
      delimiterString = "=>",
      value = {
        "x;                                  => 'x' is not defined                    => x",
        "var x = x;                          => 'x' is not defined                    => x",
        "{ var t = 1; } t;                   => 't' is not defined                    => t",
        "loop var i in 0..3 {} i;            => 'i' is not defined                    => i",
        "loop var i = 0; i < 3; i++ {} i;    => 'i' is not defined                    => i",
        "fn f(a: int) {} a;                  => 'a' is not defined                    => a",
        "var a = b, b = 1;                   => 'b' is not defined                    => b",
        "fn f() { g(); } { fn g() {} }       => 'g' is not defined                    => g",
        "var x = 1; var x = 2;               => 'x' is already declared in this scope => x = 2",
        "var a = 1, a: int;                  => 'a' is already declared in this scope => a: int",
        "fn f(a: int, a: bool) {}            => 'a' is already declared in this scope => a: bool",
        "fn f(a: int) { var a = 1; }         => 'a' is already declared in this scope => a = 1",
        "var f = 1; fn f() {}                => 'f' is already declared in this scope => f = 1",
        "break;                              => 'break' outside a loop                => break;",
        "continue if true;                   => 'continue' outside a loop             => continue if true;",
        "loop { fn g() { break; } }          => 'break' outside a loop                => break;",
        "return;                             => 'return' outside a function           => return;",
        "fn f(): int { return; }             => expected 1 return value, found none   => return;",
        "fn f() { return 1; }                => expected no return value, found 1     => return 1;",
        "fn f(): int, int { return 1; }      => expected 2 return values, found 1     => return 1;",
        "fn f(a: int) {} f();                => 'f' expects 1 argument, found 0       => f()",
        "fn f() {} f(1, 2);                  => 'f' expects 0 arguments, found 2      => f(1, 2)",
        "fn f(i: int, j: int) {} f(j: 1);    => missing argument 'i' for 'f'          => f(j: 1)",
        "fn f(i: int) {} f(i: 1, k: 2);      => 'f' has no parameter named 'k'        => k: 2",
        "fn f() {} f = 1;                    => cannot assign to function 'f'         => f",
        "fn f() {} f++;                      => cannot assign to function 'f'         => f",
      })
  void problemIsReportedOnce(String source, String message, String at) {
    var diagnostics = check(source);

    assertEquals(List.of(message), messages(diagnostics));
    assertEquals(List.of(at), texts(source, diagnostics));
  }

  @ParameterizedTest
  @CsvSource(
      quoteCharacter = '"',
      delimiterString = "=>",
      value = {
        "fn f(i: int) {} f(k: 1);                       => 'f' has no parameter named 'k' | missing argument 'i' for 'f'",
        "fn f(i: int, j: int, k: int) {} f(j: 1);       => missing arguments 'i', 'k' for 'f'",
        "x; x;                                          => 'x' is not defined | 'x' is not defined",
        "fn f(): int { return; } fn g() { return 1; }   => expected 1 return value, found none | expected no return value, found 1",
      })
  void eachProblemIsReported(String source, String messages) {
    assertEquals(List.of(messages.split(" \\| ")), messages(check(source)));
  }

  @ParameterizedTest
  @ValueSource(
      strings = {
        // A mix of positional and named arguments was reported by the parser.
        "fn f(i: int) {} f(1, i: 2);",
        // A shadowing variable is not the function, so its calls are not checked against it.
        "fn f(i: int) {} { var f = 1; f(); }",
      })
  void callsTheCheckerCannotMatchAreLeftAlone(String source) {
    assertEquals(List.of(), messages(Checker.check(parseIgnoringProblems(source))));
  }

  private static List<Stmt> parseIgnoringProblems(String source) {
    return new Parser(new Lexer(new SourceFile("test", source))).parseFile();
  }

  // Checks a program that must lex and parse cleanly.
  private static List<Diagnostic> check(String source) {
    var lexer = new Lexer(new SourceFile("test", source));
    var parser = new Parser(lexer);
    var statements = parser.parseFile();
    assertEquals(List.of(), lexer.diagnostics(), "lexer diagnostics");
    assertEquals(List.of(), parser.diagnostics(), "parser diagnostics");
    return Checker.check(statements);
  }

  private static List<String> messages(List<Diagnostic> diagnostics) {
    return diagnostics.stream().map(Diagnostic::message).toList();
  }

  private static List<String> texts(String source, List<Diagnostic> diagnostics) {
    return diagnostics.stream().map(d -> source.substring(d.start(), d.end())).toList();
  }
}
