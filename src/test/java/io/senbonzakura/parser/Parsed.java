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
    var source = new SourceFile("test", text);
    var lexer = new Lexer(source);
    var parser = new Parser(lexer);
    var expr = parser.parseExpression();
    assertTrue(parser.atEnd(), () -> "input left after the expression in: " + text);
    return new Parsed(
        source, expr, List.copyOf(lexer.diagnostics()), List.copyOf(parser.diagnostics()));
  }

  // Parses input that must have no lexer or parser problems.
  static Parsed parseClean(String text) {
    var parsed = parse(text);
    assertEquals(List.of(), parsed.lexerDiagnostics(), "lexer diagnostics");
    assertEquals(List.of(), parsed.parserDiagnostics(), "parser diagnostics");
    return parsed;
  }

  <T extends Expr> T as(Class<T> type) {
    return assertInstanceOf(type, expr);
  }

  // The source text the node covers.
  String exprText() {
    return source.text().substring(expr.start(), expr.end());
  }

  List<String> parserMessages() {
    return parserDiagnostics.stream().map(Diagnostic::message).toList();
  }
}
