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

  /** The EOF token once reached, so every later call returns it with the same flags. */
  private Token eof;

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
    if (eof != null) {
      return eof;
    }
    int flags = skipTrivia();
    Token token = scan();
    if (flags != 0) {
      token = new Token(token.kind(), token.start(), token.end(), token.flags() | flags);
    }
    if (token.kind() == Token.Kind.EOF) {
      eof = token;
    }
    return token;
  }

  private Token scan() {
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
    assert !cursor.isAtEnd() : "no character to consume at " + cursor.pos();
    int start = cursor.pos();
    cursor.advance();
    cursor.report(start, "unexpected character");
    return cursor.token(Token.Kind.ERROR, start, 0);
  }

  /**
   * Skips whitespace and comments, and returns the {@link Token#SPACE_BEFORE} and {@link
   * Token#NEWLINE_BEFORE} flags they earn the token after them.
   */
  private int skipTrivia() {
    int start = cursor.pos();
    boolean lineBreak = false;
    while (true) {
      int ch = cursor.peek();
      if (ch == ' ' || ch == '\t') {
        cursor.bump();
      } else if (ch == '\n' || ch == '\r') {
        lineBreak = true;
        cursor.bump();
      } else if (ScanComment.atComment(cursor)) {
        lineBreak |= ScanComment.skip(cursor);
      } else {
        break;
      }
    }

    int flags = cursor.pos() > start ? Token.SPACE_BEFORE : 0;
    return lineBreak ? flags | Token.NEWLINE_BEFORE : flags;
  }
}
