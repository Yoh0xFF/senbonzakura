package io.senbonzakura.check;

import static io.senbonzakura.check.Ty.BOOL;
import static io.senbonzakura.check.Ty.CHAR;
import static io.senbonzakura.check.Ty.DOUBLE;
import static io.senbonzakura.check.Ty.INT;
import static io.senbonzakura.check.Ty.STRING;
import static io.senbonzakura.check.Ty.Special.ANY;
import static io.senbonzakura.check.Ty.Special.ERROR;
import static io.senbonzakura.check.Ty.Special.FLOAT_CONSTANT;
import static io.senbonzakura.check.Ty.Special.INTEGER_CONSTANT;
import static io.senbonzakura.check.Ty.Special.NOTHING;

import io.senbonzakura.ast.Expr;
import io.senbonzakura.ast.Expr.AssignOp;
import io.senbonzakura.ast.Expr.UnaryOp;
import io.senbonzakura.ast.Stmt;
import io.senbonzakura.ast.Type;
import io.senbonzakura.ast.Type.PrimitiveKind;
import io.senbonzakura.lexer.Token.Diagnostic;
import java.util.ArrayList;
import java.util.HashMap;
import java.util.HashSet;
import java.util.IdentityHashMap;
import java.util.List;
import java.util.Map;

/**
 * Checks a parsed file for what the parser cannot see on its own:
 *
 * <ul>
 *   <li>Every name refers to a variable, parameter or function in scope, and no scope declares the
 *       same name twice.
 *   <li>{@code break} and {@code continue} are inside a loop, and {@code return} inside a function
 *       with as many values as the function declares. A function with return types returns on every
 *       path.
 *   <li>A call to a function by its name passes each parameter exactly once.
 *   <li>Every value has the type its place asks for: initializers, arguments, returned values,
 *       conditions, operands and assignments.
 * </ul>
 *
 * <p><b>Scopes.</b> Each block opens a scope, a function's parameters share one with its body, and
 * a loop's variables get one for the whole loop. A variable is visible from the end of its
 * declarator on, so {@code var x = x;} does not see itself. A function is visible throughout the
 * block that declares it, so functions can call each other in any order. Inner scopes see outer
 * names and may shadow them; a function body sees the names around it, but a loop around a function
 * does not hold its {@code break}. Built-in functions, so far only {@code print}, live in a scope
 * above the file's, so a file may declare its own.
 *
 * <p><b>Types.</b> No value converts to another type by itself; {@code x as long} converts
 * explicitly. Number literals are the exception: they take the numeric type their place asks for,
 * if they fit it, and are {@code int} or {@code double} when nothing asks. A variable without a
 * type annotation takes its initializer's type. {@code +} also joins strings.
 *
 * <p>Like the lexer and parser, it never throws on bad input; problems come back as diagnostics.
 * Run it on a tree that parsed cleanly: it skips error nodes and missing names, but the names a
 * broken statement failed to declare would show up as undefined.
 */
public final class Checker {

  private static final FunctionSymbol PRINT =
      new FunctionSymbol("print", List.of("value"), List.of(ANY), List.of());

  private final List<Diagnostic> diagnostics = new ArrayList<>();
  private final Map<Expr, Ty> types = new IdentityHashMap<>();
  private Scope scope;

  /** The function whose body is being checked, or {@code null} at the top level. */
  private FunctionSymbol function;

  /** How many loops around the current statement are inside {@link #function}. */
  private int loopDepth;

  private Checker() {
    var builtins = new Scope(null);
    builtins.names.put(PRINT.name(), PRINT);
    scope = new Scope(builtins);
  }

  /** Checks a file's statements, as {@link io.senbonzakura.parser.Parser#parseFile()} gives. */
  public static List<Diagnostic> check(List<Stmt> statements) {
    return analyze(statements).diagnostics();
  }

  /** Like {@link #check}, and also keeps the type it worked out for each expression. */
  public static Analysis analyze(List<Stmt> statements) {
    var checker = new Checker();
    checker.statements(statements);
    return new Analysis(List.copyOf(checker.diagnostics), checker.types);
  }

