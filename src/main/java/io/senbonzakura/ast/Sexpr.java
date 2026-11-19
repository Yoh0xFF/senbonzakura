package io.senbonzakura.ast;

import java.util.stream.Collectors;
import java.util.stream.Stream;

/**
 * Prints a tree as an S-expression, like {@code (+ 1 (* 2 3))}, so its shape fits on one line.
 * Calls print as {@code (call f 1 2)}, or {@code (call f x: 1 y: 2)} with named arguments.
 * Statements print as <code>{(= x 1); y;}</code>, declarations as {@code (var a: int 1, b)}, and
 * functions as <code>(fn f (a: int): bool {...})</code>. Parts that failed to parse print as {@code
 * <error>}.
 */
public final class Sexpr {

  private Sexpr() {}

  public static String print(Expr expr) {
    return switch (expr) {
      case Expr.IntLiteral e -> Long.toString(e.value());
      case Expr.FloatLiteral e -> Double.toString(e.value());
      case Expr.StringLiteral e -> '"' + e.value() + '"';
      case Expr.CharLiteral e -> "'" + Character.toString(e.codePoint()) + "'";
      case Expr.BoolLiteral e -> Boolean.toString(e.value());
      case Expr.Identifier e -> e.name();
      case Expr.Call e ->
          Stream.concat(
                  Stream.of("call", print(e.callee())), e.arguments().stream().map(Sexpr::argument))
              .collect(Collectors.joining(" ", "(", ")"));
      case Expr.Index e -> "(index " + print(e.target()) + " " + print(e.index()) + ")";
      case Expr.Field e ->
          "(. " + print(e.target()) + " " + (e.name().isEmpty() ? "<error>" : e.name()) + ")";
      case Expr.Unary e -> "(" + e.op().spelling() + " " + print(e.operand()) + ")";
      case Expr.Binary e ->
          "(" + e.op().spelling() + " " + print(e.left()) + " " + print(e.right()) + ")";
      case Expr.Range e -> "(.. " + print(e.lower()) + " " + print(e.upper()) + ")";
      case Expr.Assign e ->
          "(" + e.op().spelling() + " " + print(e.target()) + " " + print(e.value()) + ")";
      case Expr.Update e -> "(" + e.op().spelling() + " " + print(e.target()) + ")";
      case Expr.Group e -> "(group " + print(e.inner()) + ")";
      case Expr.Cast e -> "(as " + print(e.value()) + " " + print(e.type()) + ")";
      case Expr.Error e -> "<error>";
    };
  }

  public static String print(Stmt stmt) {
    return switch (stmt) {
      case Stmt.Expression s -> print(s.expr()) + ";";
      case Stmt.Var s ->
          s.declarators().stream()
              .map(Sexpr::declarator)
              .collect(Collectors.joining(", ", "(var ", ")"));
      case Stmt.If s ->
          "(if "
              + print(s.condition())
              + " "
              + print(s.then())
              + (s.otherwise() == null ? "" : " else " + print(s.otherwise()))
              + ")";
      case Stmt.Loop s ->
          "(loop "
              + (s.condition() == null ? "" : print(s.condition()) + " ")
              + print(s.body())
              + ")";
      case Stmt.LoopIn s ->
          "(loop var "
              + name(s.name())
              + typeAnnotation(s.type())
              + " in "
              + print(s.iterable())
              + " "
              + print(s.body())
              + ")";
      case Stmt.LoopStep s ->
          "(loop "
              + print(s.init())
              + " "
              + print(s.condition())
              + " "
              + print(s.step())
              + " "
              + print(s.body())
              + ")";
      case Stmt.Continue s ->
          s.condition() == null ? "(continue)" : "(continue if " + print(s.condition()) + ")";
      case Stmt.Break s ->
          s.condition() == null ? "(break)" : "(break if " + print(s.condition()) + ")";
      case Stmt.Return s ->
          "(return"
              + s.values().stream().map(v -> " " + print(v)).collect(Collectors.joining(","))
              + (s.condition() == null ? "" : " if " + print(s.condition()))
              + ")";
      case Stmt.Function s ->
          "(fn "
              + name(s.name())
              + s.parameters().stream()
                  .map(p -> name(p.name()) + typeAnnotation(p.type()))
                  .collect(Collectors.joining(", ", " (", ")"))
              + (s.returnTypes().isEmpty()
                  ? ""
                  : s.returnTypes().stream()
                      .map(Sexpr::print)
                      .collect(Collectors.joining(", ", ": ", "")))
              + " "
              + print(s.body())
              + ")";
      case Stmt.Block s ->
          s.statements().stream().map(Sexpr::print).collect(Collectors.joining(" ", "{", "}"));
    };
  }

  public static String print(Type type) {
    return switch (type) {
      case Type.Primitive t -> t.kind().spelling();
      case Type.Error t -> "<error>";
    };
  }

  private static String argument(Expr.Argument a) {
    return a.isNamed() ? name(a.name()) + ": " + print(a.value()) : print(a.value());
  }

  private static String declarator(Stmt.Declarator d) {
    return name(d.name())
        + typeAnnotation(d.type())
        + (d.initializer() == null ? "" : " " + print(d.initializer()));
  }

  private static String name(String name) {
    return name.isEmpty() ? "<error>" : name;
  }

  private static String typeAnnotation(Type type) {
    return type == null ? "" : ": " + print(type);
  }
}
