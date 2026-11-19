package io.senbonzakura.parser;

import io.senbonzakura.ast.Expr;
import io.senbonzakura.lexer.Token;
import java.util.ArrayList;

/**
 * Parses calls {@code f(a, b)}, indexing {@code a[i]} and field access {@code a.b} after a primary,
 * from {@link Parser#primary(TokenCursor)}. They bind tighter than prefix operators, so {@code
 * -f()} is {@code -(f())}, and chain left to right, so {@code a.b(c)} calls {@code a.b}.
 */
final class ParsePostfix {

  private ParsePostfix() {}

  static Expr postfix(TokenCursor c) {
    Expr expr = Parser.primary(c);
    while (true) {
      switch (c.peek().kind()) {
        case LEFT_PAREN -> expr = call(c, expr);
        case LEFT_BRACKET -> expr = index(c, expr);
        case DOT -> expr = field(c, expr);
        default -> {
          return expr;
        }
      }
    }
  }

  /** Allows a trailing comma, and treats a missing comma between arguments as one problem. */
  private static Expr call(TokenCursor c, Expr callee) {
    assert c.at(Token.Kind.LEFT_PAREN) : c.peek();
    Token open = c.advance();
    var arguments = new ArrayList<Expr>();
    while (!c.at(Token.Kind.RIGHT_PAREN) && !atCloser(c)) {
      arguments.add(Parser.expression(c));
      if (c.at(Token.Kind.COMMA)) {
        c.advance();
      } else if (!c.at(Token.Kind.RIGHT_PAREN) && !atCloser(c)) {
        c.report(c.peek(), "expected ',' or ')', found " + c.describe(c.peek()));
      }
    }

    if (!c.at(Token.Kind.RIGHT_PAREN)) {
      c.report(c.peek(), "expected ')', found " + c.describe(c.peek()));
      int end = arguments.isEmpty() ? open.end() : arguments.getLast().end();
      return new Expr.Call(callee, arguments, callee.start(), end);
    }
    Token close = c.advance();
    return new Expr.Call(callee, arguments, callee.start(), close.end());
  }

  private static Expr index(TokenCursor c, Expr target) {
    assert c.at(Token.Kind.LEFT_BRACKET) : c.peek();
    c.advance();
    Expr index = Parser.expression(c);
    if (!c.at(Token.Kind.RIGHT_BRACKET)) {
      c.report(c.peek(), "expected ']', found " + c.describe(c.peek()));
      return new Expr.Index(target, index, target.start(), index.end());
    }
    Token close = c.advance();
    return new Expr.Index(target, index, target.start(), close.end());
  }

  private static Expr field(TokenCursor c, Expr target) {
    assert c.at(Token.Kind.DOT) : c.peek();
    Token dot = c.advance();
    if (!c.at(Token.Kind.IDENTIFIER)) {
      c.report(c.peek(), "expected field name, found " + c.describe(c.peek()));
      return new Expr.Field(target, "", target.start(), dot.end());
    }
    Token name = c.advance();
    return new Expr.Field(target, c.text(name), target.start(), name.end());
  }

  /** A token that ends an enclosing construct, so an argument list cannot go past it. */
  private static boolean atCloser(TokenCursor c) {
    return switch (c.peek().kind()) {
      case RIGHT_BRACKET, RIGHT_BRACE, SEMICOLON, EOF -> true;
      default -> false;
    };
  }
}