  /** What checking a file found: its problems, and the type of every expression it checked. */
  public static final class Analysis {

    private final List<Diagnostic> diagnostics;
    private final Map<Expr, Ty> types;

    private Analysis(List<Diagnostic> diagnostics, Map<Expr, Ty> types) {
      this.diagnostics = diagnostics;
      this.types = types;
    }

    public List<Diagnostic> diagnostics() {
      return diagnostics;
    }

    /**
     * Returns the type of an expression in the checked tree. A number literal has the type it
     * settled into, like {@code byte} in {@code var b: byte = 1}; one that nothing settled, like
     * those in {@code 1 < 2}, keeps its constant type.
     *
     * @throws IllegalArgumentException if the expression was not checked, like a callee name
     */
    public Ty typeOf(Expr expr) {
      Ty type = types.get(expr);
      if (type == null) {
        throw new IllegalArgumentException("no type recorded for " + expr);
      }
      return type;
    }
  }

  // Statements

  /** Checks statements in the current scope, declaring their functions first. */
  private void statements(List<Stmt> statements) {
    for (Stmt stmt : statements) {
      if (stmt instanceof Stmt.Function fn && !fn.name().isEmpty()) {
        declare(fn.name(), signature(fn), fn.start(), fn.end());
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
        condition(s.condition());
        block(s.then());
        if (s.otherwise() != null) {
          statement(s.otherwise());
        }
      }
      case Stmt.Loop s -> {
        if (s.condition() != null) {
          condition(s.condition());
        }
        loopBody(s.body());
      }
      case Stmt.LoopIn s -> {
        Ty type = loopVariable(s);
        enterScope();
        if (!s.name().isEmpty()) {
          declare(s.name(), new VariableSymbol(type), s.start(), s.end());
        }
        loopBody(s.body());
        exitScope();
      }
      case Stmt.LoopStep s -> {
        enterScope();
        declarators(s.init());
        condition(s.condition());
        loopBody(s.body());
        expression(s.step());
        exitScope();
      }
      case Stmt.Continue s -> jump("continue", s.condition(), s.start(), s.end());
      case Stmt.Break s -> jump("break", s.condition(), s.start(), s.end());
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
      Ty type = declaratorType(declarator);
      if (!declarator.name().isEmpty()) {
        declare(declarator.name(), new VariableSymbol(type), declarator.start(), declarator.end());
      }
    }
  }

  /** The declared type if there is one, which the initializer must fit, else the initializer's. */
  private Ty declaratorType(Stmt.Declarator declarator) {
    Ty declared = declarator.type() == null ? null : resolve(declarator.type());
    Expr initializer = declarator.initializer();
    if (initializer == null) {
      if (declared == null && !declarator.name().isEmpty()) {
        report(
            declarator.start(),
            declarator.end(),
            "'" + declarator.name() + "' needs a type or an initializer");
      }
      return declared == null ? ERROR : declared;
    }

    Ty value = expression(initializer);
    if (declared != null) {
      expect(initializer, value, declared);
      return declared;
    }
    if (value == NOTHING || value instanceof Ty.Multiple) {
      report(initializer, "expected a value, found " + describe(value));
      return ERROR;
    }
    return settle(initializer, value);
  }

  /** The type of a {@code loop var x in} variable: the range's, or the declared one if it fits. */
  private Ty loopVariable(Stmt.LoopIn loop) {
    Ty iterable = expression(loop.iterable());
    Ty element;
    if (iterable instanceof Ty.Range range) {
      element = range.element();
    } else {
      if (iterable != ERROR) {
        report(loop.iterable(), "cannot loop over " + describe(iterable));
      }
      element = ERROR;
    }

    if (loop.type() == null) {
      if (element == INTEGER_CONSTANT) {
        retypeRange(loop.iterable(), INT);
        return INT;
      }
      return element;
    }
    Ty declared = resolve(loop.type());
    if (element == INTEGER_CONSTANT) {
      retypeRange(loop.iterable(), isInteger(declared) ? declared : INT);
    }
    boolean fits =
        element == ERROR
            || declared == ERROR
            || declared.equals(element)
            || (element == INTEGER_CONSTANT && isInteger(declared));
    if (!fits) {
      report(
          loop.type().start(),
          loop.type().end(),
          "the loop variable is "
              + describe(declared)
              + ", but the range holds "
              + describe(element == INTEGER_CONSTANT ? INT : element));
    }
    return declared;
  }

