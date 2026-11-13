package io.senbonzakura.lexer;

import io.senbonzakura.lexer.Token.Diagnostic;
import java.util.ArrayList;
import java.util.List;

/**
 * The lexer's only mutable state: the text, the current position and the diagnostics so far.
 * Scanners move it forward and build tokens from it.
 */
final class Cursor {

  /** What {@link #peek(int)} returns past the end. No {@code char} has this value. */
  static final int EOF = -1;

  private final String text;
  private final int length;
  private final List<Diagnostic> diagnostics = new ArrayList<>();

  private int pos;

  Cursor(String text) {
    this.text = text;
    this.length = text.length();
  }

  int pos() {
    return pos;
  }

  boolean isAtEnd() {
    return pos >= length;
  }

  int peek() {
    return peek(0);
  }

  int peek(int ahead) {
    int i = pos + ahead;
    return i < length ? text.charAt(i) : EOF;
  }

  int peekDigit(int radix) {
    return peekDigit(0, radix);
  }

  /**
   * Returns the value of the digit {@code ahead} chars past the cursor in {@code radix}, or -1 if
   * there is none. Digits above 9 are the letters {@code a}-{@code z} in either case.
   *
   * <p>ASCII only: {@link Character#digit(int, int)} would also accept digits from other scripts.
   */
  int peekDigit(int ahead, int radix) {
    int ch = peek(ahead);

    int value;
    if (ch >= '0' && ch <= '9') value = ch - '0';
    else if (ch >= 'a' && ch <= 'z') value = ch - 'a' + 10;
    else if (ch >= 'A' && ch <= 'Z') value = ch - 'A' + 10;
    else return -1;

    return value < radix ? value : -1;
  }

  String text(int start, int end) {
    return text.substring(start, end);
  }

  /** Skips {@code n} UTF-16 units. Only for characters {@link #peek(int)} has shown to be ASCII. */
  void bump(int n) {
    pos += n;
  }

  /** Skips one UTF-16 unit. Only for a character {@link #peek()} has shown to be ASCII. */
  void bump() {
    pos++;
  }

  /** Consumes one code point, so a surrogate pair (e.g. an emoji) is never split in two. */
  void advance() {
    if (!isAtEnd()) {
      pos += Character.charCount(text.codePointAt(pos));
    }
  }

  /** Consumes {@code \n}, {@code \r\n} or a lone {@code \r} as one line break, if one is next. */
  void skipLineBreak() {
    if (peek() == '\r') {
      pos++;
    }
    if (peek() == '\n') {
      pos++;
    }
  }

  /** Returns whether the cursor is on a letter or {@code _}, which can start an identifier. */
  boolean atIdentifierStart() {
    if (isAtEnd()) {
      return false;
    }
    int c = text.codePointAt(pos);
    return c == '_' || Character.isLetter(c);
  }

  /** Consumes letters, digits and {@code _}, and returns whether there were any. */
  boolean skipIdentifierChars() {
    int start = pos;
    while (!isAtEnd()) {
      int c = text.codePointAt(pos);
      if (c != '_' && !Character.isLetterOrDigit(c)) {
        break;
      }
      pos += Character.charCount(c);
    }
    return pos > start;
  }

  /** Returns a token from {@code start} up to the current position. */
  Token token(Token.Kind kind, int start, int flags) {
    return new Token(kind, start, pos, flags);
  }

  /** Records a diagnostic from {@code start} up to the current position. */
  void report(int start, String message) {
    report(start, pos, message);
  }

  void report(int start, int end, String message) {
    diagnostics.add(new Diagnostic(start, end, message));
  }

  List<Diagnostic> diagnostics() {
    return diagnostics;
  }
}
