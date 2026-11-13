package io.senbonzakura.lexer;

import io.senbonzakura.lexer.Token.Diagnostic;
import java.util.Collections;
import java.util.List;
import java.util.Objects;

/**
 * Splits a {@link SourceFile} into {@link Token}s, one {@link #nextToken()} call at a time.
 *
 * <p>Lexical errors never throw. They are collected in {@link #diagnostics()} and lexing continues.
 */
public class Lexer {

  private final SourceFile source;
  private final Cursor cursor;

  public Lexer(SourceFile source) {
    this.source = Objects.requireNonNull(source, "source");
    this.cursor = new Cursor(source.text());
  }

  public SourceFile source() {
    return source;
  }

  /**
   * Returns the problems found so far, in the order found, as a read-only view that keeps growing.
   */
  public List<Diagnostic> diagnostics() {
    return Collections.unmodifiableList(cursor.diagnostics());
  }

  /**
   * Returns the next token. Once the input is used up, every call returns the same zero-width EOF
   * token.
   */
  public Token nextToken() {
    skipWhitespace();

    return switch (cursor.peek()) {
      case Cursor.EOF -> Token.eof(cursor.pos(), 0);
      case '0', '1', '2', '3', '4', '5', '6', '7', '8', '9' -> ScanNumber.literal(cursor);
      case '"' -> ScanQuoted.string(cursor);
      case '`' -> ScanQuoted.multiline(cursor);
      case '\'' -> ScanQuoted.character(cursor);
      default -> {
        if (cursor.atIdentifierStart()) {
          yield ScanIdentifier.identifierOrKeyword(cursor);
        }
        yield ScanOperator.operator(cursor).orElseGet(this::unknownCharacter);
      }
    };
  }

  private Token unknownCharacter() {
    // Consumes one character nothing else recognized, so lexing always makes progress.
    int start = cursor.pos();
    cursor.advance();
    cursor.report(start, "unexpected character");
    return cursor.token(Token.Kind.ERROR, start, 0);
  }

  private void skipWhitespace() {
    while (true) {
      switch (cursor.peek()) {
        case ' ', '\t', '\n', '\r' -> cursor.bump();
        default -> {
          return;
        }
      }
    }
  }
}