  private void jump(String keyword, Expr condition, int start, int end) {
    if (loopDepth == 0) {
      report(start, end, "'" + keyword + "' outside a loop");
    }
    if (condition != null) {
      condition(condition);
    }
  }

  private void returnStatement(Stmt.Return stmt) {
    var types = new ArrayList<Ty>();
    for (Expr value : stmt.values()) {
      types.add(expression(value));
    }
    if (stmt.condition() != null) {
      condition(stmt.condition());
    }

    if (function == null) {
      report(stmt.start(), stmt.end(), "'return' outside a function");
      return;
    }
    int expected = function.returnTypes().size();
    if (types.size() != expected) {
      report(
          stmt.start(),
          stmt.end(),
          "expected "
              + (expected == 0 ? "no return value" : count(expected, "return value"))
              + ", found "
              + (types.isEmpty() ? "none" : types.size()));
      return;
    }
    for (int i = 0; i < expected; i++) {
      expect(stmt.values().get(i), types.get(i), function.returnTypes().get(i));
    }
  }

  /** The function's name was declared with the block around it; this checks its body. */
  private void function(Stmt.Function fn) {
    FunctionSymbol outerFunction = function;
    int outerLoopDepth = loopDepth;
    function = signature(fn);
    loopDepth = 0;

    enterScope();
    for (int i = 0; i < fn.parameters().size(); i++) {
      var parameter = fn.parameters().get(i);
      if (!parameter.name().isEmpty()) {
        Ty type = function.parameterTypes().get(i);
        declare(parameter.name(), new VariableSymbol(type), parameter.start(), parameter.end());
      }
    }
    statements(fn.body().statements());
    exitScope();

    if (!fn.returnTypes().isEmpty() && !ReturnFlow.alwaysReturns(fn.body().statements())) {
      String name = fn.name().isEmpty() ? "function" : "'" + fn.name() + "'";
      report(fn.start(), fn.returnTypes().getLast().end(), name + " does not return on every path");
    }

    function = outerFunction;
    loopDepth = outerLoopDepth;
  }

  private void condition(Expr condition) {
    expect(condition, expression(condition), BOOL);
  }

  // Expressions

  /**
   * Checks an expression and returns its type, {@link Ty.Special#ERROR} once reported, and records
   * the type in {@link #types}.
   */
  private Ty expression(Expr expr) {
    Ty type = compute(expr);
    types.put(expr, type);
    return type;
  }

  private Ty compute(Expr expr) {
    return switch (expr) {
      case Expr.IntLiteral _ -> INTEGER_CONSTANT;
      case Expr.FloatLiteral _ -> FLOAT_CONSTANT;
      case Expr.StringLiteral _ -> STRING;
      case Expr.CharLiteral _ -> CHAR;
      case Expr.BoolLiteral _ -> BOOL;
      case Expr.Identifier e -> identifier(e);
      case Expr.Call e -> call(e);
      case Expr.Index e -> {
        Ty target = expression(e.target());
        expression(e.index());
        if (target != ERROR) {
          report(e.target(), "cannot index " + describe(target));
        }
        yield ERROR;
      }
      case Expr.Field e -> {
        Ty target = expression(e.target());
        if (target != ERROR && !e.name().isEmpty()) {
          report(e, describe(target) + " has no field '" + e.name() + "'");
        }
        yield ERROR;
      }
      case Expr.Unary e -> unary(e);
      case Expr.Binary e -> binary(e);
      case Expr.Range e -> range(e);
      case Expr.Assign e -> assign(e);
      case Expr.Update e -> update(e);
      case Expr.Cast e -> cast(e);
      case Expr.Group e -> expression(e.inner());
      case Expr.Error _ -> ERROR;
    };
  }

  private Ty identifier(Expr.Identifier name) {
    return switch (scope.lookup(name.name())) {
      case null -> notDefined(name);
      case VariableSymbol variable -> variable.type();
      case FunctionSymbol _ -> {
        report(name, "function '" + name.name() + "' cannot be used as a value");
        yield ERROR;
      }
    };
  }

