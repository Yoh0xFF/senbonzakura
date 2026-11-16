package io.senbonzakura.parser;

import io.senbonzakura.ast.Expr;
import io.senbonzakura.ast.Expr.BinaryOp;
import io.senbonzakura.ast.Expr.UnaryOp;
import io.senbonzakura.lexer.Token;

/**
 * Parses prefix and infix operators, and owns their precedence. Infix operators, loosest first:
 *
 * <ol>
 *   <li>{@code ||}
 *   <li>{@code &&}
 *   <li>{@code == != < <= > >=}, which cannot be chained: {@code a < b < c} is an error
 *   <li>{@code + -}
 *   <li>{@code * / %}
 * </ol>
 *
 * <p>Prefix {@code -} and {@code !} bind tighter than all of them, and every infix operator is
 * left-associative. Operands come from {@link Parser#operand(TokenCursor)}.
 */
final class ParseOperator {

  /** The precedence of {@code ||}, the loosest infix operator. */
  private static final int LOOSEST = 1;

  private ParseOperator() {}

  /** Parses a whole expression: operands joined by any infix operators. */
  static Expr expression(TokenCursor c) {
    return infix(c, LOOSEST);
  }

  /** Parses the prefix {@code -} or {@code !} at the cursor and its operand. */
  static Expr prefix(TokenCursor c) {
    Token token = c.advance();
    UnaryOp op =
        switch (token.kind()) {
          case MINUS -> UnaryOp.NEG;
          case BANG -> UnaryOp.NOT;
          default -> throw new IllegalStateException("not a prefix operator: " + token.kind());
        };
    Expr operand = Parser.operand(c);
    return new Expr.Unary(op, operand, token.start(), operand.end());
  }

  /**
   * Parses operands joined by infix operators of precedence {@code minPrecedence} or higher, by
   * precedence climbing. Each right operand is parsed one level tighter, which makes the operators
   * left-associative.
   */
  private static Expr infix(TokenCursor c, int minPrecedence) {
    Expr left = Parser.operand(c);
    boolean afterComparison = false;
    while (true) {
      BinaryOp op = binaryOp(c.peek().kind());
      if (op == null || precedence(op) < minPrecedence) {
        return left;
      }
      Token opToken = c.advance();
      if (afterComparison && isComparison(op)) {
        c.report(opToken, "comparison operators cannot be chained; join them with '&&'");
      }
      afterComparison = isComparison(op);

      Expr right = infix(c, precedence(op) + 1);
      left = new Expr.Binary(op, left, right, left.start(), right.end());
    }
  }

  /** Returns how tightly {@code op} binds its operands; higher binds tighter. */
  private static int precedence(BinaryOp op) {
    return switch (op) {
      case OR -> 1;
      case AND -> 2;
      case EQ, NE, LT, LE, GT, GE -> 3;
      case ADD, SUB -> 4;
      case MUL, DIV, REM -> 5;
    };
  }

  private static boolean isComparison(BinaryOp op) {
    return precedence(op) == 3;
  }

  /** Returns the infix operator a token spells, or {@code null} if it is not one. */
  private static BinaryOp binaryOp(Token.Kind kind) {
    return switch (kind) {
      case PLUS -> BinaryOp.ADD;
      case MINUS -> BinaryOp.SUB;
      case STAR -> BinaryOp.MUL;
      case SLASH -> BinaryOp.DIV;
      case PERCENT -> BinaryOp.REM;
      case EQ_EQ -> BinaryOp.EQ;
      case BANG_EQ -> BinaryOp.NE;
      case LESS -> BinaryOp.LT;
      case LESS_EQ -> BinaryOp.LE;
      case GREATER -> BinaryOp.GT;
      case GREATER_EQ -> BinaryOp.GE;
      case AND_AND -> BinaryOp.AND;
      case OR_OR -> BinaryOp.OR;
      default -> null;
    };
  }
}
