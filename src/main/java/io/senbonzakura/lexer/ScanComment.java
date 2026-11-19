package io.senbonzakura.lexer;

/**
 * Skips comments: {@code // ...} up to the end of the line, and {@code /* ... *}{@code /}, which
 * may span lines and nests, so commenting out code that already holds a block comment works.
 * Comments produce no tokens; {@link Lexer} records them in the next token's flags.
 */
final class ScanComment {

  private ScanComment() {}

  static boolean atComment(Cursor c) {
    return c.peek() == '/' && (c.peek(1) == '/' || c.peek(1) == '*');
  }

  /**
   * Skips the comment at the cursor, which must be {@link #atComment on one}, and returns whether
   * it held a line break. A line comment stops before its line break, so it never holds one.
   */
  static boolean skip(Cursor c) {
    assert atComment(c) : "not on a comment at " + c.pos();
    if (c.peek(1) == '/') {
      line(c);
      return false;
    }
    return block(c);
  }

  private static void line(Cursor c) {
    c.bump(2); // '//'
    while (!c.isAtEnd() && c.peek() != '\n' && c.peek() != '\r') {
      c.advance();
    }
  }

  /** An unterminated comment is reported at its outermost opening, and runs to the end. */
  private static boolean block(Cursor c) {
    int start = c.pos();
    c.bump(2); // '/*'
    int depth = 1;
    boolean lineBreak = false;
    while (depth > 0) {
      if (c.isAtEnd()) {
        c.report(start, start + 2, "unterminated block comment");
        break;
      }
      if (c.lookingAt("/*")) {
        c.bump(2);
        depth++;
      } else if (c.lookingAt("*/")) {
        c.bump(2);
        depth--;
      } else {
        lineBreak |= c.peek() == '\n' || c.peek() == '\r';
        c.advance();
      }
    }
    return lineBreak;
  }
}
