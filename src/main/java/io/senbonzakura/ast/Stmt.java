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
   * {@code let name = initializer;}, where {@code = initializer} is optional. {@code initializer}
   * is {@code null} when there is none, and {@code name} is empty when it was missing, which the
   * parser has already reported.
   */
  record Let(String name, Expr initializer, int start, int end) implements Stmt {
    public Let {
      Objects.requireNonNull(name, "name");
      NodeRange.check(start, end);
    }
  }

  /**
   * {@code if condition { ... } else { ... }}, where {@code else} is optional and may be followed
   * by another {@code if} instead of a block. The braces are required, so {@code then} is always a
   * block, and {@code otherwise} is a block, an {@code If} for {@code else if}, or {@code null}
   * when there is no {@code else}. A body that was missing, which the parser has already reported,
   * is an empty block where it should have started.
   */
  record If(Expr condition, Block then, Stmt otherwise, int start, int end) implements Stmt {
    public If {
      Objects.requireNonNull(condition, "condition");
      Objects.requireNonNull(then, "then");
      if (otherwise != null && !(otherwise instanceof Block || otherwise instanceof If)) {
        throw new IllegalArgumentException("else branch must be a block or an if: " + otherwise);
      }
      NodeRange.check(start, end);
    }
  }

  /**
   * {@code loop condition { ... }}, which runs while {@code condition} holds, or {@code loop { ...
   * }}, which runs until something leaves it, with a {@code null} condition.
   */
  record Loop(Expr condition, Block body, int start, int end) implements Stmt {
    public Loop {
      Objects.requireNonNull(body, "body");
      NodeRange.check(start, end);
    }
  }

  /**
   * {@code loop let name in iterable { ... }}. {@code name} is empty when it was missing, which the
   * parser has already reported.
   */
  record LoopIn(String name, Expr iterable, Block body, int start, int end) implements Stmt {
    public LoopIn {
      Objects.requireNonNull(name, "name");
      Objects.requireNonNull(iterable, "iterable");
      Objects.requireNonNull(body, "body");
      NodeRange.check(start, end);
    }
  }

  /**
   * {@code loop let i = 0; condition; step { ... }}. {@code init} runs once and covers its own
   * {@code ;}, and {@code step} runs after each pass through the body.
   */
  record LoopStep(Let init, Expr condition, Expr step, Block body, int start, int end)
      implements Stmt {
    public LoopStep {
      Objects.requireNonNull(init, "init");
      Objects.requireNonNull(condition, "condition");
      Objects.requireNonNull(step, "step");
      Objects.requireNonNull(body, "body");
      NodeRange.check(start, end);
    }
  }

  /**
   * {@code continue;} or {@code continue if condition;}, which skips the rest of the loop body only
   * when {@code condition} holds. {@code condition} is {@code null} when there is no {@code if}.
   * Covers the semicolon too, unless it was missing.
   */
  record Continue(Expr condition, int start, int end) implements Stmt {
    public Continue {
      NodeRange.check(start, end);
    }
  }

  /**
   * {@code break;} or {@code break if condition;}, which leaves the loop only when {@code
   * condition} holds. {@code condition} is {@code null} when there is no {@code if}. Covers the
   * semicolon too, unless it was missing.
   */
  record Break(Expr condition, int start, int end) implements Stmt {
    public Break {
      NodeRange.check(start, end);
    }
  }

  /**
   * {@code return value if condition;}, where both {@code value} and {@code if condition} are
   * optional, and each is {@code null} when absent. With a condition it returns only when the
   * condition holds. Covers the semicolon too, unless it was missing.
   */
  record Return(Expr value, Expr condition, int start, int end) implements Stmt {
    public Return {
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
