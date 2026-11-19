package io.senbonzakura.parser;

import io.senbonzakura.ast.Expr;
import io.senbonzakura.ast.Stmt;
import io.senbonzakura.lexer.Token;
import java.util.ArrayList;
import java.util.List;

/**
 * Parses statements: an expression followed by {@code ;}, a {@code let}, an {@code if} with an
 * optional {@code else}, a {@code loop}, a {@code continue}, or a block of statements in {@code
 * {}}. Statements inside a block come from {@link Parser#statement(TokenCursor)}.
 *
 * <p>A missing {@code ;} or body is reported only when the rest of the statement had no problems,
 * so one mistake does not produce two diagnostics. A missing body becomes an empty block, so the
 * tokens after it start the next statement.
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
    int problemsBefore = c.diagnostics().size();
    return letRest(c, problemsBefore, binding(c, false));
  }

  /** {@code let mut name}, the part a {@code let} statement shares with {@code loop let}. */
  private record Binding(boolean mutable, String name, int start, int end) {}

  /**
   * Parses {@code let}, an optional {@code mut} and the name. A keyword or literal where the name
   * should be is consumed, unless it can follow the name; {@code inLoop} adds {@code in} to those.
   */
  private static Binding binding(TokenCursor c, boolean inLoop) {
    assert c.at(Token.Kind.LET) : c.peek();
    Token let = c.advance();
    boolean mutable = c.at(Token.Kind.MUT);
    int end = mutable ? c.advance().end() : let.end();

    if (c.at(Token.Kind.IDENTIFIER)) {
      Token token = c.advance();
      return new Binding(mutable, c.text(token), let.start(), token.end());
    }
    c.report(c.peek(), "expected variable name, found " + c.describe(c.peek()));
    if (!canFollowName(c) && !(inLoop && c.at(Token.Kind.IN))) {
      end = c.advance().end();
    }
    return new Binding(mutable, "", let.start(), end);
  }

  private static boolean canFollowName(TokenCursor c) {
    return switch (c.peek().kind()) {
      case EQ, SEMICOLON, RIGHT_BRACE, EOF -> true;
      default -> false;
    };
  }

  /** Parses what follows the name of a {@code let}: an optional {@code = initializer}, then ;. */
  private static Stmt.Let letRest(TokenCursor c, int problemsBefore, Binding binding) {
    int end = binding.end();
    Expr initializer = null;
    if (c.at(Token.Kind.EQ)) {
      c.advance();
      initializer = Parser.expression(c);
      end = initializer.end();
    }

    end = semicolon(c, problemsBefore, end, "expected ';' after let statement");
    return new Stmt.Let(binding.mutable(), binding.name(), initializer, binding.start(), end);
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
   * must be a block, optionally followed by {@code else} and a block or another {@code if}. An
   * {@code else} after a missing body is still parsed.
   */
  static Stmt ifStatement(TokenCursor c) {
    assert c.at(Token.Kind.IF) : c.peek();
    int problemsBefore = c.diagnostics().size();
    Token ifToken = c.advance();
    Expr condition = Parser.expression(c);
    Stmt.Block then = body(c, problemsBefore, condition.end(), "after if condition");

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

  /**
   * The cursor must be on {@code loop}. What follows it picks the form, and none has parentheses:
   *
   * <ul>
   *   <li>{@code loop { ... }} runs until something leaves it.
   *   <li>{@code loop let x in xs { ... }} runs once for each item of {@code xs}.
   *   <li>{@code loop let i = 0; i < n; i++ { ... }} runs the {@code let} once, then the body while
   *       the condition holds, running the step after each pass.
   *   <li>{@code loop condition { ... }} runs while the condition holds.
   * </ul>
   */
  static Stmt loop(TokenCursor c) {
    assert c.at(Token.Kind.LOOP) : c.peek();
    int problemsBefore = c.diagnostics().size();
    Token loop = c.advance();

    if (c.at(Token.Kind.LEFT_BRACE)) {
      Stmt.Block body = block(c);
      return new Stmt.Loop(null, body, loop.start(), body.end());
    }
    if (c.at(Token.Kind.LET)) {
      return letLoop(c, loop, problemsBefore);
    }
    Expr condition = Parser.expression(c);
    Stmt.Block body = body(c, problemsBefore, condition.end(), "after loop condition");
    return new Stmt.Loop(condition, body, loop.start(), body.end());
  }

  /**
   * Tells {@code loop let x in} from {@code loop let i = 0;} by what follows the name. Anything
   * else is reported as a missing {@code in}, and the loop is parsed as one over what follows.
   */
  private static Stmt letLoop(TokenCursor c, Token loop, int problemsBefore) {
    Binding binding = binding(c, true);
    if (c.at(Token.Kind.EQ) || c.at(Token.Kind.SEMICOLON)) {
      return stepLoop(c, loop, problemsBefore, letRest(c, problemsBefore, binding));
    }

    if (c.at(Token.Kind.IN)) {
      c.advance();
    } else if (c.diagnostics().size() == problemsBefore) {
      c.report(c.peek(), "expected 'in' or '=' after loop variable, found " + c.describe(c.peek()));
    }
    // A '{' here is the body, so the missing 'in' or name is the only problem reported.
    Expr iterable =
        c.at(Token.Kind.LEFT_BRACE) && c.diagnostics().size() > problemsBefore
            ? new Expr.Error(c.peek().start(), c.peek().start())
            : Parser.expression(c);
    Stmt.Block body = body(c, problemsBefore, iterable.end(), "after loop iterable");
    return new Stmt.LoopIn(
        binding.mutable(), binding.name(), iterable, body, loop.start(), body.end());
  }

  private static Stmt stepLoop(TokenCursor c, Token loop, int problemsBefore, Stmt.Let init) {
    Expr condition = Parser.expression(c);
    semicolon(c, problemsBefore, condition.end(), "expected ';' after loop condition");
    Expr step = Parser.expression(c);
    Stmt.Block body = body(c, problemsBefore, step.end(), "after loop step");
    return new Stmt.LoopStep(init, condition, step, body, loop.start(), body.end());
  }

  /**
   * Parses the block a statement requires, or reports it missing, as {@code expected '{' <where>},
   * and stands in an empty block at {@code end}.
   */
  private static Stmt.Block body(TokenCursor c, int problemsBefore, int end, String where) {
    if (c.at(Token.Kind.LEFT_BRACE)) {
      return block(c);
    }
    if (c.diagnostics().size() == problemsBefore) {
      c.report(c.peek(), "expected '{' " + where + ", found " + c.describe(c.peek()));
    }
    return emptyBlockAt(end);
  }

  /**
   * The cursor must be on {@code continue}. The optional {@code if condition} is a plain
   * expression, not an {@code if} statement, so it takes no block and no {@code else}.
   */
  static Stmt continueStatement(TokenCursor c) {
    assert c.at(Token.Kind.CONTINUE) : c.peek();
    int problemsBefore = c.diagnostics().size();
    Token keyword = c.advance();
    if (!c.at(Token.Kind.IF)) {
      int end =
          semicolon(c, problemsBefore, keyword.end(), "expected ';' or 'if' after 'continue'");
      return new Stmt.Continue(null, keyword.start(), end);
    }
    c.advance();
    Expr condition = Parser.expression(c);
    int end =
        semicolon(c, problemsBefore, condition.end(), "expected ';' after continue condition");
    return new Stmt.Continue(condition, keyword.start(), end);
  }

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
