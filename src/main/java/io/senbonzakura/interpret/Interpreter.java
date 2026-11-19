package io.senbonzakura.interpret;

import io.senbonzakura.ast.Expr;
import io.senbonzakura.ast.Expr.AssignOp;
import io.senbonzakura.ast.Expr.BinaryOp;
import io.senbonzakura.ast.Stmt;
import io.senbonzakura.ast.Type;
import io.senbonzakura.check.Checker;
import io.senbonzakura.check.Ty;
import io.senbonzakura.interpret.Values.RangeValue;
import java.io.PrintStream;
import java.util.ArrayList;
import java.util.HashMap;
import java.util.List;
import java.util.Map;

/**
 * Runs a file by walking its tree. The file must have checked cleanly: the interpreter relies on
 * the checker for every name to be defined and every value to have the right type, and takes the
 * type of each operation from its {@link Checker.Analysis}.
 *
 * <p>A file runs its top-level statements in order, like a script. As in the checker, the functions
 * of a block are defined before its statements run, so they can be called before their declaration,
 * and a nested function sees the variables around it as they are when it runs.
 *
 * <p>{@code print} writes its value and a line break to the output stream. Problems only a run can
 * find, like a division by zero, end it with a {@link RuntimeError}.
 */
public final class Interpreter {

  private final Checker.Analysis analysis;
  private final PrintStream out;

  /** The values a {@code return} gave, read by the call that is returning. */
  private List<Object> returned = List.of();

  public Interpreter(Checker.Analysis analysis, PrintStream out) {
    this.analysis = analysis;
    this.out = out;
  }

  /**
   * Runs a file's statements.
   *
   * @throws RuntimeError if the program fails, for example by dividing by zero
   */
  public void run(List<Stmt> statements) {
    var builtins = new Env(null);
    builtins.define("print", Builtin.PRINT);
    try {
      statements(statements, new Env(builtins));
    } catch (StackOverflowError e) {
      throw new RuntimeError("stack overflow: the calls nest too deeply");
    }
  }

  /** How a statement finished, so loops and calls know whether to go on. */
  private enum Flow {
    NORMAL,
    BREAK,
    CONTINUE,
    RETURN
  }

  // Statements

  /** Runs statements in {@code env}, after defining their functions. */
  private Flow statements(List<Stmt> statements, Env env) {
    for (Stmt stmt : statements) {
      if (stmt instanceof Stmt.Function fn) {
        env.define(fn.name(), new FunctionValue(fn, env));
      }
    }
    for (Stmt stmt : statements) {
      Flow flow = execute(stmt, env);
      if (flow != Flow.NORMAL) {
        return flow;
      }
    }
    return Flow.NORMAL;
  }

  private Flow execute(Stmt stmt, Env env) {
    return switch (stmt) {
      case Stmt.Expression s -> {
        evaluate(s.expr(), env);
        yield Flow.NORMAL;
      }
      case Stmt.Var s -> {
        declare(s, env);
        yield Flow.NORMAL;
      }
      case Stmt.If s -> {
        if (isTrue(s.condition(), env)) {
          yield block(s.then(), env);
        }
        yield s.otherwise() == null ? Flow.NORMAL : execute(s.otherwise(), env);
      }
      case Stmt.Loop s -> {
        while (s.condition() == null || isTrue(s.condition(), env)) {
          Flow flow = block(s.body(), env);
          if (flow == Flow.BREAK) {
            break;
          }
          if (flow == Flow.RETURN) {
            yield Flow.RETURN;
          }
        }
        yield Flow.NORMAL;
      }
      case Stmt.LoopIn s -> loopIn(s, env);
      case Stmt.LoopStep s -> {
        var loopEnv = new Env(env);
        declare(s.init(), loopEnv);
        while (isTrue(s.condition(), loopEnv)) {
          Flow flow = block(s.body(), loopEnv);
          if (flow == Flow.BREAK) {
            break;
          }
          if (flow == Flow.RETURN) {
            yield Flow.RETURN;
          }
          evaluate(s.step(), loopEnv);
        }
        yield Flow.NORMAL;
      }
      case Stmt.Continue s ->
          s.condition() == null || isTrue(s.condition(), env) ? Flow.CONTINUE : Flow.NORMAL;
      case Stmt.Break s ->
          s.condition() == null || isTrue(s.condition(), env) ? Flow.BREAK : Flow.NORMAL;
      case Stmt.Return s -> {
        if (s.condition() != null && !isTrue(s.condition(), env)) {
          yield Flow.NORMAL;
        }
        var values = new ArrayList<>(s.values().size());
        for (Expr value : s.values()) {
          values.add(evaluate(value, env));
        }
        returned = values;
        yield Flow.RETURN;
      }
      case Stmt.Function _ -> Flow.NORMAL; // defined with the block around it
      case Stmt.Block s -> block(s, env);
    };
  }

