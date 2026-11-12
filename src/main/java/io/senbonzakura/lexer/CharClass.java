package io.senbonzakura.lexer;

final class CharClass {

  private CharClass() {}

  // ASCII only: Character.digit would also accept digits from other scripts.
  // Returns Integer.MAX_VALUE for anything that isn't a digit, so `digitValue(c) < radix` works.
  static int digitValue(int c) {
    if (c >= '0' && c <= '9') return c - '0';
    if (c >= 'a' && c <= 'z') return c - 'a' + 10;
    if (c >= 'A' && c <= 'Z') return c - 'A' + 10;
    return Integer.MAX_VALUE;
  }

  static boolean isIdentifierPart(int codePoint) {
    return codePoint == '_' || Character.isLetterOrDigit(codePoint);
  }
}