  /** The type of a name being assigned to; a function cannot be. */
  private Ty target(Expr target) {
    if (!(target instanceof Expr.Identifier name)) {
      return expression(target);
    }
    Ty type =
        switch (scope.lookup(name.name())) {
          case null -> notDefined(name);
          case VariableSymbol variable -> variable.type();
          case FunctionSymbol _ -> {
            report(name, "cannot assign to function '" + name.name() + "'");
            yield ERROR;
          }
        };
    types.put(name, type);
    return type;
  }

  private Ty call(Expr.Call call) {
    Ty callee;
    if (call.callee() instanceof Expr.Identifier name) {
      Symbol symbol = scope.lookup(name.name());
      if (symbol instanceof FunctionSymbol fn) {
        return call(call, fn);
      }
      callee = symbol == null ? notDefined(name) : ((VariableSymbol) symbol).type();
    } else {
      callee = expression(call.callee());
    }

    call.arguments().forEach(argument -> expression(argument.value()));
    if (callee != ERROR) {
      report(call.callee(), "cannot call " + describe(callee));
    }
    return ERROR;
  }

  /**
   * Checks a call's arguments against the parameters of the function it names: as many positional
   * arguments as parameters, or every parameter named once, each of the parameter's type. A call
   * that mixes the two, or a function with a missing parameter name, was reported by the parser and
   * is left alone.
   */
  private Ty call(Expr.Call call, FunctionSymbol fn) {
    var arguments = call.arguments();
    var types = new ArrayList<Ty>();
    for (var argument : arguments) {
      types.add(expression(argument.value()));
    }
    Ty result =
        switch (fn.returnTypes().size()) {
          case 0 -> NOTHING;
          case 1 -> fn.returnTypes().getFirst();
          default -> new Ty.Multiple(fn.returnTypes());
        };

    boolean named = !arguments.isEmpty() && arguments.getFirst().isNamed();
    if (arguments.stream().anyMatch(argument -> argument.isNamed() != named)
        || fn.parameterNames().contains("")) {
      return result;
    }

    String function = "'" + fn.name() + "'";
    if (!named) {
      if (arguments.size() != fn.parameterNames().size()) {
        report(
            call,
            function
                + " expects "
                + count(fn.parameterNames().size(), "argument")
                + ", found "
                + arguments.size());
        return result;
      }
      for (int i = 0; i < arguments.size(); i++) {
        expect(arguments.get(i).value(), types.get(i), fn.parameterTypes().get(i));
      }
      return result;
    }

    var given = new HashSet<String>();
    for (int i = 0; i < arguments.size(); i++) {
      var argument = arguments.get(i);
      given.add(argument.name());
      int parameter = fn.parameterNames().indexOf(argument.name());
      if (parameter >= 0) {
        expect(argument.value(), types.get(i), fn.parameterTypes().get(parameter));
      } else if (!argument.name().isEmpty()) {
        report(
            argument.start(),
            argument.end(),
            function + " has no parameter named '" + argument.name() + "'");
      }
    }

    var missing =
        fn.parameterNames().stream()
            .filter(name -> !given.contains(name))
            .map(name -> "'" + name + "'")
            .toList();
    if (!missing.isEmpty()) {
      report(
          call,
          (missing.size() == 1 ? "missing argument " : "missing arguments ")
              + String.join(", ", missing)
              + " for "
              + function);
    }
    return result;
  }

  private Ty unary(Expr.Unary e) {
    Ty operand = expression(e.operand());
    if (operand == ERROR) {
      return ERROR;
    }
    if (e.op() == UnaryOp.NEG) {
      if (isConstant(operand) || isSigned(operand)) {
        return operand;
      }
      report(e, "cannot negate " + describe(operand));
      return ERROR;
    }
    if (operand.equals(BOOL)) {
      return BOOL;
    }
    report(e, "cannot apply '!' to " + describe(operand));
    return ERROR;
  }