  private Flow block(Stmt.Block block, Env env) {
    return statements(block.statements(), new Env(env));
  }

  /** Runs the body once for each integer of the range, each time with the variable set anew. */
  private Flow loopIn(Stmt.LoopIn loop, Env env) {
    var range = (RangeValue) evaluate(loop.iterable(), env);
    Ty element = ((Ty.Range) analysis.typeOf(loop.iterable())).element();
    for (long i = range.lower();
        Values.compare(BinaryOp.LT, element, i, range.upper());
        i = Values.wrap(i + 1, ((Ty.Primitive) element).kind())) {
      var loopEnv = new Env(env);
      loopEnv.define(loop.name(), i);
      Flow flow = block(loop.body(), loopEnv);
      if (flow == Flow.BREAK) {
        break;
      }
      if (flow == Flow.RETURN) {
        return Flow.RETURN;
      }
    }
    return Flow.NORMAL;
  }

  private void declare(Stmt.Var var, Env env) {
    for (var declarator : var.declarators()) {
      Object value =
          declarator.initializer() != null
              ? evaluate(declarator.initializer(), env)
              : Values.zero(primitive(declarator.type()));
      env.define(declarator.name(), value);
    }
  }

  private boolean isTrue(Expr condition, Env env) {
    return (Boolean) evaluate(condition, env);
  }

  // Expressions

  private Object evaluate(Expr expr, Env env) {
    return switch (expr) {
      case Expr.IntLiteral e -> Values.integerLiteral(e.value(), typeOf(e));
      case Expr.FloatLiteral e -> Values.floatLiteral(e.value(), typeOf(e));
      case Expr.StringLiteral e -> e.value();
      case Expr.CharLiteral e -> (long) e.codePoint();
      case Expr.BoolLiteral e -> e.value();
      case Expr.Identifier e -> env.get(e.name());
      case Expr.Call e -> call(e, env);
      case Expr.Unary e -> {
        Object operand = evaluate(e.operand(), env);
        yield switch (e.op()) {
          case NEG -> Values.negate(typeOf(e), operand);
          case NOT -> !(Boolean) operand;
        };
      }
      case Expr.Binary e -> binary(e, env);
      case Expr.Range e ->
          new RangeValue((Long) evaluate(e.lower(), env), (Long) evaluate(e.upper(), env));
      case Expr.Assign e -> assign(e, env);
      case Expr.Update e -> {
        String name = ((Expr.Identifier) e.target()).name();
        Ty type = typeOf(e.target());
        Object old = env.get(name);
        BinaryOp op = e.op() == Expr.UpdateOp.INCREMENT ? BinaryOp.ADD : BinaryOp.SUB;
        env.assign(name, Values.arithmetic(op, type, old, Values.one(type), e));
        yield old;
      }
      case Expr.Cast e -> Values.cast(evaluate(e.value(), env), typeOf(e.value()), typeOf(e), e);
      case Expr.Group e -> evaluate(e.inner(), env);
      case Expr.Index _, Expr.Field _, Expr.Error _ ->
          throw new IllegalStateException("the checker rejects " + expr);
    };
  }

