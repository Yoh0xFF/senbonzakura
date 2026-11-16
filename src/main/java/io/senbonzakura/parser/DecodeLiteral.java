package io.senbonzakura.parser;

import java.util.OptionalLong;

/**
 * Turns the source text of literal tokens into values.
 *
 * <p>The lexer has already checked literal syntax and reported what was wrong, so decoding never
 * fails: a malformed literal gets a best-effort value, and anything that cannot be decoded becomes
 * {@link #REPLACEMENT}. Problems the lexer cannot see, like an integer too big for 64 bits, are
 * left to the caller to report.
 */
final class DecodeLiteral {

  static final int REPLACEMENT = 0xFFFD;

  private DecodeLiteral() {}

  /**
   * Decodes an integer like {@code 1_000}, {@code 0xFF}, {@code 0o17} or {@code 0b1010}, or returns
   * empty when it does not fit in a {@code long}. Stops at the first character that is not a digit,
   * so an invalid suffix is ignored.
   */
  static OptionalLong integer(String text) {
    int radix = 10;
    int i = 0;
    if (text.length() >= 2 && text.charAt(0) == '0') {
      radix =
          switch (text.charAt(1)) {
            case 'x', 'X' -> 16;
            case 'o', 'O' -> 8;
            case 'b', 'B' -> 2;
            default -> 10;
          };
      if (radix != 10) {
        i = 2;
      }
    }

    long value = 0;
    for (; i < text.length(); i++) {
      char ch = text.charAt(i);
      if (ch == '_') {
        continue;
      }
      int digit = digit(ch, radix);
      if (digit < 0) {
        break;
      }
      if (value > (Long.MAX_VALUE - digit) / radix) {
        return OptionalLong.empty();
      }
      value = value * radix + digit;
    }
    return OptionalLong.of(value);
  }

  /**
   * Decodes a float like {@code 1.5}, {@code 1_0.5} or {@code 1.5e-3}. Returns infinity when it is
   * too large for a {@code double}, and {@code 0.0} when it is malformed.
   */
  static double floating(String text) {
    var digits = new StringBuilder(text.length());
    for (int i = 0; i < text.length(); i++) {
      char ch = text.charAt(i);
      if (ch == '_') {
        continue;
      }
      boolean partOfNumber =
          (ch >= '0' && ch <= '9') || ch == '.' || ch == 'e' || ch == 'E' || ch == '+' || ch == '-';
      if (!partOfNumber) {
        break; // an invalid suffix
      }
      digits.append(ch);
    }
    try {
      return Double.parseDouble(digits.toString());
    } catch (NumberFormatException e) {
      return 0.0; // e.g. "1e" with no exponent digits
    }
  }

  /**
   * Decodes a {@code "..."} or {@code `...`} string, applying its escapes.
   *
   * @param terminated whether the text ends with a closing quote
   */
  static String string(String text, boolean terminated) {
    return unescape(text.substring(1, terminated ? text.length() - 1 : text.length()));
  }

  /**
   * Decodes a {@code '...'} literal into one code point, or {@link #REPLACEMENT} when it does not
   * hold exactly one.
   *
   * @param terminated whether the text ends with a closing quote
   */
  static int character(String text, boolean terminated) {
    String value = string(text, terminated);
    return value.codePointCount(0, value.length()) == 1 ? value.codePointAt(0) : REPLACEMENT;
  }

  private static String unescape(String s) {
    if (s.indexOf('\\') < 0) {
      return s;
    }
    var out = new StringBuilder(s.length());
    int i = 0;
    while (i < s.length()) {
      char ch = s.charAt(i++);
      if (ch != '\\') {
        out.append(ch);
        continue;
      }
      if (i == s.length()) {
        break; // a backslash at the end of an unterminated literal
      }
      char escaped = s.charAt(i++);
      switch (escaped) {
        case 'n' -> out.append('\n');
        case 't' -> out.append('\t');
        case 'r' -> out.append('\r');
        case '0' -> out.append('\0');
        case 'u' -> i = unicodeEscape(s, i, out);
        case '\r' -> {
          // A line continuation; \r\n counts as one line break.
          if (i < s.length() && s.charAt(i) == '\n') {
            i++;
          }
        }
        case '\n' -> {} // a line continuation
        default -> out.append(escaped); // \\ \" \' \`, or an unknown escape kept as its character
      }
    }
    return out.toString();
  }

  /**
   * Decodes the braces and hex digits of a unicode escape, starting at {@code i} just after the
   * {@code u}, and returns the index after them.
   */
  private static int unicodeEscape(String s, int i, StringBuilder out) {
    if (i >= s.length() || s.charAt(i) != '{') {
      out.appendCodePoint(REPLACEMENT);
      return i;
    }
    i++;

    int value = 0;
    int count = 0;
    int digit;
    while (i < s.length() && (digit = digit(s.charAt(i), 16)) >= 0) {
      if (count < 6) {
        value = value * 16 + digit;
      }
      count++;
      i++;
    }

    if (i >= s.length() || s.charAt(i) != '}') {
      out.appendCodePoint(REPLACEMENT);
      return i;
    }
    i++;

    boolean valid =
        count >= 1
            && count <= 6
            && value <= Character.MAX_CODE_POINT
            && (value < Character.MIN_SURROGATE || value > Character.MAX_SURROGATE);
    out.appendCodePoint(valid ? value : REPLACEMENT);
    return i;
  }

  /** Returns the value of an ASCII digit in {@code radix}, or -1 if it is not one. */
  private static int digit(char ch, int radix) {
    int value;
    if (ch >= '0' && ch <= '9') value = ch - '0';
    else if (ch >= 'a' && ch <= 'z') value = ch - 'a' + 10;
    else if (ch >= 'A' && ch <= 'Z') value = ch - 'A' + 10;
    else return -1;

    return value < radix ? value : -1;
  }
}
