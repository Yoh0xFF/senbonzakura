package io.senbonzakura.parser;

import static org.junit.jupiter.api.Assertions.assertEquals;
import static org.junit.jupiter.api.Assertions.assertInstanceOf;
import static org.junit.jupiter.api.Assertions.assertTrue;

import io.senbonzakura.ast.Expr;
import io.senbonzakura.ast.Sexpr;
import io.senbonzakura.ast.Stmt;
import io.senbonzakura.ast.Type;
import io.senbonzakura.lexer.Lexer;
import io.senbonzakura.lexer.SourceFile;
import io.senbonzakura.lexer.Token.Diagnostic;
import java.util.List;

// Like Parsed, for a whole input that is one statement.
record ParsedStmt(SourceFile source, Stmt stmt, List<Diagnostic> parserDiagnostics) {

  static ParsedStmt parse(String text) {
    var source = new SourceFile("test", text);
    var lexer = new Lexer(source);
    var parser = new Parser(lexer);
    var stmt = parser.parseStatement();
    assertTrue(parser.atEnd(), () -> "input left after the statement in: " + text);
    assertEquals(List.of(), lexer.diagnostics(), "lexer diagnostics");
    return new ParsedStmt(source, stmt, List.copyOf(parser.diagnostics()));
  }

  // Parses statements until the input is used up, for inputs that recovery splits into several.
  static All parseAll(String text) {
    var parser = new Parser(new Lexer(new SourceFile("test", text)));
    var trees = parser.parseFile().stream().map(Sexpr::print).toList();
    return new All(trees, parser.diagnostics().stream().map(Diagnostic::message).toList());
  }

  record All(List<String> trees, List<String> messages) {}

  static ParsedStmt parseClean(String text) {
    var parsed = parse(text);
    assertEquals(List.of(), parsed.parserDiagnostics(), "parser diagnostics");
    return parsed;
  }

  <T extends Stmt> T as(Class<T> type) {
    return assertInstanceOf(type, stmt);
  }

  String tree() {
    return Sexpr.print(stmt);
  }

  String text(Stmt node) {
    return source.text().substring(node.start(), node.end());
  }

  String text(Expr node) {
    return source.text().substring(node.start(), node.end());
  }

  String text(Type node) {
    return source.text().substring(node.start(), node.end());
  }

  String text(Stmt.Declarator node) {
    return source.text().substring(node.start(), node.end());
  }

  String text(Stmt.Parameter node) {
    return source.text().substring(node.start(), node.end());
  }

  List<String> parserMessages() {
    return parserDiagnostics.stream().map(Diagnostic::message).toList();
  }
}
