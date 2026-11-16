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

  /** {@code { ... }}. Covers both braces, unless the closing one was missing. */
  record Block(List<Stmt> statements, int start, int end) implements Stmt {
    public Block {
      statements = List.copyOf(statements);
      NodeRange.check(start, end);
    }
  }
}
