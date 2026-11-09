package io.senbonzakura.lexer;

public class Lexer {

  private static final char END_OF_FILE = '.';
  private final String source;
  private int current;
  private int lookahead;

  public Lexer(String source) {
    this.source = source;
    this.current = 0;
    this.lookahead = source.isEmpty() ? 0 : 1;
  }

  public Token nextToken() {
    var currentChar = this.currentChar();

    switch (currentChar) {
      case '\'':
        break;
      case '"':
        break;
      default:
        break;
    }

    return Token.EndOfFile;
  }

  private char currentChar() {
    if (current == source.length()) {
      return END_OF_FILE;
    }
    current += 1;
    lookahead += (lookahead == source.length() ? 0 : 1);
    return source.charAt(current - 1);
  }

  private char nextChar() {
    if (lookahead == source.length()) {
      return END_OF_FILE;
    }
    return source.charAt(lookahead);
  }
}
