package io.senbonzakura.parser;

import io.senbonzakura.ast.Expr;
import io.senbonzakura.lexer.Lexer;
import io.senbonzakura.lexer.SourceFile;
import io.senbonzakura.lexer.Token;
import io.senbonzakura.lexer.Token.Diagnostic;
import java.util.ArrayList;
import java.util.Collections;
import java.util.List;
import java.util.Objects;

/**
 * Builds {@link Expr} trees from the tokens of a {@link Lexer}, by recursive descent.
 *
 * <p>So far it parses only literals.
 *
 * <p>Like the lexer, the parser never throws on bad input. Problems go to {@link #diagnostics()}
 * and the part that failed becomes an {@link Expr.Error} node. Problems the lexer found stay in
 * {@link Lexer#diagnostics()} and are not reported again.
 */
public class Parser {

  private final Lexer lexer;
  private final SourceFile source;
  private final List<Diagnostic> diagnostics = new ArrayList<>();

  /** The next token to parse. */
  private Token current;

  public Parser(Lexer lexer) {
    this.lexer = Objects.requireNonNull(lexer, "lexer");
    this.source = lexer.source();
    this.current = lexer.nextToken();
  }

  /**
   * Returns the problems found so far, in the order found, as a read-only view that keeps growing.
   */
  public List<Diagnostic> diagnostics() {
    return Collections.unmodifiableList(diagnostics);
  }

  /** Returns whether every token has been parsed. */
  public boolean atEnd() {
    return current.kind() == Token.Kind.EOF;
  }

  /**
   * Parses one expression. Always consumes at least one token, unless the input is used up, so a
   * loop calling it until {@link #atEnd()} always finishes.
   */
  public Expr parseExpression() {
    return primary();
  }

  private Expr primary() {
    Token token = current;
    return switch (token.kind()) {
      case INT_LITERAL -> {
        advance();
        yield intLiteral(token);
      }
      case FLOAT_LITERAL -> {
        advance();
        yield floatLiteral(token);
      }
      case STRING_LITERAL -> {
        advance();
        String value = DecodeLiteral.string(text(token), !token.has(Token.UNTERMINATED));
        yield new Expr.StringLiteral(value, token.start(), token.end());
      }
      case CHAR_LITERAL -> {
        advance();
        int codePoint = DecodeLiteral.character(text(token), !token.has(Token.UNTERMINATED));
        yield new Expr.CharLiteral(codePoint, token.start(), token.end());
      }
      case TRUE, FALSE -> {
        advance();
        yield new Expr.BoolLiteral(token.kind() == Token.Kind.TRUE, token.start(), token.end());
      }
      case ERROR -> {
        // The lexer already reported this character.
        advance();
        yield new Expr.Error(token.start(), token.end());
      }
      default -> {
        report(token, "expected expression, found " + describe(token));
        if (token.kind() != Token.Kind.EOF) {
          advance();
        }
        yield new Expr.Error(token.start(), token.end());
      }
    };
  }

  private Expr intLiteral(Token token) {
    var value = DecodeLiteral.integer(text(token));
    if (value.isEmpty()) {
      report(token, "integer literal is too large");
      return new Expr.IntLiteral(0, token.start(), token.end());
    }
    return new Expr.IntLiteral(value.getAsLong(), token.start(), token.end());
  }

  private Expr floatLiteral(Token token) {
    double value = DecodeLiteral.floating(text(token));
    if (Double.isInfinite(value)) {
      report(token, "float literal is out of range");
      value = 0.0;
    }
    return new Expr.FloatLiteral(value, token.start(), token.end());
  }

  private void advance() {
    current = lexer.nextToken();
  }

  private String text(Token token) {
    return source.text(token);
  }

  private String describe(Token token) {
    if (token.kind() == Token.Kind.EOF) {
      return "end of input";
    }
    return "'" + text(token) + "'";
  }

  private void report(Token token, String message) {
    diagnostics.add(new Diagnostic(token.start(), token.end(), message));
  }
}
