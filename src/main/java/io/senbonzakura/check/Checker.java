package io.senbonzakura.check;

import io.senbonzakura.ast.Expr;
import io.senbonzakura.ast.Stmt;
import io.senbonzakura.lexer.Token.Diagnostic;
import java.util.ArrayList;
import java.util.HashMap;
import java.util.HashSet;
import java.util.List;
import java.util.Map;

/**
 * Checks a parsed file for what the parser cannot see on its own:
 *
 * <ul>
 *   <li>Every name refers to a variable, parameter or function in scope, and no scope declares the
 *       same name twice.
 *   <li>{@code break} and {@code continue} are inside a loop, and {@code return} inside a function
 *       with as many values as the function declares.
 *   <li>A call to a function by its name passes each parameter exactly once.
 *   <li>Nothing assigns to a function.
 * </ul>
 *
 * <p>Each block opens a scope, a function's parameters share one with its body, and a loop's
 * variables get one for the whole loop. A variable is visible from the end of its declarator on, so
 * {@code var x = x;} does not see itself. A function is visible throughout the block that declares
 * it, so functions can call each other in any order. Inner scopes see outer names and may shadow
 * them; a function body sees the names around it, but a loop around a function does not hold its
 * {@code break}.
 *
 * <p>Like the lexer and parser, it never throws on bad input; problems come back as diagnostics.
 * Run it on a tree that parsed cleanly: it skips error nodes and missing names, but the names a
 * broken statement failed to declare would show up as undefined.
 */
public final class Checker {

  private final List<Diagnostic> diagnostics = new ArrayList<>();
  private Scope scope = new Scope(null);

  /** The function whose body is being checked, or {@code null} at the top level. */
  private Stmt.Function function;

  /** How many loops around the current statement are inside {@link #function}. */
  private int loopDepth;

  private Checker() {}

  /** Checks a file's statements, as {@link io.senbonzakura.parser.Parser#parseFile()} gives. */
  public static List<Diagnostic> check(List<Stmt> statements) {
    var checker = new Checker();
    checker.statements(statements);
    return List.copyOf(checker.diagnostics);
  }

  /** Checks statements in the current scope, declaring their functions first. */
  private void statements(List<Stmt> statements) {
    for (Stmt stmt : statements) {
      if (stmt instanceof Stmt.Function fn && !fn.name().isEmpty()) {
        declare(fn.name(), new FunctionSymbol(fn), fn.start(), fn.end());
      }
    }
    for (Stmt stmt : statements) {
      statement(stmt);
    }
  }

  private void statement(Stmt stmt) {
    switch (stmt) {
      case Stmt.Expression s -> expression(s.expr());
      case Stmt.Var s -> declarators(s);
      case Stmt.If s -> {
        expression(s.condition());
        block(s.then());
        if (s.otherwise() != null) {
          statement(s.otherwise());
        }
      }
      case Stmt.Loop s -> {
        optional(s.condition());
        loopBody(s.body());
      }
      case Stmt.LoopIn s -> {
        expression(s.iterable());
        enterScope();
        if (!s.name().isEmpty()) {
          declare(s.name(), VARIABLE, s.start(), s.end());
        }
        loopBody(s.body());
        exitScope();
      }
      case Stmt.LoopStep s -> {
        enterScope();
        declarators(s.init());
        expression(s.condition());
        loopBody(s.body());
        expression(s.step());
        exitScope();
      }
      case Stmt.Continue s -> {
        requireLoop("continue", s.start(), s.end());
        optional(s.condition());
      }
      case Stmt.Break s -> {
        requireLoop("break", s.start(), s.end());
        optional(s.condition());
      }
      case Stmt.Return s -> returnStatement(s);
      case Stmt.Function s -> function(s);
      case Stmt.Block s -> block(s);
    }
  }

  private void block(Stmt.Block block) {
    enterScope();
    statements(block.statements());
    exitScope();
  }

  private void loopBody(Stmt.Block body) {
    loopDepth++;
    block(body);
    loopDepth--;
  }

  private void declarators(Stmt.Var var) {
    for (var declarator : var.declarators()) {
      optional(declarator.initializer());
      if (!declarator.name().isEmpty()) {
        declare(declarator.name(), VARIABLE, declarator.start(), declarator.end());
      }
    }
  }

  /** The function's name was declared with the block around it; this checks its body. */
  private void function(Stmt.Function fn) {
    Stmt.Function outerFunction = function;
    int outerLoopDepth = loopDepth;
    function = fn;
    loopDepth = 0;

    enterScope();
    for (var parameter : fn.parameters()) {
      if (!parameter.name().isEmpty()) {
        declare(parameter.name(), VARIABLE, parameter.start(), parameter.end());
      }
    }
    statements(fn.body().statements());
    exitScope();

    function = outerFunction;
    loopDepth = outerLoopDepth;
  }

  private void requireLoop(String keyword, int start, int end) {
    if (loopDepth == 0) {
      report(start, end, "'" + keyword + "' outside a loop");
    }
  }

