package io.senbonzakura.lexer;

import io.senbonzakura.lexer.Token.Diagnostic;
import java.util.ArrayList;
import java.util.List;
import java.util.function.IntPredicate;

// The lexer's only mutable state: the text, the current position and the diagnostics so far.
// Scanners move it forward and build tokens from it.
final class Cursor {

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

  String text(int start, int end) {
    return text.substring(start, end);
  }

  // Skips n UTF-16 units. Only for characters already checked with peek() to be ASCII.
  void bump(int n) {
    pos += n;
  }

  void bump() {
    pos++;
  }

  // Consumes one code point, so a surrogate pair (e.g. an emoji) is never split in two.
  void advance() {
    if (!isAtEnd()) {
      pos += Character.charCount(text.codePointAt(pos));
    }
  }

  // Consumes \n, \r\n or a lone \r as a single line break.
  void skipLineBreak() {
    if (peek() == '\r') {
      pos++;
    }
    if (peek() == '\n') {
      pos++;
    }
  }

  // Consumes code points while they match. Returns whether it consumed any.
  boolean skipWhile(IntPredicate matches) {
    int start = pos;
    while (!isAtEnd()) {
      int c = text.codePointAt(pos);
      if (!matches.test(c)) {
        break;
      }
      pos += Character.charCount(c);
    }
    return pos > start;
  }

  Token token(Token.Kind kind, int start, int flags) {
    return new Token(kind, start, pos, flags);
  }

  // Reports a problem with the text from start up to the current position.
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
