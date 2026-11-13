package io.senbonzakura.lexer;

import static io.senbonzakura.lexer.Cursor.EOF;

/**
 * Scans literals between quotes, {@code "..."}, {@code `...`} and {@code '...'}, and the escapes
 * they share.
 */
final class ScanQuoted {

  private ScanQuoted() {}

  /** Scans a {@code "..."} string, which must end on the line it starts. */
  static Token string(Cursor c) {
    int start = c.pos();
    c.bump(); // opening '"'
    int flags = 0;
    while (true) {
      int ch = c.peek();
      if (ch == EOF || ch == '\n' || ch == '\r') {
        c.report(start, "unterminated string literal");
        flags |= Token.UNTERMINATED;
        break;
      }
      if (ch == '"') {
        c.bump();
        break;
      }
      if (ch == '\\') {
        flags |= Token.HAS_ESCAPES;
        escape(c, false);
      } else {
        c.advance();
      }
    }
    return c.token(Token.Kind.STRING_LITERAL, start, flags);
  }

  /**
   * Scans a {@code `...`} string, which may span lines. A backslash before a line break joins the
   * two lines. There is no {@code ${...}} interpolation; it is plain text.
   */
  static Token multiline(Cursor c) {
    int start = c.pos();
    c.bump(); // opening '`'
    int flags = 0;
    while (true) {
      int ch = c.peek();
      if (ch == EOF) {
        // Point at the opening backtick: the token itself runs to the end of the file.
        c.report(start, start + 1, "unterminated multi-line string literal");
        flags |= Token.UNTERMINATED;
        break;
      }
      if (ch == '`') {
        c.bump();
        break;
      }
      if (ch == '\\') {
        flags |= Token.HAS_ESCAPES;
        escape(c, true);
      } else {
        c.advance();
      }
    }
    return c.token(Token.Kind.STRING_LITERAL, start, flags);
  }

  /** Scans a {@code '...'} literal, which must hold exactly one character or escape. */
  static Token character(Cursor c) {
    int start = c.pos();
    c.bump(); // opening '\''
    int flags = 0;
    int count = 0;
    while (true) {
      int ch = c.peek();
      if (ch == EOF || ch == '\n' || ch == '\r') {
        c.report(start, "unterminated character literal");
        flags |= Token.UNTERMINATED;
        break;
      }
      if (ch == '\'') {
        c.bump();
        if (count == 0) {
          c.report(start, "empty character literal");
        } else if (count > 1) {
          c.report(start, "character literal must contain exactly one character");
        }
        break;
      }
      if (ch == '\\') {
        flags |= Token.HAS_ESCAPES;
        escape(c, false);
      } else {
        c.advance();
      }
      count++;
    }
    return c.token(Token.Kind.CHAR_LITERAL, start, flags);
  }

  /**
   * Consumes one escape, starting at the backslash: {@code \n \t \r \0 \\ \" \' \`}, or a unicode
   * escape of 1 to 6 hex digits in braces. With {@code multiline}, a backslash before a line break
   * continues the line.
   */
  private static void escape(Cursor c, boolean multiline) {
    int start = c.pos();
    c.bump(); // '\\'
    switch (c.peek()) {
      case 'n', 't', 'r', '0', '\\', '"', '\'', '`' -> c.bump();
      case 'u' -> unicodeEscape(c, start);
      case EOF -> {} // the caller reports the unterminated literal
      case '\n', '\r' -> {
        if (multiline) {
          c.skipLineBreak();
        } // otherwise the caller reports the unterminated literal
      }
      default -> {
        c.advance();
        c.report(start, "unknown escape sequence");
      }
    }
  }

  private static void unicodeEscape(Cursor c, int start) {
    c.bump(); // 'u'
    if (c.peek() != '{') {
      c.report(start, "expected '{' after '\\u'");
      return;
    }
    c.bump();

    int value = 0;
    int count = 0;
    int digit;
    while ((digit = c.peekDigit(16)) >= 0) {
      if (count < 6) {
        value = value * 16 + digit;
      }
      count++;
      c.bump();
    }

    if (c.peek() != '}') {
      c.report(start, "unterminated unicode escape, expected '}'");
      return;
    }
    c.bump();

    if (count == 0 || count > 6) {
      c.report(start, "unicode escape must have 1 to 6 hex digits");
    } else if (value > Character.MAX_CODE_POINT
        || (value >= Character.MIN_SURROGATE && value <= Character.MAX_SURROGATE)) {
      c.report(start, "unicode escape is not a valid code point");
    }
  }
}
