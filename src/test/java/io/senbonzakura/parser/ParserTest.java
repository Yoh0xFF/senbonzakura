package io.senbonzakura.parser;

import static io.senbonzakura.parser.Parsed.parse;
import static org.junit.jupiter.api.Assertions.assertEquals;
import static org.junit.jupiter.api.Assertions.assertTrue;

import io.senbonzakura.ast.Expr;
import io.senbonzakura.ast.Sexpr;
import io.senbonzakura.lexer.Lexer;
import io.senbonzakura.lexer.SourceFile;
import java.util.List;
import org.junit.jupiter.api.Test;

class ParserTest {

  @Test
  void parseFileReturnsEveryStatementInOrder() {
    var parser = new Parser(new Lexer(new SourceFile("test", "var x = 1;\nfn f() {}\nf(x);")));

    var statements = parser.parseFile();

    assertEquals(
        List.of("(var x 1)", "(fn f () {})", "(call f x);"),
        statements.stream().map(Sexpr::print).toList());
    assertTrue(parser.diagnostics().isEmpty());
    assertTrue(parser.atEnd());
  }

  @Test
  void parseFileOfAnEmptyOrCommentOnlyFileIsEmpty() {
    assertEquals(List.of(), new Parser(new Lexer(new SourceFile("test", ""))).parseFile());
    assertEquals(
        List.of(), new Parser(new Lexer(new SourceFile("test", "// x\n/* y */"))).parseFile());
  }

  @Test
  void parseFileKeepsGoingAfterProblems() {
    var parser = new Parser(new Lexer(new SourceFile("test", ") x = ; } y;")));

    var statements = parser.parseFile();

    assertEquals("y;", Sexpr.print(statements.getLast()));
    assertEquals(
        List.of(
            "expected expression, found ')'",
            "expected expression, found ';'",
            "expected expression, found '}'"),
        parser.diagnostics().stream().map(d -> d.message()).toList());
  }

  @Test
  void nonLiteralIsReportedAndBecomesAnErrorNode() {
    var parsed = parse(":");

    assertEquals(new Expr.Error(0, 1), parsed.expr());
    assertEquals(List.of("expected expression, found ':'"), parsed.parserMessages());
  }

  @Test
  void emptyInputIsReportedAtTheEnd() {
    var parsed = parse("  ");

    assertEquals(new Expr.Error(2, 2), parsed.expr());
    assertEquals(List.of("expected expression, found end of input"), parsed.parserMessages());
  }

  @Test
  void unknownCharacterIsLeftToTheLexer() {
    var parsed = parse("#");

    assertEquals(new Expr.Error(0, 1), parsed.expr());
    assertEquals(
        List.of("unexpected character"),
        parsed.lexerDiagnostics().stream().map(d -> d.message()).toList());
    assertEquals(List.of(), parsed.parserDiagnostics());
  }

  @Test
  void strayCloseParenIsReportedAndSkipped() {
    var parsed = parse(")");

    assertEquals(new Expr.Error(0, 0), parsed.expr());
    assertEquals(List.of("expected expression, found ')'"), parsed.parserMessages());
  }

  @Test
  void parsesOneExpressionPerCall() {
    var parser = new Parser(new Lexer(new SourceFile("test", "1 \"two\" 3.0 true")));

    var exprs =
        List.of(
            parser.parseExpression(),
            parser.parseExpression(),
            parser.parseExpression(),
            parser.parseExpression());

    assertTrue(parser.atEnd());
    assertEquals(
        List.of(
            new Expr.IntLiteral(1, 0, 1),
            new Expr.StringLiteral("two", 2, 7),
            new Expr.FloatLiteral(3.0, 8, 11),
            new Expr.BoolLiteral(true, 12, 16)),
        exprs);
  }
}
