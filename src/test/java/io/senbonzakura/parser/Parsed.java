package io.senbonzakura.parser;

import static org.junit.jupiter.api.Assertions.assertEquals;
import static org.junit.jupiter.api.Assertions.assertInstanceOf;
import static org.junit.jupiter.api.Assertions.assertTrue;

import io.senbonzakura.ast.Expr;
import io.senbonzakura.lexer.Lexer;
import io.senbonzakura.lexer.SourceFile;
import io.senbonzakura.lexer.Token.Diagnostic;
import java.util.List;

// Parses a whole input as one expression, and keeps everything the tests look at.
record Parsed(
    SourceFile source,
    Expr expr,
    List<Diagnostic> lexerDiagnostics,
    List<Diagnostic> parserDiagnostics) {

  static Parsed parse(String text) {
    return parse(text, true);
  }

  // Parses the first expression and ignores anything left after it.
  static Parsed parseFirst(String text) {
    return parse(text, false);
  }

  private static Parsed parse(String text, boolean wholeInput) {
    var source = new SourceFile("test", text);
    var lexer = new Lexer(source);
    var parser = new Parser(lexer);
    var expr = parser.parseExpression();
    if (wholeInput) {
      assertTrue(parser.atEnd(), () -> "input left after the expression in: " + text);
    }
    return new Parsed(
        source, expr, List.copyOf(lexer.diagnostics()), List.copyOf(parser.diagnostics()));
  }

  static Parsed parseClean(String text) {
    var parsed = parse(text);
    assertEquals(List.of(), parsed.lexerDiagnostics(), "lexer diagnostics");
    assertEquals(List.of(), parsed.parserDiagnostics(), "parser diagnostics");
    return parsed;
  }

  <T extends Expr> T as(Class<T> type) {
    return assertInstanceOf(type, expr);
  }

  String exprText() {
    return text(expr);
  }

  String text(Expr node) {
    return source.text().substring(node.start(), node.end());
  }

  List<String> parserMessages() {
    return parserDiagnostics.stream().map(Diagnostic::message).toList();
  }

  // The source text each parser diagnostic points at.
  List<String> parserTexts() {
    return parserDiagnostics.stream()
        .map(d -> source.text().substring(d.start(), d.end()))
        .toList();
  }
}
