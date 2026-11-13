package io.senbonzakura.lexer;

/**
 * Scans number literals: {@code 123}, {@code 1_000}, {@code 0x1F}, {@code 0o17}, {@code 0b1010},
 * {@code 1.5}, {@code 1e10}, {@code 1.5e-3}.
 *
 * <p>A {@code .} only starts a fraction when a digit follows, so {@code 1.foo} and {@code 1..2}
 * leave it for the next token.
 */
final class ScanNumber {

  private ScanNumber() {}

  /** Scans the number at the cursor, which must be on a decimal digit. */
  static Token literal(Cursor c) {
    int start = c.pos();
    if (c.peek() == '0') {
      switch (c.peek(1)) {
        case 'x', 'X' -> {
          return prefixedInteger(c, start, 16, "hexadecimal");
        }
        case 'o', 'O' -> {
          return prefixedInteger(c, start, 8, "octal");
        }
        case 'b', 'B' -> {
          return prefixedInteger(c, start, 2, "binary");
        }
        default -> {}
      }
    }

    var kind = Token.Kind.INT_LITERAL;
    digits(c, 10);

    if (c.peek() == '.' && c.peekDigit(1, 10) >= 0) {
      kind = Token.Kind.FLOAT_LITERAL;
      c.bump(); // '.'
      digits(c, 10);
    }

    boolean missingExponentDigits = false;
    if (c.peek() == 'e' || c.peek() == 'E') {
      kind = Token.Kind.FLOAT_LITERAL;
      c.bump(); // 'e'
      if (c.peek() == '+' || c.peek() == '-') {
        c.bump();
      }
      missingExponentDigits = digits(c, 10) == 0;
    }

    int suffixStart = c.pos();
    if (c.skipIdentifierChars()) {
      c.report(suffixStart, "invalid suffix on number literal");
    } else if (missingExponentDigits) {
      c.report(start, "expected digits in exponent");
    }
    return c.token(kind, start, 0);
  }

  private static Token prefixedInteger(Cursor c, int start, int radix, String name) {
    c.bump(2); // '0x', '0o' or '0b'
    int count = digits(c, radix);

    int suffixStart = c.pos();
    boolean hasSuffix = c.skipIdentifierChars();
    if (count == 0) {
      c.report(start, "expected " + name + " digits after '" + c.text(start, start + 2) + "'");
    } else if (hasSuffix) {
      c.report(suffixStart, "invalid " + name + " digit or suffix");
    }
    return c.token(Token.Kind.INT_LITERAL, start, 0);
  }

  /** Consumes digits of {@code radix} and {@code _} separators, and returns how many digits. */
  private static int digits(Cursor c, int radix) {
    int count = 0;
    while (true) {
      if (c.peek() == '_') {
        c.bump();
      } else if (c.peekDigit(radix) >= 0) {
        c.bump();
        count++;
      } else {
        return count;
      }
    }
  }
}
