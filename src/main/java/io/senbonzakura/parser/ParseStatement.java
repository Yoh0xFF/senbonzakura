package io.senbonzakura.parser;

import io.senbonzakura.ast.Expr;
import io.senbonzakura.ast.Stmt;
import io.senbonzakura.lexer.Token;
import java.util.ArrayList;
import java.util.List;

/**
 * Parses statements: an expression followed by {@code ;}, a {@code let}, an {@code if} with an
 * optional {@code else}, or a block of statements in {@code {}}. Statements inside a block come
 * from {@link Parser#statement(TokenCursor)}.
 *
 * <p>A missing {@code ;} is reported only when the rest of the statement had no problems, so one
 * mistake does not produce two diagnostics.
 */
final class ParseStatement {

  private ParseStatement() {}

  static Stmt expression(TokenCursor c) {
    int problemsBefore = c.diagnostics().size();
    Expr expr = Parser.expression(c);
    int end = semicolon(c, problemsBefore, expr.end(), "expected ';' after expression");
    return new Stmt.Expression(expr, expr.start(), end);
  }

  /** The cursor must be on {@code let}. */
  static Stmt let(TokenCursor c) {
    assert c.at(Token.Kind.LET) : c.peek();
    int problemsBefore = c.diagnostics().size();
    Token let = c.advance();
    boolean mutable = c.at(Token.Kind.MUT);
    int end = mutable ? c.advance().end() : let.end();

    String name = "";
    if (c.at(Token.Kind.IDENTIFIER)) {
      Token token = c.advance();
      name = c.text(token);
      end = token.end();
    } else {
      c.report(c.peek(), "expected variable name, found " + c.describe(c.peek()));
      if (!canFollowName(c)) {
        end = c.advance().end(); // a keyword or literal where the name should be
      }
    }

    Expr initializer = null;
    if (c.at(Token.Kind.EQ)) {
      c.advance();
      initializer = Parser.expression(c);
      end = initializer.end();
    }

    end = semicolon(c, problemsBefore, end, "expected ';' after let statement");
    return new Stmt.Let(mutable, name, initializer, let.start(), end);
  }

  private static boolean canFollowName(TokenCursor c) {
    return switch (c.peek().kind()) {
      case EQ, SEMICOLON, RIGHT_BRACE, EOF -> true;
      default -> false;
    };
  }

  /**
   * Consumes the {@code ;} that ends a statement and returns the statement's end: past the {@code
   * ;}, or {@code end} if it is missing.
   */
  private static int semicolon(TokenCursor c, int problemsBefore, int end, String message) {
    if (c.at(Token.Kind.SEMICOLON)) {
      return c.advance().end();
    }
    if (c.diagnostics().size() == problemsBefore) {
      c.report(end, end, message);
    }
    return end;
  }

  /**
   * The cursor must be on {@code if}. There are no parentheses around the condition, and the body
   * must be a block, optionally followed by {@code else} and a block or another {@code if}.
   *
   * <p>A missing body becomes an empty block, so the tokens after it start the next statement. A
   * missing body after the condition is reported only when the condition had no problems, and an
   * {@code else} after it is still parsed.
   */
  static Stmt ifStatement(TokenCursor c) {
    assert c.at(Token.Kind.IF) : c.peek();
    int problemsBefore = c.diagnostics().size();
    Token ifToken = c.advance();
    Expr condition = Parser.expression(c);

    Stmt.Block then;
    if (c.at(Token.Kind.LEFT_BRACE)) {
      then = block(c);
    } else {
      if (c.diagnostics().size() == problemsBefore) {
        c.report(c.peek(), "expected '{' after if condition, found " + c.describe(c.peek()));
      }
      then = emptyBlockAt(condition.end());
    }

    if (!c.at(Token.Kind.ELSE)) {
      return new Stmt.If(condition, then, null, ifToken.start(), then.end());
    }
    Stmt otherwise = elseBranch(c);
    return new Stmt.If(condition, then, otherwise, ifToken.start(), otherwise.end());
  }

  private static Stmt elseBranch(TokenCursor c) {
    assert c.at(Token.Kind.ELSE) : c.peek();
    Token elseToken = c.advance();
    return switch (c.peek().kind()) {
      case LEFT_BRACE -> block(c);
      case IF -> ifStatement(c);
      default -> {
        c.report(c.peek(), "expected '{' or 'if' after 'else', found " + c.describe(c.peek()));
        yield emptyBlockAt(elseToken.end());
      }
    };
  }

  /** Stands in for a body that was missing. */
  private static Stmt.Block emptyBlockAt(int offset) {
    return new Stmt.Block(List.of(), offset, offset);
  }

  /**
   * The cursor must be on an opening brace. A missing closing brace is reported at the end of
   * input.
   */
  static Stmt.Block block(TokenCursor c) {
    assert c.at(Token.Kind.LEFT_BRACE) : c.peek();
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
