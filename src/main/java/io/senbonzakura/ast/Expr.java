package io.senbonzakura.ast;

import java.util.Objects;

/**
 * An expression node. Every node covers the source offsets {@code start} up to (not including)
 * {@code end}, like a {@link io.senbonzakura.lexer.Token}.
 *
 * <p>Literal nodes hold decoded values: {@code 0x1F} becomes {@code 31} and {@code "a\n"} becomes a
 * two-character string.
 */
public sealed interface Expr {

  int start();

  int end();

  /** An integer literal, like {@code 42}, {@code 1_000} or {@code 0xFF}. */
  record IntLiteral(long value, int start, int end) implements Expr {
    public IntLiteral {
      checkRange(start, end);
    }
  }

  /** A float literal, like {@code 1.5} or {@code 1e-3}. */
  record FloatLiteral(double value, int start, int end) implements Expr {
    public FloatLiteral {
      checkRange(start, end);
    }
  }

  /** A string literal, {@code "..."} or {@code `...`}, with its escapes already applied. */
  record StringLiteral(String value, int start, int end) implements Expr {
    public StringLiteral {
      Objects.requireNonNull(value, "value");
      checkRange(start, end);
    }
  }

  /** A character literal, like {@code 'a'} or {@code '\n'}, as one code point. */
  record CharLiteral(int codePoint, int start, int end) implements Expr {
    public CharLiteral {
      if (!Character.isValidCodePoint(codePoint)) {
        throw new IllegalArgumentException("invalid code point " + codePoint);
      }
      checkRange(start, end);
    }
  }

  /** {@code true} or {@code false}. */
  record BoolLiteral(boolean value, int start, int end) implements Expr {
    public BoolLiteral {
      checkRange(start, end);
    }
  }

  /** A prefix operator applied to one operand, like {@code -x} or {@code !done}. */
  record Unary(UnaryOp op, Expr operand, int start, int end) implements Expr {
    public Unary {
      Objects.requireNonNull(op, "op");
      Objects.requireNonNull(operand, "operand");
      checkRange(start, end);
    }
  }

  /** An infix operator applied to two operands, like {@code a + b} or {@code a && b}. */
  record Binary(BinaryOp op, Expr left, Expr right, int start, int end) implements Expr {
    public Binary {
      Objects.requireNonNull(op, "op");
      Objects.requireNonNull(left, "left");
      Objects.requireNonNull(right, "right");
      checkRange(start, end);
    }
  }

  /**
   * An expression in parentheses. Kept as its own node so the tree still says where the parentheses
   * were, for diagnostics and formatting.
   */
  record Group(Expr inner, int start, int end) implements Expr {
    public Group {
      Objects.requireNonNull(inner, "inner");
      checkRange(start, end);
    }
  }

  /** The prefix operators. */
  enum UnaryOp {
    /** {@code -}, arithmetic negation. */
    NEG("-"),
    /** {@code !}, logical not. */
    NOT("!");

    private final String spelling;

    UnaryOp(String spelling) {
      this.spelling = spelling;
    }

    public String spelling() {
      return spelling;
    }
  }

  /** The infix operators. */
  enum BinaryOp {
    ADD("+"),
    SUB("-"),
    MUL("*"),
    DIV("/"),
    REM("%"),
    EQ("=="),
    NE("!="),
    LT("<"),
    LE("<="),
    GT(">"),
    GE(">="),
    /** {@code &&}, which skips the right operand when the left is false. */
    AND("&&"),
    /** {@code ||}, which skips the right operand when the left is true. */
    OR("||");

    private final String spelling;

    BinaryOp(String spelling) {
      this.spelling = spelling;
    }

    public String spelling() {
      return spelling;
    }
  }

  /**
   * Stands in for an expression that could not be parsed. A diagnostic has already been reported,
   * so later passes skip it without reporting again.
   */
  record Error(int start, int end) implements Expr {
    public Error {
      checkRange(start, end);
    }
  }

  private static void checkRange(int start, int end) {
    if (start < 0 || end < start) {
      throw new IllegalArgumentException("invalid node range " + start + ".." + end);
    }
  }
}
