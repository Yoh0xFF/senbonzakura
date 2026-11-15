package io.senbonzakura.parser;

import io.senbonzakura.ast.Expr;

// Prints a tree as an S-expression, like (+ 1 (* 2 3)), so tests can check its shape in one line.
final class Sexpr {

  private Sexpr() {}

  static String print(Expr expr) {
    return switch (expr) {
      case Expr.IntLiteral e -> Long.toString(e.value());
      case Expr.FloatLiteral e -> Double.toString(e.value());
      case Expr.StringLiteral e -> '"' + e.value() + '"';
      case Expr.CharLiteral e -> "'" + Character.toString(e.codePoint()) + "'";
      case Expr.BoolLiteral e -> Boolean.toString(e.value());
      case Expr.Unary e -> "(" + e.op().spelling() + " " + print(e.operand()) + ")";
      case Expr.Binary e ->
          "(" + e.op().spelling() + " " + print(e.left()) + " " + print(e.right()) + ")";
      case Expr.Group e -> "(group " + print(e.inner()) + ")";
      case Expr.Error e -> "<error>";
    };
  }
}
