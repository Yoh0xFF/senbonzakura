package io.senbonzakura.parser;

import io.senbonzakura.ast.Expr;
import io.senbonzakura.ast.Expr.AssignOp;
import io.senbonzakura.ast.Expr.BinaryOp;
import io.senbonzakura.ast.Expr.UnaryOp;
import io.senbonzakura.lexer.Token;

/**
 * Parses prefix and infix operators, and owns their precedence. Infix operators, loosest first:
 *
 * <ol>
 *   <li>{@code = += -= *= /= %=}, which are right-associative and need a name, field or index on
 *       the left
 *   <li>{@code ..}, which cannot be chained: {@code a..b..c} is an error
 *   <li>{@code ||}
 *   <li>{@code &&}
 *   <li>{@code == != < <= > >=}, which cannot be chained: {@code a < b < c} is an error
 *   <li>{@code + -}
 *   <li>{@code * / %}
 * </ol>
 *
 * <p>Prefix {@code -} and {@code !} bind tighter than all of them, and postfix calls, indexing,
 * field access, {@code ++} and {@code --} tighter still. The infix operators from {@code ||} down
 * are left-associative. Operands come from {@link Parser#operand(TokenCursor)}.
 */
final class ParseOperator {

  private static final int LOOSEST = 1;

  private ParseOperator() {}

  static Expr expression(TokenCursor c) {
    return assignment(c);
  }

  static Expr prefix(TokenCursor c) {
    assert c.at(Token.Kind.MINUS) || c.at(Token.Kind.BANG) : c.peek();
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

  private static Expr assignment(TokenCursor c) {
    Expr target = range(c);
    AssignOp op = assignOp(c.peek().kind());
    if (op == null) {
      return target;
    }
    c.advance();
    if (!Parser.isAssignable(target)) {
      c.report(target.start(), target.end(), "invalid assignment target");
    }
    Expr value = assignment(c);
    return new Expr.Assign(op, target, value, target.start(), value.end());
  }

  private static Expr range(TokenCursor c) {
    Expr left = infix(c, LOOSEST);
    boolean chained = false;
    while (c.at(Token.Kind.DOT_DOT)) {
      Token opToken = c.advance();
      if (chained) {
        c.report(opToken, "range operators cannot be chained");
      }
      chained = true;

      Expr right = infix(c, LOOSEST);
      left = new Expr.Range(left, right, left.start(), right.end());
    }
    return left;
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

  /** Higher binds tighter. */
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

  /** Returns {@code null} if the token is not an assignment operator. */
  private static AssignOp assignOp(Token.Kind kind) {
    return switch (kind) {
      case EQ -> AssignOp.ASSIGN;
      case PLUS_EQ -> AssignOp.ADD;
      case MINUS_EQ -> AssignOp.SUB;
      case STAR_EQ -> AssignOp.MUL;
      case SLASH_EQ -> AssignOp.DIV;
      case PERCENT_EQ -> AssignOp.REM;
      default -> null;
    };
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
