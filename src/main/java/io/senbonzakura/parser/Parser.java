package io.senbonzakura.parser;

import io.senbonzakura.ast.Expr;
import io.senbonzakura.ast.Expr.BinaryOp;
import io.senbonzakura.ast.Expr.UnaryOp;
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
 * <p>So far it parses expressions made of literals, parentheses, prefix {@code -} and {@code !},
 * and these infix operators, loosest first:
 *
 * <ol>
 *   <li>{@code ||}
 *   <li>{@code &&}
 *   <li>{@code == != < <= > >=}, which cannot be chained: {@code a < b < c} is an error
 *   <li>{@code + -}
 *   <li>{@code * / %}
 * </ol>
 *
 * <p>Prefix operators bind tighter than all of them, and every infix operator is left-associative.
 *
 * <p>Like the lexer, the parser never throws on bad input. Problems go to {@link #diagnostics()}
 * and the part that failed becomes an {@link Expr.Error} node. Problems the lexer found stay in
 * {@link Lexer#diagnostics()} and are not reported again.
 */
public class Parser {

  /** The precedence of {@code ||}, the loosest infix operator. */
  private static final int LOOSEST = 1;

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
    Token first = current;
    Expr expr = expression();
    if (current == first && !atEnd()) {
      advance(); // a stray closing delimiter, already reported
    }
    return expr;
  }

  private Expr expression() {
    return binary(LOOSEST);
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

  /**
   * Parses operands joined by infix operators of precedence {@code minPrecedence} or higher, by
   * precedence climbing. Each right operand is parsed one level tighter, which makes the operators
   * left-associative.
   */
  private Expr binary(int minPrecedence) {
    Expr left = unary();
    boolean afterComparison = false;
    while (true) {
      BinaryOp op = binaryOp(current.kind());
      if (op == null || precedence(op) < minPrecedence) {
        return left;
      }
      Token opToken = current;
      advance();
      if (afterComparison && isComparison(op)) {
        report(opToken, "comparison operators cannot be chained; join them with '&&'");
      }
      afterComparison = isComparison(op);

      Expr right = binary(precedence(op) + 1);
      left = new Expr.Binary(op, left, right, left.start(), right.end());
    }
  }

  private Expr unary() {
    Token token = current;
    UnaryOp op =
        switch (token.kind()) {
          case MINUS -> UnaryOp.NEG;
          case BANG -> UnaryOp.NOT;
          default -> null;
        };
    if (op == null) {
      return primary();
    }
    advance();
    Expr operand = unary();
    return new Expr.Unary(op, operand, token.start(), operand.end());
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
      case LEFT_PAREN -> {
        advance();
        yield group(token);
      }
      case ERROR -> {
        // The lexer already reported this character.
        advance();
        yield new Expr.Error(token.start(), token.end());
      }
      case RIGHT_PAREN, RIGHT_BRACKET, RIGHT_BRACE, EOF -> {
        // Left in place, so an enclosing group can still match it.
        report(token, "expected expression, found " + describe(token));
        yield new Expr.Error(token.start(), token.start());
      }
      default -> {
        report(token, "expected expression, found " + describe(token));
        advance();
        yield new Expr.Error(token.start(), token.end());
      }
    };
  }

  /** Parses the rest of a parenthesized expression, after the {@code (} in {@code open}. */
  private Expr group(Token open) {
    Expr inner = expression();
    if (current.kind() != Token.Kind.RIGHT_PAREN) {
      report(current, "expected ')', found " + describe(current));
      return new Expr.Group(inner, open.start(), inner.end());
    }
    Token close = current;
    advance();
    return new Expr.Group(inner, open.start(), close.end());
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
