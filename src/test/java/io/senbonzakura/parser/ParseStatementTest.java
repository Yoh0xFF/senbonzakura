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
import org.junit.jupiter.params.provider.ValueSource;

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
        "var x = 1;              => (var x 1)",
        "var x;                  => (var x)",
        "var mut = 1;            => (var mut 1)",
        "var while = 0;          => (var while 0)",
        "var for;                => (var for)",
        "var let = 1;            => (var let 1)",
        "var x = a + f(b);       => (var x (+ a (call f b)))",
        "var r = 0..n;           => (var r (.. 0 n))",
        "{ var x = 1; x += 2; }  => {(var x 1) (+= x 2);}",
      })
  void varStatements(String input, String tree) {
    assertEquals(tree, parseClean(input).tree());
  }

  @Test
  void varCoversItsSource() {
    var parsed = parseClean("var x = 1 + 2;");
    var decl = parsed.as(Stmt.Var.class);

    assertEquals("x", decl.name());
    assertEquals("var x = 1 + 2;", parsed.text(decl));
  }

  @ParameterizedTest
  @CsvSource(
      delimiterString = "=>",
      value = {
        "var = 1;      => (var <error> 1)      => expected variable name, found '='",
        "var;          => (var <error>)        => expected variable name, found ';'",
        "var fn = 1;   => (var <error> 1)      => expected variable name, found 'fn'",
        "var 1 = 2;    => (var <error> 2)      => expected variable name, found '1'",
        "var x = ;     => (var x <error>)      => expected expression, found ';'",
      })
  void malformedVarIsReportedOnce(String input, String tree, String message) {
    var parsed = parse(input);

    assertEquals(tree, parsed.tree());
    assertEquals(List.of(message), parsed.parserMessages());
  }

  @ParameterizedTest
  @CsvSource(
      delimiterString = "=>",
      value = {
        "var x = 1     => (var x 1)      => 9",
        "var x         => (var x)        => 5",
        "{ var x 1; }  => {(var x) 1;}   => 7",
      })
  void missingSemicolonAfterVarIsReported(String input, String tree, int offset) {
    var parsed = parse(input);

    assertEquals(tree, parsed.tree());
    assertEquals(
        List.of(new Diagnostic(offset, offset, "expected ';' after var statement")),
        parsed.parserDiagnostics());
  }

  @Test
  void varIsNotAnExpression() {
    var parsed = parse("{ x = var y; }");

    assertEquals(List.of("expected expression, found 'var'"), parsed.parserMessages());
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
        "loop var i in 0..10 { x; }              => (loop var i in (.. 0 10) {x;})",
        "loop var c in s.chars() {}              => (loop var c in (call (. s chars)) {})",
        "loop var i = 0; i < 10; i++ { x; }      => (loop (var i 0) (< i 10) (++ i) {x;})",
        "loop var i = 9; i >= 0; i-- {}          => (loop (var i 9) (>= i 0) (-- i) {})",
        "loop var i = 0; i < n; i += 2 {}        => (loop (var i 0) (< i n) (+= i 2) {})",
        "{ loop a { b; } c; }                    => {(loop a {b;}) c;}",
        "loop a { loop var x in xs { if x {} } } => (loop a {(loop var x in xs {(if x {})})})",
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
    var parsed = parseClean("loop var i in 0..n { x; }");
    var loop = parsed.as(Stmt.LoopIn.class);

    assertEquals("i", loop.name());
    assertEquals("loop var i in 0..n { x; }", parsed.text(loop));
    assertEquals("0..n", parsed.text(loop.iterable()));
  }

  @Test
  void loopStepCoversItsSource() {
    var parsed = parseClean("loop var i = 0; i < 10; i++ { x; }");
    var loop = parsed.as(Stmt.LoopStep.class);

    assertEquals("loop var i = 0; i < 10; i++ { x; }", parsed.text(loop));
    assertEquals("var i = 0;", parsed.text(loop.init()));
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
        "loop var in xs {}            => (loop var <error> in xs {})               => expected variable name, found 'in'",
        "loop var i {}                => (loop var i in <error> {})                => expected 'in' or '=' after loop variable, found '{'",
        "loop var i xs {}             => (loop var i in xs {})                     => expected 'in' or '=' after loop variable, found 'xs'",
        "loop var i in {}             => (loop var i in <error> {})                => expected expression, found '{'",
        "loop var i in xs             => (loop var i in xs {})                     => expected '{' after loop iterable, found end of input",
        "loop var i = 0 i < 9; i++ {} => (loop (var i 0) (< i 9) (++ i) {})       => expected ';' after var statement",
        "loop var i = 0; i < 9 i++ {} => (loop (var i 0) (< i 9) (++ i) {})       => expected ';' after loop condition",
        "loop var i = 0; i < 9; {}    => (loop (var i 0) (< i 9) <error> {})       => expected expression, found '{'",
        "loop var i = 0; i < 9; i++   => (loop (var i 0) (< i 9) (++ i) {})       => expected '{' after loop step, found end of input",
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
  void plainVarSkipsAnInWhereTheNameShouldBe() {
    var parsed = parse("var in = 1;");

    assertEquals("(var <error> 1)", parsed.tree());
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
        "break;                         => (break)",
        "break if a == b;               => (break if (== a b))",
        "break if i >= 10;              => (break if (>= i 10))",
        "break if (a > b && c < d);     => (break if (group (&& (> a b) (< c d))))",
        "loop { continue if a; break if b; x; } => (loop {(continue if a) (break if b) x;})",
      })
  void continueAndBreakStatements(String input, String tree) {
    assertEquals(tree, parseClean(input).tree());
  }

  @Test
  void continueCoversItsSource() {
    var parsed = parseClean("continue if a < b;");
    var stmt = parsed.as(Stmt.Continue.class);

    assertEquals("continue if a < b;", parsed.text(stmt));
    assertEquals("a < b", parsed.text(stmt.condition()));
  }

  @Test
  void breakCoversItsSource() {
    var parsed = parseClean("break if a < b;");
    var stmt = parsed.as(Stmt.Break.class);

    assertEquals("break if a < b;", parsed.text(stmt));
    assertEquals("a < b", parsed.text(stmt.condition()));
  }

  @ParameterizedTest
  @CsvSource(
      delimiterString = "=>",
      value = {
        "continue        => (continue)               => 8  => expected ';' or 'if' after 'continue'",
        "continue if a   => (continue if a)          => 13 => expected ';' after continue condition",
        "{ continue }    => {(continue)}             => 10 => expected ';' or 'if' after 'continue'",
        "break           => (break)                  => 5  => expected ';' or 'if' after 'break'",
        "break if a      => (break if a)             => 10 => expected ';' after break condition",
        "{ break }       => {(break)}                => 7  => expected ';' or 'if' after 'break'",
      })
  void missingSemicolonAfterContinueOrBreakIsReported(
      String input, String tree, int offset, String message) {
    var parsed = parse(input);

    assertEquals(tree, parsed.tree());
    assertEquals(List.of(new Diagnostic(offset, offset, message)), parsed.parserDiagnostics());
  }

  @ParameterizedTest
  @ValueSource(strings = {"continue", "break", "return"})
  void missingConditionIsReportedOnce(String keyword) {
    var parsed = parse(keyword + " if ;");

    assertEquals("(" + keyword + " if <error>)", parsed.tree());
    assertEquals(List.of("expected expression, found ';'"), parsed.parserMessages());
  }

  @ParameterizedTest
  @ValueSource(strings = {"continue", "break"})
  void continueOrBreakLeavesWhatFollowsForTheNextStatement(String keyword) {
    var all = parseAll(keyword + " x; y;");

    assertEquals(List.of("(" + keyword + ")", "x;", "y;"), all.trees());
    assertEquals(List.of("expected ';' or 'if' after '" + keyword + "'"), all.messages());
  }

  @ParameterizedTest
  @ValueSource(strings = {"continue", "break", "return"})
  void continueBreakAndReturnAreNotExpressions(String keyword) {
    var parsed = parse("{ x = " + keyword + "; }");

    assertEquals(List.of("expected expression, found '" + keyword + "'"), parsed.parserMessages());
  }

  @ParameterizedTest
  @CsvSource(
      delimiterString = "=>",
      value = {
        "return;                        => (return)",
        "return if a == b;              => (return if (== a b))",
        "return if i >= 10;             => (return if (>= i 10))",
        "return if (a > b && c < d);    => (return if (group (&& (> a b) (< c d))))",
        "return x;                      => (return x)",
        "return x if a == b;            => (return x if (== a b))",
        "return x if i >= 10;           => (return x if (>= i 10))",
        "return x if (a > b && c < d);  => (return x if (group (&& (> a b) (< c d))))",
        "return a + f(b);               => (return (+ a (call f b)))",
        "return (x) if y;               => (return (group x) if y)",
        "return a == b if c;            => (return (== a b) if c)",
        "{ return x if a; y; }          => {(return x if a) y;}",
      })
  void returnStatements(String input, String tree) {
    assertEquals(tree, parseClean(input).tree());
  }

  @Test
  void returnCoversItsSource() {
    var parsed = parseClean("return x + 1 if a < b;");
    var stmt = parsed.as(Stmt.Return.class);

    assertEquals("return x + 1 if a < b;", parsed.text(stmt));
    assertEquals("x + 1", parsed.text(stmt.value()));
    assertEquals("a < b", parsed.text(stmt.condition()));
  }

  @ParameterizedTest
  @CsvSource(
      delimiterString = "=>",
      value = {
        "return          => (return)                 => 6  => expected ';' or 'if' after 'return'",
        "{ return }      => {(return)}               => 8  => expected ';' or 'if' after 'return'",
        "return x        => (return x)               => 8  => expected ';' or 'if' after return value",
        "return if a     => (return if a)            => 11 => expected ';' after return condition",
        "return x if a   => (return x if a)          => 13 => expected ';' after return condition",
      })
  void missingSemicolonAfterReturnIsReported(
      String input, String tree, int offset, String message) {
    var parsed = parse(input);

    assertEquals(tree, parsed.tree());
    assertEquals(List.of(new Diagnostic(offset, offset, message)), parsed.parserDiagnostics());
  }

  @ParameterizedTest
  @CsvSource(
      delimiterString = "=>",
      value = {
        "return x if ;   => (return x if <error>)    => expected expression, found ';'",
        "return x +;     => (return (+ x <error>))   => expected expression, found ';'",
        "return x + if a; => (return (+ x <error>) if a) => expected expression, found 'if'",
      })
  void malformedReturnIsReportedOnce(String input, String tree, String message) {
    var parsed = parse(input);

    assertEquals(tree, parsed.tree());
    assertEquals(List.of(message), parsed.parserMessages());
  }

  @Test
  void returnLeavesWhatFollowsItsValueForTheNextStatement() {
    var all = parseAll("return x y; z;");

    assertEquals(List.of("(return x)", "y;", "z;"), all.trees());
    assertEquals(List.of("expected ';' or 'if' after return value"), all.messages());
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
