package io.senbonzakura.parser;

import io.senbonzakura.ast.Expr;
import io.senbonzakura.lexer.Lexer;
import io.senbonzakura.lexer.Token;
import io.senbonzakura.lexer.Token.Diagnostic;
import java.util.Collections;
import java.util.List;
import java.util.Objects;

/**
 * Builds {@link Expr} trees from the tokens of a {@link Lexer}, one {@link #parseExpression()} call
 * at a time.
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
   * Parses one expression. Always consumes at least one token, unless the input is used up, so a
   * loop calling it until {@link #atEnd()} always finishes.
   */
  public Expr parseExpression() {
    Token first = cursor.peek();
    Expr expr = expression(cursor);
    if (cursor.peek() == first && !cursor.atEnd()) {
      cursor.advance(); // a stray closing delimiter, already reported
    }
    return expr;
  }

  static Expr expression(TokenCursor c) {
    return ParseOperator.expression(c);
  }

  /**
   * Parses one operand: the part of an expression between infix operators. Picks a parser from the
   * token at the cursor.
   */
  static Expr operand(TokenCursor c) {
    return switch (c.peek().kind()) {
      case INT_LITERAL -> ParseLiteral.integer(c);
      case FLOAT_LITERAL -> ParseLiteral.floating(c);
      case STRING_LITERAL -> ParseLiteral.string(c);
      case CHAR_LITERAL -> ParseLiteral.character(c);
      case TRUE, FALSE -> ParseLiteral.bool(c);
      case IDENTIFIER -> ParseIdentifier.identifier(c);
      case LEFT_PAREN -> ParseGroup.parenthesized(c);
      case MINUS, BANG -> ParseOperator.prefix(c);
      case ERROR -> lexerError(c);
      case RIGHT_PAREN, RIGHT_BRACKET, RIGHT_BRACE, EOF -> missingOperand(c);
      default -> unexpectedToken(c);
    };
  }

  private static Expr lexerError(TokenCursor c) {
    // The lexer already reported this character.
    Token token = c.advance();
    return new Expr.Error(token.start(), token.end());
  }

  private static Expr missingOperand(TokenCursor c) {
    // Leaves the token in place, so an enclosing group can still match it.
    Token token = c.peek();
    c.report(token, "expected expression, found " + c.describe(token));
    return new Expr.Error(token.start(), token.start());
  }

  private static Expr unexpectedToken(TokenCursor c) {
    // Consumes a token nothing else recognized, so parsing always makes progress.
    Token token = c.advance();
    c.report(token, "expected expression, found " + c.describe(token));
    return new Expr.Error(token.start(), token.end());
  }
}
