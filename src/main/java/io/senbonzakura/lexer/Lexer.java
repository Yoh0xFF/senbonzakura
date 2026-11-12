package io.senbonzakura.lexer;

import java.util.ArrayList;
import java.util.Collections;
import java.util.List;
import java.util.Objects;

public class Lexer {

  private static final int EOF = -1;

  private final SourceFile source;
  private final String text;
  private final int length;
  private final List<Diagnostic> diagnostics = new ArrayList<>();

  private int cursor;

  public Lexer(SourceFile source) {
    this.source = Objects.requireNonNull(source, "source");
    this.text = source.text();
    this.length = text.length();
  }

  public SourceFile source() {
    return source;
  }

  public List<Diagnostic> diagnostics() {
    return Collections.unmodifiableList(diagnostics);
  }

  public Token nextToken() {
    skipWhitespace();

    int start = cursor;
    return switch (peek()) {
      case EOF -> Token.eof(start, 0);
      case '0', '1', '2', '3', '4', '5', '6', '7', '8', '9' -> number(start);
      case '"' -> string(start);
      case '`' -> multilineString(start);
      case '\'' -> character(start);
      default -> unknownCharacter(start);
    };
  }

  // Numbers: 123, 1_000, 0x1F, 0o17, 0b1010, 1.5, 1e10, 1.5e-3.
  // A '.' only starts a fraction when a digit follows, so `1.foo` and `1..2` leave the '.' alone.
  private Token number(int start) {
    if (peek() == '0') {
      switch (peekNext()) {
        case 'x', 'X' -> {
          return prefixedInteger(start, 16, "hexadecimal");
        }
        case 'o', 'O' -> {
          return prefixedInteger(start, 8, "octal");
        }
        case 'b', 'B' -> {
          return prefixedInteger(start, 2, "binary");
        }
        default -> {}
      }
    }

    var kind = Token.Kind.INT_LITERAL;
    digits(10);

    if (peek() == '.' && digitValue(peekNext()) < 10) {
      kind = Token.Kind.FLOAT_LITERAL;
      cursor++; // '.'
      digits(10);
    }

    boolean missingExponentDigits = false;
    if (peek() == 'e' || peek() == 'E') {
      kind = Token.Kind.FLOAT_LITERAL;
      cursor++; // 'e'
      if (peek() == '+' || peek() == '-') {
        cursor++;
      }
      missingExponentDigits = digits(10) == 0;
    }

    int suffixStart = cursor;
    if (skipIdentifierChars()) {
      report(suffixStart, cursor, "invalid suffix on number literal");
    } else if (missingExponentDigits) {
      report(start, cursor, "expected digits in exponent");
    }
    return new Token(kind, start, cursor, 0);
  }

  private Token prefixedInteger(int start, int radix, String name) {
    cursor += 2; // '0x', '0o' or '0b'
    int count = digits(radix);

    int suffixStart = cursor;
    boolean hasSuffix = skipIdentifierChars();
    if (count == 0) {
      report(
          start,
          cursor,
          "expected " + name + " digits after '" + text.substring(start, start + 2) + "'");
    } else if (hasSuffix) {
      report(suffixStart, cursor, "invalid " + name + " digit or suffix");
    }
    return new Token(Token.Kind.INT_LITERAL, start, cursor, 0);
  }

  // Consumes digits of the given radix and '_' separators. Returns how many digits it saw.
  private int digits(int radix) {
    int count = 0;
    while (true) {
      int c = peek();
      if (c == '_') {
        cursor++;
      } else if (digitValue(c) < radix) {
        cursor++;
        count++;
      } else {
        return count;
      }
    }
  }

  // Strings: "...", on one line, with the escapes handled by escape().
  private Token string(int start) {
    cursor++; // opening '"'
    int flags = 0;
    while (true) {
      int c = peek();
      if (c == EOF || c == '\n' || c == '\r') {
        report(start, cursor, "unterminated string literal");
        flags |= Token.UNTERMINATED;
        break;
      }
      if (c == '"') {
        cursor++;
        break;
      }
      if (c == '\\') {
        flags |= Token.HAS_ESCAPES;
        escape(false);
      } else {
        advance();
      }
    }
    return new Token(Token.Kind.STRING_LITERAL, start, cursor, flags);
  }

  // Multi-line strings: `...`, which may span lines. Same escapes as "...", plus a backslash at the
  // end of a line, which joins it to the next one. `${` is plain text: there is no interpolation.
  private Token multilineString(int start) {
    cursor++; // opening '`'
    int flags = 0;
    while (true) {
      int c = peek();
      if (c == EOF) {
        // Point at the opening backtick: the token itself runs to the end of the file.
        report(start, start + 1, "unterminated multi-line string literal");
        flags |= Token.UNTERMINATED;
        break;
      }
      if (c == '`') {
        cursor++;
        break;
      }
      if (c == '\\') {
        flags |= Token.HAS_ESCAPES;
        escape(true);
      } else {
        advance();
      }
    }
    return new Token(Token.Kind.STRING_LITERAL, start, cursor, flags);
  }

