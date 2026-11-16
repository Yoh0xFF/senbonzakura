package io.senbonzakura.parser;

import io.senbonzakura.lexer.Lexer;
import io.senbonzakura.lexer.SourceFile;
import io.senbonzakura.lexer.Token;
import io.senbonzakura.lexer.Token.Diagnostic;
import java.util.ArrayList;
import java.util.List;

/**
 * The parser's only mutable state: the lexer, the next token and the diagnostics so far. Parsers
 * move it forward and report problems through it.
 */
final class TokenCursor {

  private final Lexer lexer;
  private final SourceFile source;
  private final List<Diagnostic> diagnostics = new ArrayList<>();

  private Token current;

  TokenCursor(Lexer lexer) {
    this.lexer = lexer;
    this.source = lexer.source();
    this.current = lexer.nextToken();
  }

  Token peek() {
    return current;
  }

  boolean at(Token.Kind kind) {
    return current.kind() == kind;
  }

  boolean atEnd() {
    return at(Token.Kind.EOF);
  }

  /** Consumes the next token and returns it. At the end, returns EOF without moving. */
  Token advance() {
    Token token = current;
    current = lexer.nextToken();
    return token;
  }

  /**
   * Consumes {@code start} if it is still the next token, so a loop whose last step parsed nothing
   * moves on. Whatever left the token in place has already reported it.
   */
  void skipIfStuck(Token start) {
    if (current == start && !atEnd()) {
      advance();
    }
  }

  String text(Token token) {
    return source.text(token);
  }

  /** Describes a token for a diagnostic, like {@code ')'} or {@code end of input}. */
  String describe(Token token) {
    if (token.kind() == Token.Kind.EOF) {
      return "end of input";
    }
    return "'" + text(token) + "'";
  }

  void report(Token token, String message) {
    report(token.start(), token.end(), message);
  }

  void report(int start, int end, String message) {
    diagnostics.add(new Diagnostic(start, end, message));
  }

  List<Diagnostic> diagnostics() {
    return diagnostics;
  }
}