  private Ty binary(Expr.Binary e) {
    Ty left = expression(e.left());
    Ty right = expression(e.right());
    String op = e.op().spelling();
    return switch (e.op()) {
      case ADD, SUB, MUL, DIV, REM ->
          arithmetic(e, op, e.op() == Expr.BinaryOp.ADD, e.left(), left, e.right(), right);
      case AND, OR -> {
        if (left != ERROR && right != ERROR && !(left.equals(BOOL) && right.equals(BOOL))) {
          cannotApply(e, op, left, right);
        }
        yield BOOL;
      }
      case EQ, NE, LT, LE, GT, GE -> {
        if (left == ERROR || right == ERROR) {
          yield BOOL;
        }
        Ty common = common(left, right);
        boolean ordered = e.op() != Expr.BinaryOp.EQ && e.op() != Expr.BinaryOp.NE;
        boolean comparable =
            common != null
                && (ordered
                    ? isNumeric(common) || common.equals(CHAR) || common.equals(STRING)
                    : common instanceof Ty.Primitive || isConstant(common));
        if (comparable) {
          fit(e.left(), left, common);
          fit(e.right(), right, common);
        } else {
          cannotApply(e, op, left, right);
        }
        yield BOOL;
      }
    };
  }

  /**
   * Checks {@code + - * / %} and their compound assignments: two numbers of one type, or two
   * strings for {@code +}. Returns that type.
   */
  private Ty arithmetic(
      Expr node,
      String op,
      boolean joinsStrings,
      Expr leftExpr,
      Ty left,
      Expr rightExpr,
      Ty right) {
    if (left == ERROR || right == ERROR) {
      return ERROR;
    }
    Ty common = common(left, right);
    if (common == null || !(isNumeric(common) || (joinsStrings && common.equals(STRING)))) {
      cannotApply(node, op, left, right);
      return ERROR;
    }
    fit(leftExpr, left, common);
    fit(rightExpr, right, common);
    return common;
  }

  private Ty range(Expr.Range e) {
    Ty lower = expression(e.lower());
    Ty upper = expression(e.upper());
    if (lower == ERROR || upper == ERROR) {
      return ERROR;
    }
    Ty common = common(lower, upper);
    if (common == null) {
      report(
          e, "range bounds have different types, " + describe(lower) + " and " + describe(upper));
      return ERROR;
    }
    if (!(isInteger(common) || common == INTEGER_CONSTANT)) {
      report(e, "range bounds must be integers, found " + describe(common));
      return ERROR;
    }
    fit(e.lower(), lower, common);
    fit(e.upper(), upper, common);
    return new Ty.Range(common);
  }

  private Ty assign(Expr.Assign e) {
    Ty target = target(e.target());
    Ty value = expression(e.value());
    if (target == ERROR) {
      return ERROR;
    }
    if (e.op() == AssignOp.ASSIGN) {
      expect(e.value(), value, target);
    } else {
      arithmetic(
          e, e.op().spelling(), e.op() == AssignOp.ADD, e.target(), target, e.value(), value);
    }
    return target;
  }

  private Ty update(Expr.Update e) {
    Ty target = target(e.target());
    if (target == ERROR) {
      return ERROR;
    }
    if (!isInteger(target) && !isFloat(target)) {
      report(e, "cannot apply '" + e.op().spelling() + "' to " + describe(target));
      return ERROR;
    }
    return target;
  }

  /** Numbers convert to any numeric type, and integers to and from {@code char}. */
  private Ty cast(Expr.Cast e) {
    Ty from = expression(e.value());
    Ty to = resolve(e.type());
    if (from == ERROR || to == ERROR) {
      return to;
    }
    boolean converts =
        from.equals(to)
            || (isNumeric(from) && (isInteger(to) || isFloat(to)))
            || ((isInteger(from) || from == INTEGER_CONSTANT) && to.equals(CHAR))
            || (from.equals(CHAR) && isInteger(to));
    if (!converts) {
      report(e, "cannot convert " + describe(from) + " to " + describe(to));
    }
    return to;
  }

  // Types

