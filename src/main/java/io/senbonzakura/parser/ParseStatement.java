package io.senbonzakura.parser;

import io.senbonzakura.ast.Expr;
import io.senbonzakura.ast.Stmt;
import io.senbonzakura.lexer.Token;
import java.util.ArrayList;

/**
 * Parses statements: an expression followed by {@code ;}, or a block of statements in {@code {}}.
 * Statements inside a block come from {@link Parser#statement(TokenCursor)}.
 */
final class ParseStatement {

  private ParseStatement() {}

  /**
   * A missing {@code ;} is reported only when the expression itself had no problems, so one mistake
   * does not produce two diagnostics.
   */
  static Stmt expression(TokenCursor c) {
    int problemsBefore = c.diagnostics().size();
    Expr expr = Parser.expression(c);
    if (!c.at(Token.Kind.SEMICOLON)) {
      if (c.diagnostics().size() == problemsBefore) {
        c.report(expr.end(), expr.end(), "expected ';' after expression");
      }
      return new Stmt.Expression(expr, expr.start(), expr.end());
    }
    Token semicolon = c.advance();
    return new Stmt.Expression(expr, expr.start(), semicolon.end());
  }

  /**
   * The cursor must be on an opening brace. A missing closing brace is reported at the end of
   * input.
   */
  static Stmt block(TokenCursor c) {
    Token open = c.advance();
    var statements = new ArrayList<Stmt>();
    while (!c.at(Token.Kind.RIGHT_BRACE) && !c.atEnd()) {
      Token first = c.peek();
      statements.add(Parser.statement(c));
      c.skipIfStuck(first);
    }

    if (c.atEnd()) {
      c.report(c.peek(), "expected '}', found end of input");
      int end = statements.isEmpty() ? open.end() : statements.getLast().end();
      return new Stmt.Block(statements, open.start(), end);
    }
    Token close = c.advance();
    return new Stmt.Block(statements, open.start(), close.end());
  }
}
