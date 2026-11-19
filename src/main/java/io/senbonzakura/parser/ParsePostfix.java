package io.senbonzakura.parser;

import io.senbonzakura.ast.Expr;
import io.senbonzakura.ast.Expr.UpdateOp;
import io.senbonzakura.lexer.Token;
import java.util.ArrayList;

/**
 * Parses calls {@code f(a, b)}, indexing {@code a[i]}, field access {@code a.b} and {@code a++} and
 * {@code a--} after a primary, from {@link Parser#primary(TokenCursor)}. They bind tighter than
 * prefix operators, so {@code -f()} is {@code -(f())}, and chain left to right, so {@code a.b(c)}
 * calls {@code a.b}.
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
        case PLUS_PLUS, MINUS_MINUS -> expr = update(c, expr);
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

  /** Like an assignment, reports a target that cannot be written to but still builds the node. */
  private static Expr update(TokenCursor c, Expr target) {
    assert c.at(Token.Kind.PLUS_PLUS) || c.at(Token.Kind.MINUS_MINUS) : c.peek();
    Token token = c.advance();
    UpdateOp op = token.kind() == Token.Kind.PLUS_PLUS ? UpdateOp.INCREMENT : UpdateOp.DECREMENT;
    if (!Parser.isAssignable(target)) {
      c.report(target.start(), target.end(), "invalid target for '" + op.spelling() + "'");
    }
    return new Expr.Update(op, target, target.start(), token.end());
  }

  /**
   * A token an argument list cannot go past: one that ends an enclosing construct, or a {@code {}
   * that starts a block. It must cover every token {@link Parser#primary(TokenCursor)} leaves in
   * place, or the argument loop would never move.
   */
  private static boolean atCloser(TokenCursor c) {
    return switch (c.peek().kind()) {
      case RIGHT_BRACKET, LEFT_BRACE, RIGHT_BRACE, SEMICOLON, EOF -> true;
      default -> false;
    };
  }
}
