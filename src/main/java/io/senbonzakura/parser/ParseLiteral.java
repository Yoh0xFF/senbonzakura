package io.senbonzakura.parser;

import io.senbonzakura.ast.Expr;
import io.senbonzakura.lexer.Token;

/**
 * Parses literal tokens into nodes holding their values, using {@link DecodeLiteral}.
 *
 * <p>Each method consumes one token, which must be of the kind it names. Problems in the literal's
 * syntax were reported by the lexer; only values out of range are reported here.
 */
final class ParseLiteral {

  private ParseLiteral() {}

  /** Parses the {@code INT_LITERAL} at the cursor. */
  static Expr integer(TokenCursor c) {
    Token token = c.advance();
    var value = DecodeLiteral.integer(c.text(token));
    if (value.isEmpty()) {
      c.report(token, "integer literal is too large");
      return new Expr.IntLiteral(0, token.start(), token.end());
    }
    return new Expr.IntLiteral(value.getAsLong(), token.start(), token.end());
  }

  /** Parses the {@code FLOAT_LITERAL} at the cursor. */
  static Expr floating(TokenCursor c) {
    Token token = c.advance();
    double value = DecodeLiteral.floating(c.text(token));
    if (Double.isInfinite(value)) {
      c.report(token, "float literal is out of range");
      value = 0.0;
    }
    return new Expr.FloatLiteral(value, token.start(), token.end());
  }

  /** Parses the {@code STRING_LITERAL} at the cursor. */
  static Expr string(TokenCursor c) {
    Token token = c.advance();
    String value = DecodeLiteral.string(c.text(token), !token.has(Token.UNTERMINATED));
    return new Expr.StringLiteral(value, token.start(), token.end());
  }

  /** Parses the {@code CHAR_LITERAL} at the cursor. */
  static Expr character(TokenCursor c) {
    Token token = c.advance();
    int codePoint = DecodeLiteral.character(c.text(token), !token.has(Token.UNTERMINATED));
    return new Expr.CharLiteral(codePoint, token.start(), token.end());
  }

  /** Parses the {@code true} or {@code false} at the cursor. */
  static Expr bool(TokenCursor c) {
    Token token = c.advance();
    return new Expr.BoolLiteral(token.kind() == Token.Kind.TRUE, token.start(), token.end());
  }
}