  /**
   * Reports unless a value of type {@code found} can go where {@code expected} is asked for: the
   * same type, or a constant that fits it.
   */
  private void expect(Expr expr, Ty found, Ty expected) {
    if (found == ERROR || expected == ERROR) {
      return;
    }
    if (expected == ANY) {
      if (found == NOTHING || found instanceof Ty.Multiple) {
        report(expr, "expected a value, found " + describe(found));
      } else {
        settle(expr, found);
      }
      return;
    }
    if (found.equals(expected)) {
      return;
    }
    if (isConstant(found) && takesConstant(expected, found)) {
      fit(expr, found, expected);
      return;
    }
    report(expr, "expected " + describe(expected) + ", found " + describe(found));
  }

  /** The type two operands share, letting a constant take the other side's type; else null. */
  private static Ty common(Ty left, Ty right) {
    if (left.equals(right)) {
      return left;
    }
    if (isConstant(left) && isConstant(right)) {
      return FLOAT_CONSTANT; // one of each
    }
    if (isConstant(left)) {
      return takesConstant(right, left) ? right : null;
    }
    if (isConstant(right)) {
      return takesConstant(left, right) ? left : null;
    }
    return null;
  }

  private static boolean takesConstant(Ty type, Ty constant) {
    return constant == INTEGER_CONSTANT ? isInteger(type) || isFloat(type) : isFloat(type);
  }

  /** The type a constant takes when nothing asks for one: {@code int} or {@code double}. */
  private Ty settle(Expr expr, Ty type) {
    if (type == INTEGER_CONSTANT) {
      fit(expr, type, INT);
      return INT;
    }
    if (type == FLOAT_CONSTANT) {
      fit(expr, type, DOUBLE);
      return DOUBLE;
    }
    if (type instanceof Ty.Range range && range.element() == INTEGER_CONSTANT) {
      retypeRange(expr, INT);
      return new Ty.Range(INT);
    }
    return type;
  }

  /**
   * Reports an integer literal that a constant of type {@code from}, going into {@code to}, does
   * not fit. Only literals are checked, possibly negated or in parentheses: {@code 200 + 100} is
   * not worked out.
   */
  private void fit(Expr expr, Ty from, Ty to) {
    if (!isConstant(from) || from.equals(to)) {
      return;
    }
    retype(expr, to);
    if (from != INTEGER_CONSTANT || !(to instanceof Ty.Primitive(var kind)) || !isInteger(to)) {
      return;
    }
    Long value = literalValue(expr);
    if (value != null && (value < min(kind) || value > max(kind))) {
      report(expr, value + " does not fit in " + describe(to));
    }
  }

  /**
   * Gives a settled constant, and the constants it was computed from, the type it settled into, so
   * {@link Analysis#typeOf} says how to compute each of them.
   */
  private void retype(Expr expr, Ty to) {
    if (!isConstant(types.get(expr))) {
      return;
    }
    types.put(expr, to);
    switch (expr) {
      case Expr.Group e -> retype(e.inner(), to);
      case Expr.Unary e -> retype(e.operand(), to);
      case Expr.Binary e -> {
        retype(e.left(), to);
        retype(e.right(), to);
      }
      default -> {}
    }
  }

  /** Like {@link #retype} for a range of integer constants, such as {@code 0..3}. */
  private void retypeRange(Expr expr, Ty element) {
    switch (expr) {
      case Expr.Group e -> {
        types.put(e, new Ty.Range(element));
        retypeRange(e.inner(), element);
      }
      case Expr.Range e -> {
        types.put(e, new Ty.Range(element));
        fit(e.lower(), INTEGER_CONSTANT, element);
        fit(e.upper(), INTEGER_CONSTANT, element);
      }
      default -> {}
    }
  }

  private static Long literalValue(Expr expr) {
    return switch (expr) {
      case Expr.IntLiteral e -> e.value();
      case Expr.Group e -> literalValue(e.inner());
      case Expr.Unary e when e.op() == UnaryOp.NEG -> {
        Long value = literalValue(e.operand());
        yield value == null ? null : -value;
      }
      default -> null;
    };
  }

  private static Ty resolve(Type type) {
    return switch (type) {
      case Type.Primitive p -> new Ty.Primitive(p.kind());
      case Type.Error _ -> ERROR;
    };
  }