  private void returnStatement(Stmt.Return stmt) {
    if (function == null) {
      report(stmt.start(), stmt.end(), "'return' outside a function");
    } else {
      int expected = function.returnTypes().size();
      int found = stmt.values().size();
      if (found != expected) {
        report(
            stmt.start(),
            stmt.end(),
            "expected "
                + (expected == 0 ? "no return value" : count(expected, "return value"))
                + ", found "
                + (found == 0 ? "none" : found));
      }
    }
    stmt.values().forEach(this::expression);
    optional(stmt.condition());
  }

  private void expression(Expr expr) {
    switch (expr) {
      case Expr.Identifier e -> {
        if (scope.lookup(e.name()) == null) {
          report(e.start(), e.end(), "'" + e.name() + "' is not defined");
        }
      }
      case Expr.Call e -> {
        expression(e.callee());
        e.arguments().forEach(argument -> expression(argument.value()));
        if (e.callee() instanceof Expr.Identifier name
            && scope.lookup(name.name()) instanceof FunctionSymbol(var declaration)) {
          arguments(e, declaration);
        }
      }
      case Expr.Index e -> {
        expression(e.target());
        expression(e.index());
      }
      case Expr.Field e -> expression(e.target());
      case Expr.Unary e -> expression(e.operand());
      case Expr.Binary e -> {
        expression(e.left());
        expression(e.right());
      }
      case Expr.Range e -> {
        expression(e.lower());
        expression(e.upper());
      }
      case Expr.Assign e -> {
        writable(e.target());
        expression(e.target());
        expression(e.value());
      }
      case Expr.Update e -> {
        writable(e.target());
        expression(e.target());
      }
      case Expr.Group e -> expression(e.inner());
      case Expr.IntLiteral _,
          Expr.FloatLiteral _,
          Expr.StringLiteral _,
          Expr.CharLiteral _,
          Expr.BoolLiteral _,
          Expr.Error _ -> {}
    }
  }

  private void optional(Expr expr) {
    if (expr != null) {
      expression(expr);
    }
  }

  private void writable(Expr target) {
    if (target instanceof Expr.Identifier name
        && scope.lookup(name.name()) instanceof FunctionSymbol) {
      report(name.start(), name.end(), "cannot assign to function '" + name.name() + "'");
    }
  }

  /**
   * Checks a call's arguments against the parameters of the function it names: as many positional
   * arguments as parameters, or every parameter named once. A call that mixes the two, or a
   * function with a missing parameter name, was reported by the parser and is left alone.
   */
  private void arguments(Expr.Call call, Stmt.Function fn) {
    var arguments = call.arguments();
    var parameters = fn.parameters();
    boolean named = !arguments.isEmpty() && arguments.getFirst().isNamed();
    if (arguments.stream().anyMatch(argument -> argument.isNamed() != named)
        || parameters.stream().anyMatch(parameter -> parameter.name().isEmpty())) {
      return;
    }

    String function = "'" + fn.name() + "'";
    if (!named) {
      if (arguments.size() != parameters.size()) {
        report(
            call.start(),
            call.end(),
            function
                + " expects "
                + count(parameters.size(), "argument")
                + ", found "
                + arguments.size());
      }
      return;
    }

    var parameterNames = new HashSet<String>();
    parameters.forEach(parameter -> parameterNames.add(parameter.name()));
    var given = new HashSet<String>();
    for (var argument : arguments) {
      given.add(argument.name());
      if (!argument.name().isEmpty() && !parameterNames.contains(argument.name())) {
        report(
            argument.start(),
            argument.end(),
            function + " has no parameter named '" + argument.name() + "'");
      }
    }

    var missing =
        parameters.stream()
            .map(Stmt.Parameter::name)
            .filter(name -> !given.contains(name))
            .map(name -> "'" + name + "'")
            .toList();
    if (!missing.isEmpty()) {
      report(
          call.start(),
          call.end(),
          (missing.size() == 1 ? "missing argument " : "missing arguments ")
              + String.join(", ", missing)
              + " for "
              + function);
    }
  }

  private void declare(String name, Symbol symbol, int start, int end) {
    if (scope.names.putIfAbsent(name, symbol) != null) {
      report(start, end, "'" + name + "' is already declared in this scope");
    }
  }

  private void enterScope() {
    scope = new Scope(scope);
  }

  private void exitScope() {
    scope = scope.parent;
  }

  private void report(int start, int end, String message) {
    diagnostics.add(new Diagnostic(start, end, message));
  }

  private static String count(int n, String noun) {
    return n + " " + noun + (n == 1 ? "" : "s");
  }

  private sealed interface Symbol {}

  private record VariableSymbol() implements Symbol {}

  private record FunctionSymbol(Stmt.Function declaration) implements Symbol {}

  /** Variables and parameters carry nothing the checks need, so they share one symbol. */
  private static final Symbol VARIABLE = new VariableSymbol();

  private static final class Scope {

    final Scope parent;
    final Map<String, Symbol> names = new HashMap<>();

    Scope(Scope parent) {
      this.parent = parent;
    }

    /** Returns the innermost symbol with this name, or {@code null} if there is none. */
    Symbol lookup(String name) {
      for (Scope s = this; s != null; s = s.parent) {
        Symbol symbol = s.names.get(name);
        if (symbol != null) {
          return symbol;
        }
      }
      return null;
    }
  }
}