  private Object binary(Expr.Binary e, Env env) {
    if (e.op() == BinaryOp.AND) {
      return isTrue(e.left(), env) && isTrue(e.right(), env);
    }
    if (e.op() == BinaryOp.OR) {
      return isTrue(e.left(), env) || isTrue(e.right(), env);
    }
    Object left = evaluate(e.left(), env);
    Object right = evaluate(e.right(), env);
    // The checker settles both operands into one type, so the left one's is the operation's.
    Ty type = typeOf(e.left());
    return switch (e.op()) {
      case ADD, SUB, MUL, DIV, REM -> Values.arithmetic(e.op(), type, left, right, e);
      default -> Values.compare(e.op(), type, left, right);
    };
  }

  private Object assign(Expr.Assign e, Env env) {
    String name = ((Expr.Identifier) e.target()).name();
    Object value = evaluate(e.value(), env);
    if (e.op() != AssignOp.ASSIGN) {
      value = Values.arithmetic(arithmetic(e.op()), typeOf(e.target()), env.get(name), value, e);
    }
    env.assign(name, value);
    return value;
  }

  private static BinaryOp arithmetic(AssignOp op) {
    return switch (op) {
      case ADD -> BinaryOp.ADD;
      case SUB -> BinaryOp.SUB;
      case MUL -> BinaryOp.MUL;
      case DIV -> BinaryOp.DIV;
      case REM -> BinaryOp.REM;
      case ASSIGN -> throw new IllegalStateException("not compound: " + op);
    };
  }

  /**
   * Calls a function by name. It returns {@code null} for no value, the value for one, and a list
   * for several, which only a statement can receive.
   */
  private Object call(Expr.Call call, Env env) {
    Object callee = env.get(((Expr.Identifier) call.callee()).name());
    var arguments = new ArrayList<>(call.arguments().size());
    for (var argument : call.arguments()) {
      arguments.add(evaluate(argument.value(), env));
    }

    if (callee == Builtin.PRINT) {
      Expr argument = call.arguments().getFirst().value();
      out.println(Values.format(typeOf(argument), arguments.getFirst()));
      return null;
    }

    var fn = (FunctionValue) callee;
    var parameters = fn.declaration().parameters();
    var callEnv = new Env(fn.closure());
    for (int i = 0; i < arguments.size(); i++) {
      var argument = call.arguments().get(i);
      String name = argument.isNamed() ? argument.name() : parameters.get(i).name();
      callEnv.define(name, arguments.get(i));
    }

    if (statements(fn.declaration().body().statements(), callEnv) != Flow.RETURN) {
      return null;
    }
    List<Object> values = returned;
    returned = List.of();
    return switch (values.size()) {
      case 0 -> null;
      case 1 -> values.getFirst();
      default -> values;
    };
  }

  private Ty typeOf(Expr expr) {
    return analysis.typeOf(expr);
  }

  private static Ty primitive(Type type) {
    return new Ty.Primitive(((Type.Primitive) type).kind());
  }

  private enum Builtin {
    PRINT
  }

  private record FunctionValue(Stmt.Function declaration, Env closure) {}

  /** The variables and functions of one scope, by name, and the scope around it. */
  private static final class Env {

    private final Env parent;
    private final Map<String, Object> values = new HashMap<>();

    Env(Env parent) {
      this.parent = parent;
    }

    void define(String name, Object value) {
      values.put(name, value);
    }

    Object get(String name) {
      return scopeOf(name).values.get(name);
    }

    void assign(String name, Object value) {
      scopeOf(name).values.put(name, value);
    }

    private Env scopeOf(String name) {
      for (Env env = this; env != null; env = env.parent) {
        if (env.values.containsKey(name)) {
          return env;
        }
      }
      throw new IllegalStateException("the checker rejects undefined '" + name + "'");
    }
  }
}
