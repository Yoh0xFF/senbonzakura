package io.senbonzakura.parser;

import io.senbonzakura.ast.Expr;
import io.senbonzakura.ast.Stmt;
import io.senbonzakura.ast.Type;
import io.senbonzakura.lexer.Token;
import java.util.ArrayList;
import java.util.List;

/**
 * Parses statements: an expression followed by {@code ;}, a {@code var}, an {@code if} with an
 * optional {@code else}, a {@code loop}, a {@code continue}, a {@code break}, a {@code return}, or
 * a block of statements in {@code {}}. Statements inside a block come from {@link
 * Parser#statement(TokenCursor)}.
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

  /** The cursor must be on {@code var}. */
  static Stmt varStatement(TokenCursor c) {
    assert c.at(Token.Kind.VAR) : c.peek();
    int problemsBefore = c.diagnostics().size();
    Token keyword = c.advance();
    return varRest(c, problemsBefore, keyword, binding(c, false));
  }

  /**
   * {@code name: type}, the part of a declarator before its initializer, and all of a {@code loop
   * var x in} variable. {@code type} is {@code null} when there is none.
   */
  private record Binding(String name, Type type, int start, int end) {}

  /**
   * Parses a name and an optional {@code : type}. A keyword or literal where the name should be is
   * consumed, unless it can follow the name; {@code inLoop} adds {@code in} to those.
   */
  private static Binding binding(TokenCursor c, boolean inLoop) {
    int start = c.peek().start();
    String name = "";
    int end = start;
    if (c.at(Token.Kind.IDENTIFIER)) {
      Token token = c.advance();
      name = c.text(token);
      end = token.end();
    } else {
      c.report(c.peek(), "expected variable name, found " + c.describe(c.peek()));
      if (!canFollowName(c) && !(inLoop && c.at(Token.Kind.IN))) {
        end = c.advance().end();
      }
    }

    Type type = null;
    if (c.at(Token.Kind.COLON)) {
      c.advance();
      type = Parser.type(c);
      end = type.end();
    }
    return new Binding(name, type, start, end);
  }

  private static boolean canFollowName(TokenCursor c) {
    return switch (c.peek().kind()) {
      case COLON, EQ, COMMA, SEMICOLON, RIGHT_BRACE, EOF -> true;
      default -> false;
    };
  }

  /**
   * Parses the rest of a {@code var} after its first name and type: that declarator's optional
   * {@code = initializer}, any more declarators after commas, then {@code ;}.
   */
  private static Stmt.Var varRest(TokenCursor c, int problemsBefore, Token keyword, Binding first) {
    var declarators = new ArrayList<Stmt.Declarator>();
    Binding binding = first;
    while (true) {
      Expr initializer = null;
      int end = binding.end();
      if (c.at(Token.Kind.EQ)) {
        c.advance();
        initializer = Parser.expression(c);
        end = initializer.end();
      }
      declarators.add(
          new Stmt.Declarator(binding.name(), binding.type(), initializer, binding.start(), end));

      if (!c.at(Token.Kind.COMMA)) {
        break;
      }
      c.advance();
      binding = binding(c, false);
    }

    int end =
        semicolon(
            c, problemsBefore, declarators.getLast().end(), "expected ';' after var statement");
    return new Stmt.Var(declarators, keyword.start(), end);
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
   *   <li>{@code loop var x in xs { ... }} runs once for each item of {@code xs}.
   *   <li>{@code loop var i = 0; i < n; i++ { ... }} runs the {@code var} once, then the body while
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
    if (c.at(Token.Kind.VAR)) {
      return varLoop(c, loop, problemsBefore);
    }
    Expr condition = Parser.expression(c);
    Stmt.Block body = body(c, problemsBefore, condition.end(), "after loop condition");
    return new Stmt.Loop(condition, body, loop.start(), body.end());
  }

  /**
   * Tells {@code loop var x in} from {@code loop var i = 0;} by what follows the name and type.
   * Anything else is reported as a missing {@code in}, and the loop is parsed as one over what
   * follows.
   */
  private static Stmt varLoop(TokenCursor c, Token loop, int problemsBefore) {
    assert c.at(Token.Kind.VAR) : c.peek();
    Token keyword = c.advance();
    Binding binding = binding(c, true);
    if (c.at(Token.Kind.EQ) || c.at(Token.Kind.COMMA) || c.at(Token.Kind.SEMICOLON)) {
      return stepLoop(c, loop, problemsBefore, varRest(c, problemsBefore, keyword, binding));
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
        binding.name(), binding.type(), iterable, body, loop.start(), body.end());
  }

  private static Stmt stepLoop(TokenCursor c, Token loop, int problemsBefore, Stmt.Var init) {
    Expr condition = Parser.expression(c);
    semicolon(c, problemsBefore, condition.end(), "expected ';' after loop condition");
    Expr step = Parser.expression(c);
    Stmt.Block body = body(c, problemsBefore, step.end(), "after loop step");
    return new Stmt.LoopStep(init, condition, step, body, loop.start(), body.end());
  }

  /**
   * The cursor must be on {@code fn}. Parses {@code fn name(a: int, b: int): int, bool { ... }},
   * where the {@code :} and return types are left out for a function that returns nothing. The
   * parameter list allows a trailing comma. A missing {@code :} before a parameter's type or the
   * return types is reported, and the types after it still parsed.
   */
  static Stmt function(TokenCursor c) {
    assert c.at(Token.Kind.FN) : c.peek();
    int problemsBefore = c.diagnostics().size();
    Token fn = c.advance();
    int end = fn.end();

    String name = "";
    if (c.at(Token.Kind.IDENTIFIER)) {
      Token token = c.advance();
      name = c.text(token);
      end = token.end();
    } else {
      c.report(c.peek(), "expected function name, found " + c.describe(c.peek()));
      if (!canFollowFunctionName(c)) {
        end = c.advance().end();
      }
    }

    var parameters = new ArrayList<Stmt.Parameter>();
    if (c.at(Token.Kind.LEFT_PAREN)) {
      end = parameters(c, parameters);
    } else if (c.diagnostics().size() == problemsBefore) {
      c.report(c.peek(), "expected '(' after function name, found " + c.describe(c.peek()));
    }

    var returnTypes = new ArrayList<Type>();
    if (c.at(Token.Kind.COLON) || Parser.atType(c)) {
      if (c.at(Token.Kind.COLON)) {
        c.advance();
      } else if (c.diagnostics().size() == problemsBefore) {
        c.report(c.peek(), "expected ':' before return type, found " + c.describe(c.peek()));
      }
      returnTypes.add(Parser.type(c));
      while (c.at(Token.Kind.COMMA)) {
        c.advance();
        returnTypes.add(Parser.type(c));
      }
      end = returnTypes.getLast().end();
    }

    Stmt.Block body = body(c, problemsBefore, end, "after function signature");
    return new Stmt.Function(name, parameters, returnTypes, body, fn.start(), body.end());
  }

  private static boolean canFollowFunctionName(TokenCursor c) {
    return switch (c.peek().kind()) {
      case LEFT_PAREN, COLON, LEFT_BRACE, RIGHT_BRACE, SEMICOLON, EOF -> true;
      default -> false;
    };
  }

  /**
   * The cursor must be on {@code (}. Adds the parameters to {@code into} and returns where the list
   * ends: past the {@code )}, or after the last parameter if it is missing.
   */
  private static int parameters(TokenCursor c, List<Stmt.Parameter> into) {
    assert c.at(Token.Kind.LEFT_PAREN) : c.peek();
    Token open = c.advance();
    while (!c.at(Token.Kind.RIGHT_PAREN) && !atParametersEnd(c)) {
      int problemsBefore = c.diagnostics().size();
      into.add(parameter(c));
      if (c.at(Token.Kind.COMMA)) {
        c.advance();
      } else if (!c.at(Token.Kind.RIGHT_PAREN)
          && !atParametersEnd(c)
          && c.diagnostics().size() == problemsBefore) {
        c.report(c.peek(), "expected ',' or ')', found " + c.describe(c.peek()));
      }
    }

    if (!c.at(Token.Kind.RIGHT_PAREN)) {
      c.report(c.peek(), "expected ')', found " + c.describe(c.peek()));
      return into.isEmpty() ? open.end() : into.getLast().end();
    }
    return c.advance().end();
  }

  /**
   * Parses {@code name: type}. It consumes a token unless the cursor is on one {@link
   * #canFollowParameterName} accepts, each of which the parameter loop consumes or stops at, so the
   * loop always moves.
   */
  private static Stmt.Parameter parameter(TokenCursor c) {
    int problemsBefore = c.diagnostics().size();
    int start = c.peek().start();
    String name = "";
    int end = start;
    if (c.at(Token.Kind.IDENTIFIER)) {
      Token token = c.advance();
      name = c.text(token);
      end = token.end();
    } else {
      c.report(c.peek(), "expected parameter name, found " + c.describe(c.peek()));
      if (!canFollowParameterName(c)) {
        end = c.advance().end();
      }
    }

    Type type;
    if (c.at(Token.Kind.COLON)) {
      c.advance();
      type = Parser.type(c);
    } else {
      if (c.diagnostics().size() == problemsBefore) {
        c.report(c.peek(), "expected ':' after parameter name, found " + c.describe(c.peek()));
      }
      type = Parser.atType(c) ? Parser.type(c) : new Type.Error(end, end);
    }
    return new Stmt.Parameter(name, type, start, type.end());
  }

  private static boolean canFollowParameterName(TokenCursor c) {
    return c.at(Token.Kind.COLON)
        || c.at(Token.Kind.COMMA)
        || c.at(Token.Kind.RIGHT_PAREN)
        || atParametersEnd(c);
  }

  /** A token a parameter list cannot go past, so a missing {@code )} does not swallow the body. */
  private static boolean atParametersEnd(TokenCursor c) {
    return switch (c.peek().kind()) {
      case LEFT_BRACE, RIGHT_BRACE, SEMICOLON, EOF -> true;
      default -> false;
    };
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

  /** The cursor must be on {@code continue}. */
  static Stmt continueStatement(TokenCursor c) {
    assert c.at(Token.Kind.CONTINUE) : c.peek();
    Jump jump = jump(c, "continue", false);
    return new Stmt.Continue(jump.condition(), jump.start(), jump.end());
  }

  /** The cursor must be on {@code break}. */
  static Stmt breakStatement(TokenCursor c) {
    assert c.at(Token.Kind.BREAK) : c.peek();
    Jump jump = jump(c, "break", false);
    return new Stmt.Break(jump.condition(), jump.start(), jump.end());
  }

  /** The cursor must be on {@code return}. */
  static Stmt returnStatement(TokenCursor c) {
    assert c.at(Token.Kind.RETURN) : c.peek();
    Jump jump = jump(c, "return", true);
    return new Stmt.Return(jump.values(), jump.condition(), jump.start(), jump.end());
  }

  /**
   * What {@code continue}, {@code break} and {@code return} share. {@code values} is empty and
   * {@code condition} null when left out.
   */
  private record Jump(List<Expr> values, Expr condition, int start, int end) {}

  /**
   * Parses the keyword spelled {@code word}, values separated by commas if {@code takesValue}, an
   * optional {@code if condition}, then {@code ;}. The condition is a plain expression, not an
   * {@code if} statement, so it takes no block and no {@code else}.
   *
   * <p>Values are optional too. None follow when the next token is {@code ;}, {@code if}, or one
   * that closes the enclosing block, so {@code return }} reports the missing {@code ;} rather than
   * a missing expression.
   */
  private static Jump jump(TokenCursor c, String word, boolean takesValue) {
    int problemsBefore = c.diagnostics().size();
    Token keyword = c.advance();
    int end = keyword.end();
    String message = "expected ';' or 'if' after '" + word + "'";

    var values = new ArrayList<Expr>();
    if (takesValue && !endsJump(c)) {
      values.add(Parser.expression(c));
      while (c.at(Token.Kind.COMMA)) {
        c.advance();
        values.add(Parser.expression(c));
      }
      end = values.getLast().end();
      message = "expected ';' or 'if' after " + word + " value";
    }

    Expr condition = null;
    if (c.at(Token.Kind.IF)) {
      c.advance();
      condition = Parser.expression(c);
      end = condition.end();
      message = "expected ';' after " + word + " condition";
    }

    end = semicolon(c, problemsBefore, end, message);
    return new Jump(values, condition, keyword.start(), end);
  }

  private static boolean endsJump(TokenCursor c) {
    return switch (c.peek().kind()) {
      case SEMICOLON, IF, RIGHT_BRACE, EOF -> true;
      default -> false;
    };
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
