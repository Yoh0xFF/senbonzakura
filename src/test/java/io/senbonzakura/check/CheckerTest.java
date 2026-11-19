package io.senbonzakura.check;

import static org.junit.jupiter.api.Assertions.assertEquals;

import io.senbonzakura.ast.Expr;
import io.senbonzakura.ast.Stmt;
import io.senbonzakura.ast.Type.PrimitiveKind;
import io.senbonzakura.lexer.Lexer;
import io.senbonzakura.lexer.SourceFile;
import io.senbonzakura.lexer.Token.Diagnostic;
import io.senbonzakura.parser.Parser;
import java.util.List;
import org.junit.jupiter.api.Test;
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
      })
  void callsTheCheckerCannotMatchAreLeftAlone(String source) {
    assertEquals(List.of(), messages(Checker.check(parseIgnoringProblems(source))));
  }

  @ParameterizedTest
  @ValueSource(
      strings = {
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
      })
  void wellTypedProgramsHaveNoProblems(String source) {
    assertEquals(List.of(), messages(check(source)));
  }

  @ParameterizedTest
  @CsvSource(
      quoteCharacter = '"',
      delimiterString = "=>",
      value = {
        "var x: int = true;                    => expected 'int', found 'bool'                     => true",
        "var i: int = 1.5;                     => expected 'int', found float constant             => 1.5",
        "var s: string = 'c';                  => expected 'string', found 'char'                  => 'c'",
        "var x: int = 1; var y: long = x;      => expected 'long', found 'int'                     => x",
        "var b: byte = 300;                    => 300 does not fit in 'byte'                       => 300",
        "var u: uint = -1;                     => -1 does not fit in 'uint'                        => -1",
        "var u: ubyte = (256);                 => 256 does not fit in 'ubyte'                      => (256)",
        "var x = 3_000_000_000;                => 3000000000 does not fit in 'int'                 => 3_000_000_000",
        "var b: byte = 1; var c = b + 200;     => 200 does not fit in 'byte'                       => 200",
        "var x;                                => 'x' needs a type or an initializer               => x",
        "fn g() {} var x = g();                => expected a value, found no value                 => g()",
        "fn g(): int, int { return 1, 2; } var x = g(); => expected a value, found 2 values        => g()",
        "fn g() {} print(g());                 => expected a value, found no value                 => g()",
        "if 1 {}                               => expected 'bool', found integer constant          => 1",
        "loop `forever` {}                     => expected 'bool', found 'string'                  => `forever`",
        "var x = 1; loop var i = 0; x; i++ {}  => expected 'bool', found 'int'                     => x",
        "loop { break if 1; }                  => expected 'bool', found integer constant          => 1",
        "fn f() { return if 2; }               => expected 'bool', found integer constant          => 2",
        "var a = 1 + true;                     => cannot apply '+' to integer constant and 'bool'  => 1 + true",
        "var s = `a` + 1;                      => cannot apply '+' to 'string' and integer constant => `a` + 1",
        "var s = `a` - `b`;                    => cannot apply '-' to 'string' and 'string'        => `a` - `b`",
        "var i: int = 1; var l: long = 2; var x = i * l; => cannot apply '*' to 'int' and 'long'   => i * l",
        "var x: int = 1; var y = x + 1.5;      => cannot apply '+' to 'int' and float constant     => x + 1.5",
        "var a = 1 < true;                     => cannot apply '<' to integer constant and 'bool'  => 1 < true",
        "var a = true < false;                 => cannot apply '<' to 'bool' and 'bool'            => true < false",
        "var a = true && 1;                    => cannot apply '&&' to 'bool' and integer constant => true && 1",
        "var b = !1;                           => cannot apply '!' to integer constant             => !1",
        "var u: uint = 1; var n = -u;          => cannot negate 'uint'                             => -u",
        "var r = 0..1.5;                       => range bounds must be integers, found float constant => 0..1.5",
        "var i: int = 1; var l: long = 2; var r = i..l; => range bounds have different types, 'int' and 'long' => i..l",
        "var n = 5; loop var x in n {}         => cannot loop over 'int'                           => n",
        "loop var x: bool in 0..3 {}           => the loop variable is 'bool', but the range holds 'int' => bool",
        "loop var x: long in 0..3 { var y: int = x; } => expected 'int', found 'long'               => x",
        "var x = 1; x = true;                  => expected 'int', found 'bool'                     => true",
        "var s = `a`; s -= `b`;                => cannot apply '-=' to 'string' and 'string'       => s -= `b`",
        "var b = true; b++;                    => cannot apply '++' to 'bool'                      => b++",
        "fn f(a: int) {} f(true);              => expected 'int', found 'bool'                     => true",
        "fn f(a: int, b: bool) {} f(b: 1, a: 2); => expected 'bool', found integer constant        => 1",
        "fn f(): int { return true; }          => expected 'int', found 'bool'                     => true",
        "fn f(): int, bool { return 1, 2; }    => expected 'bool', found integer constant          => 2",
        "fn f(): int { if true { return 1; } } => 'f' does not return on every path                => fn f(): int",
        "fn f(): int { loop { break; } }       => 'f' does not return on every path                => fn f(): int",
        "fn f(): int { loop { if true { break; } return 1; } } => 'f' does not return on every path => fn f(): int",
        "fn f(): int { return 1 if true; }     => 'f' does not return on every path                => fn f(): int",
        "fn f(x: int): int { loop var i in 0..x { return i; } } => 'f' does not return on every path => fn f(x: int): int",
        "var x = 1; x();                       => cannot call 'int'                                => x",
        "fn f(i: int) {} { var f = 1; f(); }   => cannot call 'int'                                => f",
        "fn f() {} var g = f;                  => function 'f' cannot be used as a value           => f",
        "var x = 1; x[0];                      => cannot index 'int'                               => x",
        "var x = 1; x.y;                       => 'int' has no field 'y'                           => x.y",
        "var c = `a` as int;                   => cannot convert 'string' to 'int'                 => `a` as int",
        "var b = true as int;                  => cannot convert 'bool' to 'int'                   => true as int",
        "var c = 1.5 as char;                  => cannot convert float constant to 'char'          => 1.5 as char",
        "print();                              => 'print' expects 1 argument, found 0              => print()",
        "print(1, 2);                          => 'print' expects 1 argument, found 2              => print(1, 2)",
      })
  void typeProblemIsReportedOnce(String source, String message, String at) {
    var diagnostics = check(source);

    assertEquals(List.of(message), messages(diagnostics));
    assertEquals(List.of(at), texts(source, diagnostics));
  }

  @ParameterizedTest
  @CsvSource(
      quoteCharacter = '"',
      delimiterString = "=>",
      value = {
        "var y: int = x + 1 * 2;           => 1",
        "var a = b..c;                     => 2",
        "print(-z);                        => 1",
        "loop var i in nope { i + 1; }     => 1",
        "var x = 1; x = nope; x += nope;   => 2",
        "f(x: g());                        => 2",
      })
  void undefinedNameIsReportedOncePerUseAndNotAgainByWhatUsesIt(String source, int uses) {
    var messages = messages(check(source));

    assertEquals(uses, messages.size(), messages::toString);
    assertEquals(
        uses,
        messages.stream().filter(m -> m.endsWith("is not defined")).count(),
        messages::toString);
  }

  @Test
  void literalsRecordTheTypeTheySettledInto() {
    var statements =
        parseIgnoringProblems(
            "var b: byte = 1 + 2; var d: double = 1; var c = 1 < 1.5; print(7);"
                + " loop var i: long in 0..3 {}");
    var analysis = Checker.analyze(statements);
    assertEquals(List.of(), analysis.diagnostics());

    var sum = (Expr.Binary) initializer(statements, 0);
    assertEquals(primitive(PrimitiveKind.BYTE), analysis.typeOf(sum));
    assertEquals(primitive(PrimitiveKind.BYTE), analysis.typeOf(sum.left()));
    assertEquals(primitive(PrimitiveKind.DOUBLE), analysis.typeOf(initializer(statements, 1)));

    var comparison = (Expr.Binary) initializer(statements, 2);
    assertEquals(Ty.BOOL, analysis.typeOf(comparison));
    assertEquals(Ty.Special.FLOAT_CONSTANT, analysis.typeOf(comparison.left()));

    var print = (Expr.Call) ((Stmt.Expression) statements.get(3)).expr();
    assertEquals(Ty.INT, analysis.typeOf(print.arguments().getFirst().value()));

    var range = (Expr.Range) ((Stmt.LoopIn) statements.get(4)).iterable();
    assertEquals(new Ty.Range(primitive(PrimitiveKind.LONG)), analysis.typeOf(range));
    assertEquals(primitive(PrimitiveKind.LONG), analysis.typeOf(range.upper()));
  }

  private static Expr initializer(List<Stmt> statements, int index) {
    return ((Stmt.Var) statements.get(index)).declarators().getFirst().initializer();
  }

  private static Ty primitive(PrimitiveKind kind) {
    return new Ty.Primitive(kind);
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