  private static FunctionSymbol signature(Stmt.Function fn) {
    return new FunctionSymbol(
        fn.name(),
        fn.parameters().stream().map(Stmt.Parameter::name).toList(),
        fn.parameters().stream().map(parameter -> resolve(parameter.type())).toList(),
        fn.returnTypes().stream().map(Checker::resolve).toList());
  }

  private static boolean isConstant(Ty type) {
    return type == INTEGER_CONSTANT || type == FLOAT_CONSTANT;
  }

  private static boolean isNumeric(Ty type) {
    return isConstant(type) || isInteger(type) || isFloat(type);
  }

  private static boolean isInteger(Ty type) {
    return type instanceof Ty.Primitive(var kind)
        && switch (kind) {
          case BYTE, SHORT, INT, SIZE, LONG, UBYTE, USHORT, UINT, USIZE, ULONG -> true;
          default -> false;
        };
  }

  private static boolean isFloat(Ty type) {
    return type instanceof Ty.Primitive(var kind)
        && (kind == PrimitiveKind.FLOAT || kind == PrimitiveKind.DOUBLE);
  }

  private static boolean isSigned(Ty type) {
    return type instanceof Ty.Primitive(var kind)
        && switch (kind) {
          case BYTE, SHORT, INT, SIZE, LONG, FLOAT, DOUBLE -> true;
          default -> false;
        };
  }

  // size and usize are taken to be 64 bits wide.
  private static long min(PrimitiveKind kind) {
    return switch (kind) {
      case BYTE -> Byte.MIN_VALUE;
      case SHORT -> Short.MIN_VALUE;
      case INT -> Integer.MIN_VALUE;
      case SIZE, LONG -> Long.MIN_VALUE;
      default -> 0;
    };
  }

  private static long max(PrimitiveKind kind) {
    return switch (kind) {
      case BYTE -> Byte.MAX_VALUE;
      case SHORT -> Short.MAX_VALUE;
      case INT -> Integer.MAX_VALUE;
      case UBYTE -> 0xFF;
      case USHORT -> 0xFFFF;
      case UINT -> 0xFFFF_FFFFL;
      default -> Long.MAX_VALUE; // a literal larger than that was reported by the parser
    };
  }

  static String describe(Ty type) {
    return switch (type) {
      case Ty.Primitive p -> "'" + p.kind().spelling() + "'";
      case Ty.Range r ->
          "a range of " + describe(r.element() == INTEGER_CONSTANT ? INT : r.element());
      case Ty.Multiple m -> m.types().size() + " values";
      case Ty.Special s ->
          switch (s) {
            case INTEGER_CONSTANT -> "integer constant";
            case FLOAT_CONSTANT -> "float constant";
            case NOTHING -> "no value";
            case ANY -> "a value";
            case ERROR -> "an error";
          };
    };
  }

  // Scopes and reporting

  private void declare(String name, Symbol symbol, int start, int end) {
    if (scope.names.putIfAbsent(name, symbol) != null) {
      report(start, end, "'" + name + "' is already declared in this scope");
    }
  }

  private Ty notDefined(Expr.Identifier name) {
    report(name, "'" + name.name() + "' is not defined");
    return ERROR;
  }

  private void enterScope() {
    scope = new Scope(scope);
  }

  private void exitScope() {
    scope = scope.parent;
  }

  private void cannotApply(Expr node, String op, Ty left, Ty right) {
    report(node, "cannot apply '" + op + "' to " + describe(left) + " and " + describe(right));
  }

  private void report(Expr node, String message) {
    report(node.start(), node.end(), message);
  }

  private void report(int start, int end, String message) {
    diagnostics.add(new Diagnostic(start, end, message));
  }

  private static String count(int n, String noun) {
    return n + " " + noun + (n == 1 ? "" : "s");
  }

  private sealed interface Symbol {}

  private record VariableSymbol(Ty type) implements Symbol {}

  /** A function's signature. Names are empty where the parser reported a missing one. */
  private record FunctionSymbol(
      String name, List<String> parameterNames, List<Ty> parameterTypes, List<Ty> returnTypes)
      implements Symbol {}

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
