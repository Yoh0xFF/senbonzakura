package io.senbonzakura.parser;

import io.senbonzakura.ast.Expr;
import io.senbonzakura.lexer.Token;

final class ParseIdentifier {

  private ParseIdentifier() {}

  static Expr identifier(TokenCursor c) {
    Token token = c.advance();
    return new Expr.Identifier(c.text(token), token.start(), token.end());
  }
}
