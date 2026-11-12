package io.senbonzakura.lexer;

import static io.senbonzakura.lexer.CharClass.digitValue;

// Numbers: 123, 1_000, 0x1F, 0o17, 0b1010, 1.5, 1e10, 1.5e-3.
// A '.' only starts a fraction when a digit follows, so `1.foo` and `1..2` leave the '.' alone.
final class ScanNumber {

  private ScanNumber() {}

  // The cursor is on a decimal digit.
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

    if (c.peek() == '.' && digitValue(c.peek(1)) < 10) {
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
    if (c.skipWhile(CharClass::isIdentifierPart)) {
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
    boolean hasSuffix = c.skipWhile(CharClass::isIdentifierPart);
    if (count == 0) {
      c.report(start, "expected " + name + " digits after '" + c.text(start, start + 2) + "'");
    } else if (hasSuffix) {
      c.report(suffixStart, "invalid " + name + " digit or suffix");
    }
    return c.token(Token.Kind.INT_LITERAL, start, 0);
  }

  // Consumes digits of the given radix and '_' separators. Returns how many digits it saw.
  private static int digits(Cursor c, int radix) {
    int count = 0;
    while (true) {
      int ch = c.peek();
      if (ch == '_') {
        c.bump();
      } else if (digitValue(ch) < radix) {
        c.bump();
        count++;
      } else {
        return count;
      }
    }
  }
}
