package io.senbonzakura.parser;

import io.senbonzakura.ast.Expr;
import io.senbonzakura.lexer.Token;

/** Parses expressions between delimiters. So far only {@code (...)}. */
final class ParseGroup {

  private ParseGroup() {}

  /**
   * Parses the parenthesized expression at the cursor, which must be on {@code (}. A missing
   * closing parenthesis is reported, and the group then ends where its contents do.
   */
  static Expr parenthesized(TokenCursor c) {
    assert c.at(Token.Kind.LEFT_PAREN) : c.peek();
    Token open = c.advance();
    Expr inner = Parser.expression(c);
    if (!c.at(Token.Kind.RIGHT_PAREN)) {
      c.report(c.peek(), "expected ')', found " + c.describe(c.peek()));
      return new Expr.Group(inner, open.start(), inner.end());
    }
    Token close = c.advance();
    return new Expr.Group(inner, open.start(), close.end());
  }
}
