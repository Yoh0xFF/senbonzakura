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
   * {@code var a: int = 1, b = 2;}, which declares one or more variables. Covers the semicolon too,
   * unless it was missing.
   */
  record Var(List<Declarator> declarators, int start, int end) implements Stmt {
    public Var {
      declarators = List.copyOf(declarators);
      if (declarators.isEmpty()) {
        throw new IllegalArgumentException("a var declares at least one variable");
      }
      NodeRange.check(start, end);
    }
  }

  /**
   * One variable of a {@link Var}: {@code name: type = initializer}, where {@code : type} and
   * {@code = initializer} are optional, and each is {@code null} when absent. {@code name} is empty
   * when it was missing, which the parser has already reported. Not a statement on its own.
   */
  record Declarator(String name, Type type, Expr initializer, int start, int end) {
    public Declarator {
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
   * {@code loop var name: type in iterable { ... }}, where {@code : type} is optional and {@code
   * type} is {@code null} without it. {@code name} is empty when it was missing, which the parser
   * has already reported.
   */
  record LoopIn(String name, Type type, Expr iterable, Block body, int start, int end)
      implements Stmt {
    public LoopIn {
      Objects.requireNonNull(name, "name");
      Objects.requireNonNull(iterable, "iterable");
      Objects.requireNonNull(body, "body");
      NodeRange.check(start, end);
    }
  }

  /**
   * {@code loop var i = 0; condition; step { ... }}. {@code init} runs once and covers its own
   * {@code ;}, and {@code step} runs after each pass through the body.
   */
  record LoopStep(Var init, Expr condition, Expr step, Block body, int start, int end)
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
   * {@code return a, b if condition;}, which returns zero or more values; {@code values} is empty
   * for a plain {@code return}. The {@code if condition} is optional and {@code null} when absent;
   * with one it returns only when the condition holds. Covers the semicolon too, unless it was
   * missing.
   */
  record Return(List<Expr> values, Expr condition, int start, int end) implements Stmt {
    public Return {
      values = List.copyOf(values);
      NodeRange.check(start, end);
    }
  }

  /**
   * {@code fn name(a: int, b: int): int, bool { ... }}. {@code returnTypes} is empty when there is
   * no {@code :} after the parameters, so the function returns nothing, and holds several types
   * when it returns several values. {@code name} is empty when it was missing, which the parser has
   * already reported.
   */
  record Function(
      String name,
      List<Parameter> parameters,
      List<Type> returnTypes,
      Block body,
      int start,
      int end)
      implements Stmt {
    public Function {
      Objects.requireNonNull(name, "name");
      parameters = List.copyOf(parameters);
      returnTypes = List.copyOf(returnTypes);
      Objects.requireNonNull(body, "body");
      NodeRange.check(start, end);
    }
  }

  /**
   * One parameter of a {@link Function}: {@code name: type}. The type is required; when it was
   * missing, which the parser has already reported, it is a {@link Type.Error}. {@code name} is
   * empty when it was missing. Not a statement on its own.
   */
  record Parameter(String name, Type type, int start, int end) {
    public Parameter {
      Objects.requireNonNull(name, "name");
      Objects.requireNonNull(type, "type");
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
