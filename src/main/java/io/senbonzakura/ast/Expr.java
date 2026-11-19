package io.senbonzakura.ast;

import java.util.List;
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

  record IntLiteral(long value, int start, int end) implements Expr {
    public IntLiteral {
      NodeRange.check(start, end);
    }
  }

  record FloatLiteral(double value, int start, int end) implements Expr {
    public FloatLiteral {
      NodeRange.check(start, end);
    }
  }

  record StringLiteral(String value, int start, int end) implements Expr {
    public StringLiteral {
      Objects.requireNonNull(value, "value");
      NodeRange.check(start, end);
    }
  }

  record CharLiteral(int codePoint, int start, int end) implements Expr {
    public CharLiteral {
      if (!Character.isValidCodePoint(codePoint)) {
        throw new IllegalArgumentException("invalid code point " + codePoint);
      }
      NodeRange.check(start, end);
    }
  }

  record BoolLiteral(boolean value, int start, int end) implements Expr {
    public BoolLiteral {
      NodeRange.check(start, end);
    }
  }

  record Identifier(String name, int start, int end) implements Expr {
    public Identifier {
      Objects.requireNonNull(name, "name");
      NodeRange.check(start, end);
    }
  }

  /**
   * {@code callee(a, b)} with positional arguments, or {@code callee(x: a, y: b)} with named ones.
   * A call's arguments are all one kind or all the other; the parser reports a mix, but keeps each
   * argument as written.
   */
  record Call(Expr callee, List<Argument> arguments, int start, int end) implements Expr {
    public Call {
      Objects.requireNonNull(callee, "callee");
      arguments = List.copyOf(arguments);
      NodeRange.check(start, end);
    }
  }

  /**
   * One argument of a {@link Call}: {@code value}, or {@code name: value}. {@code name} is {@code
   * null} for a positional argument, and empty when what came before the {@code :} was not a name,
   * which the parser has already reported. Not an expression on its own.
   */
  record Argument(String name, Expr value, int start, int end) {
    public Argument {
      Objects.requireNonNull(value, "value");
      NodeRange.check(start, end);
    }

    public boolean isNamed() {
      return name != null;
    }
  }

  record Index(Expr target, Expr index, int start, int end) implements Expr {
    public Index {
      Objects.requireNonNull(target, "target");
      Objects.requireNonNull(index, "index");
      NodeRange.check(start, end);
    }
  }

  /**
   * {@code target.name}. The name is empty when it was missing, which the parser has already
   * reported.
   */
  record Field(Expr target, String name, int start, int end) implements Expr {
    public Field {
      Objects.requireNonNull(target, "target");
      Objects.requireNonNull(name, "name");
      NodeRange.check(start, end);
    }
  }

  record Unary(UnaryOp op, Expr operand, int start, int end) implements Expr {
    public Unary {
      Objects.requireNonNull(op, "op");
      Objects.requireNonNull(operand, "operand");
      NodeRange.check(start, end);
    }
  }

  record Binary(BinaryOp op, Expr left, Expr right, int start, int end) implements Expr {
    public Binary {
      Objects.requireNonNull(op, "op");
      Objects.requireNonNull(left, "left");
      Objects.requireNonNull(right, "right");
      NodeRange.check(start, end);
    }
  }

  /** {@code lower..upper}, which excludes {@code upper}. */
  record Range(Expr lower, Expr upper, int start, int end) implements Expr {
    public Range {
      Objects.requireNonNull(lower, "lower");
      Objects.requireNonNull(upper, "upper");
      NodeRange.check(start, end);
    }
  }

  /**
   * {@code target = value}, or a compound form like {@code target += value}. The parser has
   * reported a target that cannot be assigned to, but still builds the node.
   */
  record Assign(AssignOp op, Expr target, Expr value, int start, int end) implements Expr {
    public Assign {
      Objects.requireNonNull(op, "op");
      Objects.requireNonNull(target, "target");
      Objects.requireNonNull(value, "value");
      NodeRange.check(start, end);
    }
  }

  /**
   * {@code target++} or {@code target--}. There is no prefix form. Like {@link Assign}, the parser
   * has reported a target that cannot be assigned to, but still builds the node.
   */
  record Update(UpdateOp op, Expr target, int start, int end) implements Expr {
    public Update {
      Objects.requireNonNull(op, "op");
      Objects.requireNonNull(target, "target");
      NodeRange.check(start, end);
    }
  }

  /**
   * An expression in parentheses. Kept as its own node so the tree still says where the parentheses
   * were, for diagnostics and formatting.
   */
  record Group(Expr inner, int start, int end) implements Expr {
    public Group {
      Objects.requireNonNull(inner, "inner");
      NodeRange.check(start, end);
    }
  }

  enum UnaryOp {
    NEG("-"),
    NOT("!");

    private final String spelling;

    UnaryOp(String spelling) {
      this.spelling = spelling;
    }

    public String spelling() {
      return spelling;
    }
  }

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

  enum AssignOp {
    ASSIGN("="),
    ADD("+="),
    SUB("-="),
    MUL("*="),
    DIV("/="),
    REM("%=");

    private final String spelling;

    AssignOp(String spelling) {
      this.spelling = spelling;
    }

    public String spelling() {
      return spelling;
    }
  }

  enum UpdateOp {
    INCREMENT("++"),
    DECREMENT("--");

    private final String spelling;

    UpdateOp(String spelling) {
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
      NodeRange.check(start, end);
    }
  }
}
