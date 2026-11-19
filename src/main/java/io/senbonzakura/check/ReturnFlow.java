package io.senbonzakura.check;

import io.senbonzakura.ast.Stmt;
import java.util.List;

/**
 * Works out whether a function body cannot finish without returning, for functions that must return
 * values. It looks only at the shape of the code, not at values: {@code loop true { }} counts as a
 * loop that may end.
 */
final class ReturnFlow {

  private ReturnFlow() {}

  /** Whether running these statements always ends in a {@code return}, or never ends. */
  static boolean alwaysReturns(List<Stmt> statements) {
    return statements.stream().anyMatch(ReturnFlow::alwaysReturns);
  }

  private static boolean alwaysReturns(Stmt stmt) {
    return switch (stmt) {
      case Stmt.Return s -> s.condition() == null;
      case Stmt.Block s -> alwaysReturns(s.statements());
      case Stmt.If s ->
          s.otherwise() != null && alwaysReturns(s.then()) && alwaysReturns(s.otherwise());
      // A loop with no condition ends only by a break, and never ending counts as returning.
      case Stmt.Loop s -> s.condition() == null && !breaks(s.body().statements());
      default -> false;
    };
  }

  /** Whether these statements hold a {@code break} out of the loop whose body they are. */
  private static boolean breaks(List<Stmt> statements) {
    return statements.stream().anyMatch(ReturnFlow::breaks);
  }

  private static boolean breaks(Stmt stmt) {
    return switch (stmt) {
      case Stmt.Break _ -> true;
      case Stmt.Block s -> breaks(s.statements());
      case Stmt.If s -> breaks(s.then()) || (s.otherwise() != null && breaks(s.otherwise()));
      // A break inside a nested loop or function leaves that, not this loop.
      default -> false;
    };
  }
}
