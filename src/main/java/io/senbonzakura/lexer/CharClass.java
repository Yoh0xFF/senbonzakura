package io.senbonzakura.lexer;

final class CharClass {

  private CharClass() {}

  /**
   * Returns the value of an ASCII digit or letter ({@code 0}-{@code 9}, then {@code a}-{@code z} in
   * either case as 10-35), or {@link Integer#MAX_VALUE} for anything else, so {@code digitValue(c)
   * < radix} tests for a digit.
   *
   * <p>ASCII only: {@link Character#digit(int, int)} would also accept digits from other scripts.
   */
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
