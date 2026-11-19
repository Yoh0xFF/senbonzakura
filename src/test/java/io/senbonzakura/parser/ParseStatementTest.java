package io.senbonzakura.parser;

import static io.senbonzakura.parser.ParsedStmt.parse;
import static io.senbonzakura.parser.ParsedStmt.parseClean;
import static org.junit.jupiter.api.Assertions.assertEquals;
import static org.junit.jupiter.api.Assertions.assertInstanceOf;
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

  @ParameterizedTest
  @CsvSource(
      delimiterString = "=>",
      value = {
        "if a { x; }                     => (if a {x;})",
        "if a {}                         => (if a {})",
        "if a == b || c == d { x = 1; }  => (if (|| (== a b) (== c d)) {(= x 1);})",
        "if f(x) && !y.z { }             => (if (&& (call f x) (! (. y z))) {})",
        "if (a) { b; }                   => (if (group a) {b;})",
        "if a { if b { c; } }            => (if a {(if b {c;})})",
        "{ if a { b; } c; }              => {(if a {b;}) c;}",
      })
  void ifStatements(String input, String tree) {
    assertEquals(tree, parseClean(input).tree());
  }

  @Test
  void ifCoversItsSource() {
    var parsed = parseClean("if a < b { x; }");
    var ifStmt = parsed.as(Stmt.If.class);

    assertEquals("if a < b { x; }", parsed.text(ifStmt));
    assertEquals("a < b", parsed.text(ifStmt.condition()));
    assertEquals("{ x; }", parsed.text(ifStmt.then()));
  }

  @Test
  void ifNeedsNoSemicolon() {
    var all = parseAll("if a { b; } c;");

    assertEquals(List.of("(if a {b;})", "c;"), all.trees());
    assertEquals(List.of(), all.messages());
  }

  @ParameterizedTest
  @CsvSource(
      delimiterString = "=>",
      value = {
        "if { x; }     => (if <error> {x;})        => expected expression, found '{'",
        "if a + { }    => (if (+ a <error>) {})    => expected expression, found '{'",
        "if a +        => (if (+ a <error>) {})    => expected expression, found end of input",
        "if a          => (if a {})                => expected '{' after if condition, found end of input",
        "if a { x;     => (if a {x;})              => expected '}', found end of input",
      })
  void malformedIfIsReportedOnce(String input, String tree, String message) {
    var parsed = parse(input);

    assertEquals(tree, parsed.tree());
    assertEquals(List.of(message), parsed.parserMessages());
  }

  @Test
  void missingBodyLeavesTheRestForTheNextStatement() {
    var all = parseAll("if a b; c;");

    assertEquals(List.of("(if a {})", "b;", "c;"), all.trees());
    assertEquals(List.of("expected '{' after if condition, found 'b'"), all.messages());
  }

  @Test
  void missingBodyIsAnEmptyBlockAtTheEndOfTheCondition() {
    var parsed = parse("if a ");
    var ifStmt = parsed.as(Stmt.If.class);

    assertEquals("if a", parsed.text(ifStmt));
    assertEquals(4, ifStmt.then().start());
    assertEquals(4, ifStmt.then().end());
  }

  @ParameterizedTest
  @CsvSource(
      delimiterString = "=>",
      value = {
        "if a { x; } else { y; }         => (if a {x;} else {y;})",
        "if a {} else {}                 => (if a {} else {})",
        "if a {} else if b { x; }        => (if a {} else (if b {x;}))",
        "if a {} else if b {} else {}    => (if a {} else (if b {} else {}))",
        "if a { if b {} else {} }        => (if a {(if b {} else {})})",
        "{ if a {} else {} b; }          => {(if a {} else {}) b;}",
      })
  void ifElseStatements(String input, String tree) {
    assertEquals(tree, parseClean(input).tree());
  }

  @Test
  void ifElseCoversItsSource() {
    var parsed = parseClean("if a { x; } else if b { y; } else { z; }");
    var ifStmt = parsed.as(Stmt.If.class);
    var elseIf = assertInstanceOf(Stmt.If.class, ifStmt.otherwise());

    assertEquals("if a { x; } else if b { y; } else { z; }", parsed.text(ifStmt));
    assertEquals("if b { y; } else { z; }", parsed.text(elseIf));
    assertEquals("{ z; }", parsed.text(elseIf.otherwise()));
  }

  @ParameterizedTest
  @CsvSource(
      delimiterString = "=>",
      value = {
        "if a {} else        => (if a {} else {})          => expected '{' or 'if' after 'else', found end of input",
        "if a else {}        => (if a {} else {})          => expected '{' after if condition, found 'else'",
        "if a {} else if {}  => (if a {} else (if <error> {})) => expected expression, found '{'",
        "if a {} else { x;   => (if a {} else {x;})        => expected '}', found end of input",
      })
  void malformedElseIsReportedOnce(String input, String tree, String message) {
    var parsed = parse(input);

    assertEquals(tree, parsed.tree());
    assertEquals(List.of(message), parsed.parserMessages());
  }

  @Test
  void missingElseBodyLeavesTheRestForTheNextStatement() {
    var all = parseAll("if a {} else b; c;");

    assertEquals(List.of("(if a {} else {})", "b;", "c;"), all.trees());
    assertEquals(List.of("expected '{' or 'if' after 'else', found 'b'"), all.messages());
  }

  @Test
  void missingElseBodyIsAnEmptyBlockAfterTheElse() {
    var parsed = parse("if a {} else ");
    var ifStmt = parsed.as(Stmt.If.class);

    assertEquals("if a {} else", parsed.text(ifStmt));
    assertEquals(12, ifStmt.otherwise().start());
    assertEquals(12, ifStmt.otherwise().end());
  }

  @Test
  void elseWithoutIfIsReported() {
    var all = parseAll("else { x; }");

    assertEquals(List.of("expected expression, found 'else'"), all.messages());
  }

  @ParameterizedTest
  @CsvSource(
      delimiterString = "=>",
      value = {
        "loop {}                                 => (loop {})",
        "loop { x; }                             => (loop {x;})",
        "loop a == b { x; }                      => (loop (== a b) {x;})",
        "loop !done && n < 10 {}                 => (loop (&& (! done) (< n 10)) {})",
        "loop let i in 0..10 { x; }              => (loop let i in (.. 0 10) {x;})",
        "loop let mut c in s.chars() {}          => (loop let mut c in (call (. s chars)) {})",
        "loop let i = 0; i < 10; i++ { x; }      => (loop (let i 0) (< i 10) (++ i) {x;})",
        "loop let mut i = 9; i >= 0; i-- {}      => (loop (let mut i 9) (>= i 0) (-- i) {})",
        "loop let i = 0; i < n; i += 2 {}        => (loop (let i 0) (< i n) (+= i 2) {})",
        "{ loop a { b; } c; }                    => {(loop a {b;}) c;}",
        "loop a { loop let x in xs { if x {} } } => (loop a {(loop let x in xs {(if x {})})})",
      })
  void loopStatements(String input, String tree) {
    assertEquals(tree, parseClean(input).tree());
  }

  @Test
  void loopCoversItsSource() {
    var parsed = parseClean("loop a < b { x; }");
    var loop = parsed.as(Stmt.Loop.class);

    assertEquals("loop a < b { x; }", parsed.text(loop));
    assertEquals("a < b", parsed.text(loop.condition()));
    assertEquals("{ x; }", parsed.text(loop.body()));
  }

  @Test
  void loopInCoversItsSource() {
    var parsed = parseClean("loop let mut i in 0..n { x; }");
    var loop = parsed.as(Stmt.LoopIn.class);

    assertTrue(loop.mutable());
    assertEquals("i", loop.name());
    assertEquals("loop let mut i in 0..n { x; }", parsed.text(loop));
    assertEquals("0..n", parsed.text(loop.iterable()));
  }

  @Test
  void loopStepCoversItsSource() {
    var parsed = parseClean("loop let i = 0; i < 10; i++ { x; }");
    var loop = parsed.as(Stmt.LoopStep.class);

    assertEquals("loop let i = 0; i < 10; i++ { x; }", parsed.text(loop));
    assertEquals("let i = 0;", parsed.text(loop.init()));
    assertEquals("i < 10", parsed.text(loop.condition()));
    assertEquals("i++", parsed.text(loop.step()));
    assertEquals("{ x; }", parsed.text(loop.body()));
  }

  @Test
  void loopNeedsNoSemicolon() {
    var all = parseAll("loop a { b; } loop {} c;");

    assertEquals(List.of("(loop a {b;})", "(loop {})", "c;"), all.trees());
    assertEquals(List.of(), all.messages());
  }

  @ParameterizedTest
  @CsvSource(
      delimiterString = "=>",
      value = {
        "loop a                       => (loop a {})                               => expected '{' after loop condition, found end of input",
        "loop a +                     => (loop (+ a <error>) {})                   => expected expression, found end of input",
        "loop { x;                    => (loop {x;})                               => expected '}', found end of input",
        "loop let in xs {}            => (loop let <error> in xs {})               => expected variable name, found 'in'",
        "loop let i {}                => (loop let i in <error> {})                => expected 'in' or '=' after loop variable, found '{'",
        "loop let i xs {}             => (loop let i in xs {})                     => expected 'in' or '=' after loop variable, found 'xs'",
        "loop let i in {}             => (loop let i in <error> {})                => expected expression, found '{'",
        "loop let i in xs             => (loop let i in xs {})                     => expected '{' after loop iterable, found end of input",
        "loop let i = 0 i < 9; i++ {} => (loop (let i 0) (< i 9) (++ i) {})       => expected ';' after let statement",
        "loop let i = 0; i < 9 i++ {} => (loop (let i 0) (< i 9) (++ i) {})       => expected ';' after loop condition",
        "loop let i = 0; i < 9; {}    => (loop (let i 0) (< i 9) <error> {})       => expected expression, found '{'",
        "loop let i = 0; i < 9; i++   => (loop (let i 0) (< i 9) (++ i) {})       => expected '{' after loop step, found end of input",
      })
  void malformedLoopIsReportedOnce(String input, String tree, String message) {
    var parsed = parse(input);

    assertEquals(tree, parsed.tree());
    assertEquals(List.of(message), parsed.parserMessages());
  }

  @Test
  void missingLoopBodyLeavesTheRestForTheNextStatement() {
    var all = parseAll("loop a b; c;");

    assertEquals(List.of("(loop a {})", "b;", "c;"), all.trees());
    assertEquals(List.of("expected '{' after loop condition, found 'b'"), all.messages());
  }

  @Test
  void plainLetSkipsAnInWhereTheNameShouldBe() {
    var parsed = parse("let in = 1;");

    assertEquals("(let <error> 1)", parsed.tree());
    assertEquals(List.of("expected variable name, found 'in'"), parsed.parserMessages());
  }

  @ParameterizedTest
  @CsvSource(
      delimiterString = "=>",
      value = {
        "continue;                      => (continue)",
        "continue if a == b;            => (continue if (== a b))",
        "continue if i >= 10;           => (continue if (>= i 10))",
        "continue if (a > b && c < d);  => (continue if (group (&& (> a b) (< c d))))",
        "continue if !f(x);             => (continue if (! (call f x)))",
        "loop { continue if done; x; }  => (loop {(continue if done) x;})",
      })
  void continueStatements(String input, String tree) {
    assertEquals(tree, parseClean(input).tree());
  }

  @Test
  void continueCoversItsSource() {
    var parsed = parseClean("continue if a < b;");
    var stmt = parsed.as(Stmt.Continue.class);

    assertEquals("continue if a < b;", parsed.text(stmt));
    assertEquals("a < b", parsed.text(stmt.condition()));
  }

  @ParameterizedTest
  @CsvSource(
      delimiterString = "=>",
      value = {
        "continue        => (continue)               => 8  => expected ';' or 'if' after 'continue'",
        "continue if a   => (continue if a)          => 13 => expected ';' after continue condition",
        "{ continue }    => {(continue)}             => 10 => expected ';' or 'if' after 'continue'",
      })
  void missingSemicolonAfterContinueIsReported(
      String input, String tree, int offset, String message) {
    var parsed = parse(input);

    assertEquals(tree, parsed.tree());
    assertEquals(List.of(new Diagnostic(offset, offset, message)), parsed.parserDiagnostics());
  }

  @Test
  void missingConditionIsReportedOnce() {
    var parsed = parse("continue if ;");

    assertEquals("(continue if <error>)", parsed.tree());
    assertEquals(List.of("expected expression, found ';'"), parsed.parserMessages());
  }

  @Test
  void continueLeavesWhatFollowsForTheNextStatement() {
    var all = parseAll("continue x; y;");

    assertEquals(List.of("(continue)", "x;", "y;"), all.trees());
    assertEquals(List.of("expected ';' or 'if' after 'continue'"), all.messages());
  }

  @Test
  void continueIsNotAnExpression() {
    var parsed = parse("{ x = continue; }");

    assertEquals(List.of("expected expression, found 'continue'"), parsed.parserMessages());
  }

  private record ParsedAll(List<String> trees, List<String> messages) {}

  private static ParsedAll parseAll(String text) {
    var parser = new Parser(new Lexer(new SourceFile("test", text)));
    var trees = new ArrayList<String>();
    while (!parser.atEnd()) {
      trees.add(Sexpr.print(parser.parseStatement()));
    }
    return new ParsedAll(trees, parser.diagnostics().stream().map(Diagnostic::message).toList());
  }
}
