package io.senbonzakura.lexer;

public class Lexer {

  private static final char EOF = '\uFFFF';
  private final String source;
  private final int length;
  private int cursor;

  public Lexer(String source) {
    this.source = source;
    this.length = source.length();
    this.cursor = 0;
  }

  public Token nextToken() {
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

    return Token.EndOfFile;
  }

  private char peek() {
    return cursor >= length ? EOF : source.charAt(cursor);
  }

  private char peekNext() {
    return cursor + 1 >= length ? EOF : source.charAt(cursor + 1);
  }
}