  // Characters: 'a', '\n', or a unicode escape. Exactly one character between the quotes.
  private Token character(int start) {
    cursor++; // opening '\''
    int flags = 0;
    int count = 0;
    while (true) {
      int c = peek();
      if (c == EOF || c == '\n' || c == '\r') {
        report(start, cursor, "unterminated character literal");
        flags |= Token.UNTERMINATED;
        break;
      }
      if (c == '\'') {
        cursor++;
        if (count == 0) {
          report(start, cursor, "empty character literal");
        } else if (count > 1) {
          report(start, cursor, "character literal must contain exactly one character");
        }
        break;
      }
      if (c == '\\') {
        flags |= Token.HAS_ESCAPES;
        escape(false);
      } else {
        advance();
      }
      count++;
    }
    return new Token(Token.Kind.CHAR_LITERAL, start, cursor, flags);
  }

  // Escapes: \n \t \r \0 \\ \" \' \` and a unicode escape: backslash, 'u', then 1 to 6 hex digits
  // in braces, e.g. u{1F600}. In multi-line strings a backslash before a line break continues the
  // line.
  private void escape(boolean multiline) {
    int start = cursor;
    cursor++; // '\\'
    switch (peek()) {
      case 'n', 't', 'r', '0', '\\', '"', '\'', '`' -> cursor++;
      case 'u' -> unicodeEscape(start);
      case EOF -> {} // the caller reports the unterminated literal
      case '\n', '\r' -> {
        if (multiline) {
          skipLineBreak();
        } // otherwise the caller reports the unterminated literal
      }
      default -> {
        advance();
        report(start, cursor, "unknown escape sequence");
      }
    }
  }

  private void unicodeEscape(int start) {
    cursor++; // 'u'
    if (peek() != '{') {
      report(start, cursor, "expected '{' after '\\u'");
      return;
    }
    cursor++;

    int value = 0;
    int count = 0;
    while (digitValue(peek()) < 16) {
      if (count < 6) {
        value = value * 16 + digitValue(peek());
      }
      count++;
      cursor++;
    }

    if (peek() != '}') {
      report(start, cursor, "unterminated unicode escape, expected '}'");
      return;
    }
    cursor++;

    if (count == 0 || count > 6) {
      report(start, cursor, "unicode escape must have 1 to 6 hex digits");
    } else if (value > Character.MAX_CODE_POINT
        || (value >= Character.MIN_SURROGATE && value <= Character.MAX_SURROGATE)) {
      report(start, cursor, "unicode escape is not a valid code point");
    }
  }

  // Consumes \n, \r\n or a lone \r as a single line break.
  private void skipLineBreak() {
    if (peek() == '\r') {
      cursor++;
    }
    if (peek() == '\n') {
      cursor++;
    }
  }

  private Token unknownCharacter(int start) {
    // Consumes one character nothing else recognized, so lexing always makes progress.
    advance();
    report(start, cursor, "unexpected character");
    return new Token(Token.Kind.ERROR, start, cursor, 0);
  }

  // Consumes letters, digits and '_' stuck to the end of a literal. Returns whether there were any.
  private boolean skipIdentifierChars() {
    int start = cursor;
    while (!isAtEnd()) {
      int c = text.codePointAt(cursor);
      if (c != '_' && !Character.isLetterOrDigit(c)) {
        break;
      }
      cursor += Character.charCount(c);
    }
    return cursor > start;
  }

  // ASCII only: Character.digit would also accept digits from other scripts.
  private static int digitValue(int c) {
    if (c >= '0' && c <= '9') return c - '0';
    if (c >= 'a' && c <= 'z') return c - 'a' + 10;
    if (c >= 'A' && c <= 'Z') return c - 'A' + 10;
    return Integer.MAX_VALUE;
  }

  private void report(int start, int end, String message) {
    diagnostics.add(new Diagnostic(start, end, message));
  }

  private boolean isAtEnd() {
    return cursor >= length;
  }

  private int peek() {
    return isAtEnd() ? EOF : text.charAt(cursor);
  }

  private int peekNext() {
    return cursor + 1 >= length ? EOF : text.charAt(cursor + 1);
  }

  private void advance() {
    // Consumes one code point, so a surrogate pair (e.g. an emoji) is never split in two.
    if (!isAtEnd()) {
      cursor += Character.charCount(text.codePointAt(cursor));
    }
  }

  private void skipWhitespace() {
    while (true) {
      switch (peek()) {
        case ' ', '\t', '\n', '\r' -> cursor++;
        default -> {
          return;
        }
      }
    }
  }

  public record Diagnostic(int start, int end, String message) {}
}
