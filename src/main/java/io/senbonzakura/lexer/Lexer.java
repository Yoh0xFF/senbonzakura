package io.senbonzakura.lexer;

import java.util.Objects;

public class Lexer {

  private static final int EOF = -1;

  private final SourceFile source;
  private final String text;
  private final int length;

  private int cursor;

  public Lexer(SourceFile source) {
    this.source = Objects.requireNonNull(source, "source");
    this.text = source.text();
    this.length = text.length();
  }

  public SourceFile source() {
    return source;
  }

  public Token nextToken() {
    skipWhitespace();

    int start = cursor;
    return switch (peek()) {
      case EOF -> Token.eof(start, 0);
      // TODO: case '\'' -> charLiteral(start);
      // TODO: case '"' -> stringLiteral(start);
      default -> unknownCharacter(start);
    };
  }

  private Token unknownCharacter(int start) {
    // Consumes one character nothing else recognized, so lexing always makes progress.
    advance();
    return new Token(Token.Kind.ERROR, start, cursor, 0);
  }

  private boolean isAtEnd() {
    return cursor >= length;
  }

  private int peek() {
    return isAtEnd() ? EOF : text.charAt(cursor);
  }

  private int peekNext() {
    return cursor + 1 >= length ? EOF : text.charAt(cursor + 1);
  }

  private void advance() {
    // Consumes one code point, so a surrogate pair (e.g. an emoji) is never split in two.
    if (!isAtEnd()) {
      cursor += Character.charCount(text.codePointAt(cursor));
    }
  }

  private void skipWhitespace() {
    while (true) {
      switch (peek()) {
        case ' ', '\t', '\n', '\r' -> cursor++;
        default -> {
          return;
        }
      }
    }
  }
}
