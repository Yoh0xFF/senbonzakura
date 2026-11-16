package io.senbonzakura.ast;

import java.util.List;
import java.util.Objects;

/**
 * A statement node. Like {@link Expr}, every node covers the source offsets {@code start} up to
 * (not including) {@code end}.
 */
public sealed interface Stmt {

  int start();

  int end();

  /** {@code expr;}. Covers the semicolon too, unless it was missing. */
  record Expression(Expr expr, int start, int end) implements Stmt {
    public Expression {
      Objects.requireNonNull(expr, "expr");
      NodeRange.check(start, end);
    }
  }

  /**
   * {@code let mut name = initializer;}, where {@code mut} and {@code = initializer} are optional.
   * {@code initializer} is {@code null} when there is none, and {@code name} is empty when it was
   * missing, which the parser has already reported.
   */
  record Let(boolean mutable, String name, Expr initializer, int start, int end) implements Stmt {
    public Let {
      Objects.requireNonNull(name, "name");
      NodeRange.check(start, end);
    }
  }

  /** {@code { ... }}. Covers both braces, unless the closing one was missing. */
  record Block(List<Stmt> statements, int start, int end) implements Stmt {
    public Block {
      statements = List.copyOf(statements);
      NodeRange.check(start, end);
    }
  }
}
