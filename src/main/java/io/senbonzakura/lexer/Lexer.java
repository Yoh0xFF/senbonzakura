package io.senbonzakura.lexer;

public class Lexer {

  private static final char EOF = '\uFFFF';

  private final String source;
  private final int length;

  private int cursor;
  private int line, column;

  public Lexer(String source) {
    this.source = source;
    this.length = source.length();
    this.cursor = this.line = this.column = 0;
  }

  public Token nextToken() {
    skipWhitespace();

    var currentChar = this.peek();
    switch (currentChar) {
      case '\'':
        break;
      case '"':
        break;
      default:
        break;
    }
    cursor++;

    return Token.eof(length, 0);
  }

  private char peek() {
    return cursor >= length ? EOF : source.charAt(cursor);
  }

  private char peekNext() {
    return cursor + 1 >= length ? EOF : source.charAt(cursor + 1);
  }

  private void skipWhitespace() {
    while (true) {
      var currentChar = peek();
      switch (currentChar) {
        case ' ':
        case '\t':
          cursor++;
          break;
        case '\n':
        case '\r':
          line++;
          column = 0;
          cursor++;
          break;
        default:
          return;
      }
    }
  }
}
