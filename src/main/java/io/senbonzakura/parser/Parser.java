package io.senbonzakura.parser;

import io.senbonzakura.ast.Expr;
import io.senbonzakura.ast.Stmt;
import io.senbonzakura.lexer.Lexer;
import io.senbonzakura.lexer.Token;
import io.senbonzakura.lexer.Token.Diagnostic;
import java.util.Collections;
import java.util.List;
import java.util.Objects;

/**
 * Builds {@link Stmt} and {@link Expr} trees from the tokens of a {@link Lexer}, one {@link
 * #parseStatement()} or {@link #parseExpression()} call at a time.
 *
 * <p>Like the lexer, the parser never throws on bad input. Problems go to {@link #diagnostics()}
 * and the part that failed becomes an {@link Expr.Error} node. Problems the lexer found stay in
 * {@link Lexer#diagnostics()} and are not reported again.
 */
public class Parser {

  private final TokenCursor cursor;

  public Parser(Lexer lexer) {
    this.cursor = new TokenCursor(Objects.requireNonNull(lexer, "lexer"));
  }

  /**
   * Returns the problems found so far, in the order found, as a read-only view that keeps growing.
   */
  public List<Diagnostic> diagnostics() {
    return Collections.unmodifiableList(cursor.diagnostics());
  }

  public boolean atEnd() {
    return cursor.atEnd();
  }

  /**
   * Parses one statement. Always consumes at least one token, unless the input is used up, so a
   * loop calling it until {@link #atEnd()} always finishes.
   */
  public Stmt parseStatement() {
    Token first = cursor.peek();
    Stmt stmt = statement(cursor);
    cursor.skipIfStuck(first);
    return stmt;
  }

  /** Like {@link #parseStatement()}, always consumes at least one token unless at the end. */
  public Expr parseExpression() {
    Token first = cursor.peek();
    Expr expr = expression(cursor);
    cursor.skipIfStuck(first);
    return expr;
  }

  /** Picks a parser from the token that starts the statement. */
  static Stmt statement(TokenCursor c) {
    return switch (c.peek().kind()) {
      case LEFT_BRACE -> ParseStatement.block(c);
      case LET -> ParseStatement.let(c);
      case IF -> ParseStatement.ifStatement(c);
      default -> ParseStatement.expression(c);
    };
  }

  static Expr expression(TokenCursor c) {
    return ParseOperator.expression(c);
  }

  /** Parses the part of an expression between infix operators: prefix operators, then a postfix. */
  static Expr operand(TokenCursor c) {
    return switch (c.peek().kind()) {
      case MINUS, BANG -> ParseOperator.prefix(c);
      default -> ParsePostfix.postfix(c);
    };
  }

  /** Picks a parser from the token that starts the primary: a literal, a name or a group. */
  static Expr primary(TokenCursor c) {
    return switch (c.peek().kind()) {
      case INT_LITERAL -> ParseLiteral.integer(c);
      case FLOAT_LITERAL -> ParseLiteral.floating(c);
      case STRING_LITERAL -> ParseLiteral.string(c);
      case CHAR_LITERAL -> ParseLiteral.character(c);
      case TRUE, FALSE -> ParseLiteral.bool(c);
      case IDENTIFIER -> ParseIdentifier.identifier(c);
      case LEFT_PAREN -> ParseGroup.parenthesized(c);
      case ERROR -> lexerError(c);
      case RIGHT_PAREN, RIGHT_BRACKET, LEFT_BRACE, RIGHT_BRACE, COMMA, SEMICOLON, EOF ->
          missingOperand(c);
      default -> unexpectedToken(c);
    };
  }

  private static Expr lexerError(TokenCursor c) {
    // The lexer already reported this character.
    assert c.at(Token.Kind.ERROR) : c.peek();
    Token token = c.advance();
    return new Expr.Error(token.start(), token.end());
  }

  private static Expr missingOperand(TokenCursor c) {
    // Leaves the token in place, so an enclosing group, argument list or statement can match it.
    // A '{' is left for the statement it belongs to, like the body after an if condition.
    Token token = c.peek();
    c.report(token, "expected expression, found " + c.describe(token));
    return new Expr.Error(token.start(), token.start());
  }

  private static Expr unexpectedToken(TokenCursor c) {
    // Consumes a token nothing else recognized, so parsing always makes progress.
    assert !c.atEnd() : c.peek();
    Token token = c.advance();
    c.report(token, "expected expression, found " + c.describe(token));
    return new Expr.Error(token.start(), token.end());
  }
}
