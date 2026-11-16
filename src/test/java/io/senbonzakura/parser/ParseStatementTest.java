package io.senbonzakura.parser;

import static io.senbonzakura.parser.ParsedStmt.parse;
import static io.senbonzakura.parser.ParsedStmt.parseClean;
import static org.junit.jupiter.api.Assertions.assertEquals;
import static org.junit.jupiter.api.Assertions.assertTrue;

import io.senbonzakura.ast.Stmt;
import io.senbonzakura.lexer.Lexer;
import io.senbonzakura.lexer.SourceFile;
import io.senbonzakura.lexer.Token.Diagnostic;
import java.util.ArrayList;
import java.util.List;
import org.junit.jupiter.api.Test;
import org.junit.jupiter.params.ParameterizedTest;
import org.junit.jupiter.params.provider.CsvSource;

class ParseStatementTest {

  @ParameterizedTest
  @CsvSource(
      delimiterString = "=>",
      value = {
        "1;              => 1;",
        "x = 1 + 2;      => (= x (+ 1 2));",
        "a..b;           => (.. a b);",
        "(x);            => (group x);",
      })
  void expressionStatements(String input, String tree) {
    assertEquals(tree, parseClean(input).tree());
  }

  @ParameterizedTest
  @CsvSource(
      delimiterString = "=>",
      value = {
        "{}                      => {}",
        "{ x; }                  => {x;}",
        "{ x = 1; y += 2; }      => {(= x 1); (+= y 2);}",
        "{ a; { b; } c; }        => {a; {b;} c;}",
        "{{}}                    => {{}}",
      })
  void blocks(String input, String tree) {
    assertEquals(tree, parseClean(input).tree());
  }

  @Test
  void statementsCoverTheirSource() {
    var parsed = parseClean("{ x = 1; { y; } }");
    var outer = parsed.as(Stmt.Block.class);

    assertEquals("{ x = 1; { y; } }", parsed.text(outer));
    assertEquals("x = 1;", parsed.text(outer.statements().get(0)));
    assertEquals("{ y; }", parsed.text(outer.statements().get(1)));
  }

  @ParameterizedTest
  @CsvSource(
      delimiterString = "=>",
      value = {
        "x            => x;              => 1",
        "x = 1 + 2    => (= x (+ 1 2));  => 9",
        "{ x }        => {x;}            => 3",
        "{ a b; }     => {a; b;}         => 3",
      })
  void missingSemicolonIsReportedAfterTheExpression(String input, String tree, int offset) {
    var parsed = parse(input);

    assertEquals(tree, parsed.tree());
    assertEquals(
        List.of(new Diagnostic(offset, offset, "expected ';' after expression")),
        parsed.parserDiagnostics());
  }

  @ParameterizedTest
  @CsvSource(
      delimiterString = "=>",
      value = {
        "x = ;        => (= x <error>);  => expected expression, found ';'",
        "1 +          => (+ 1 <error>);  => expected expression, found end of input",
        ";            => <error>;        => expected expression, found ';'",
        "{ 1 + }      => {(+ 1 <error>);} => expected expression, found '}'",
      })
  void missingSemicolonIsNotReportedAfterABrokenExpression(
      String input, String tree, String message) {
    var parsed = parse(input);

    assertEquals(tree, parsed.tree());
    assertEquals(List.of(message), parsed.parserMessages());
  }

  @Test
  void unclosedBlockIsReported() {
    var parsed = parse("{ x;");

    assertEquals("{x;}", parsed.tree());
    assertEquals(List.of("expected '}', found end of input"), parsed.parserMessages());
    assertEquals("{ x;", parsed.text(parsed.stmt()));
  }

  @Test
  void unclosedEmptyBlockEndsAtItsBrace() {
    var parsed = parse("{");

    assertEquals("{}", parsed.tree());
    assertEquals("{", parsed.text(parsed.stmt()));
  }

  @Test
  void strayCloserInsideABlockIsReportedAndSkipped() {
    var parsed = parse("{ ) x; }");

    assertEquals("{<error>; x;}", parsed.tree());
    assertEquals(List.of("expected expression, found ')'"), parsed.parserMessages());
  }

  @Test
  void strayCloseBraceIsReportedAndSkipped() {
    var parsed = parse("}");

    assertEquals(List.of("expected expression, found '}'"), parsed.parserMessages());
  }

  @Test
  void parsesOneStatementPerCall() {
    var parser = new Parser(new Lexer(new SourceFile("test", "a; { b; }\nc = 1;")));

    var trees = new ArrayList<String>();
    while (!parser.atEnd()) {
      trees.add(Sexpr.print(parser.parseStatement()));
    }

    assertEquals(List.of("a;", "{b;}", "(= c 1);"), trees);
    assertTrue(parser.diagnostics().isEmpty());
  }

  @ParameterizedTest
  @CsvSource(
      delimiterString = "=>",
      value = {
        "let x = 1;              => (let x 1)",
        "let mut x = 1;          => (let mut x 1)",
        "let x;                  => (let x)",
        "let mut x;              => (let mut x)",
        "let x = a + f(b);       => (let x (+ a (call f b)))",
        "let r = 0..n;           => (let r (.. 0 n))",
        "{ let x = 1; x += 2; }  => {(let x 1) (+= x 2);}",
      })
  void letStatements(String input, String tree) {
    assertEquals(tree, parseClean(input).tree());
  }

  @Test
  void letCoversItsSource() {
    var parsed = parseClean("let mut x = 1 + 2;");
    var let = parsed.as(Stmt.Let.class);

    assertTrue(let.mutable());
    assertEquals("x", let.name());
    assertEquals("let mut x = 1 + 2;", parsed.text(let));
  }

  @ParameterizedTest
  @CsvSource(
      delimiterString = "=>",
      value = {
        "let = 1;      => (let <error> 1)      => expected variable name, found '='",
        "let;          => (let <error>)        => expected variable name, found ';'",
        "let mut;      => (let mut <error>)    => expected variable name, found ';'",
        "let fn = 1;   => (let <error> 1)      => expected variable name, found 'fn'",
        "let 1 = 2;    => (let <error> 2)      => expected variable name, found '1'",
        "let x = ;     => (let x <error>)      => expected expression, found ';'",
      })
  void malformedLetIsReportedOnce(String input, String tree, String message) {
    var parsed = parse(input);

    assertEquals(tree, parsed.tree());
    assertEquals(List.of(message), parsed.parserMessages());
  }

  @ParameterizedTest
  @CsvSource(
      delimiterString = "=>",
      value = {
        "let x = 1     => (let x 1)      => 9",
        "let x         => (let x)        => 5",
        "{ let x 1; }  => {(let x) 1;}   => 7",
      })
  void missingSemicolonAfterLetIsReported(String input, String tree, int offset) {
    var parsed = parse(input);

    assertEquals(tree, parsed.tree());
    assertEquals(
        List.of(new Diagnostic(offset, offset, "expected ';' after let statement")),
        parsed.parserDiagnostics());
  }

  @Test
  void letIsNotAnExpression() {
    var parsed = parse("{ x = let y; }");

    assertEquals(List.of("expected expression, found 'let'"), parsed.parserMessages());
  }
}
