package io.senbonzakura.lexer;

import java.util.Objects;

public record Token(Kind kind, int start, int end, int flags) {

  // A line break appears between the previous token and this one.
  public static final int NEWLINE_BEFORE = 1;
  // Whitespace or a comment appears between the previous token and this one.
  public static final int SPACE_BEFORE = 1 << 1;
  // A string, char literal or block comment that never closes before the end of input.
  public static final int UNTERMINATED = 1 << 2;
  // A string or char literal containing escape sequences, so it must be unescaped before use.
  public static final int HAS_ESCAPES = 1 << 3;

  public Token {
    Objects.requireNonNull(kind, "kind");
    if (start < 0 || end < start) {
      throw new IllegalArgumentException("invalid token range " + start + ".." + end);
    }
  }

  public static Token eof(int offset, int flags) {
    return new Token(Kind.EOF, offset, offset, flags);
  }

  public int length() {
    return end - start;
  }

  public boolean has(int flag) {
    return (flags & flag) != 0;
  }

  public record Diagnostic(int start, int end, String message) {}

  public enum Kind {
    // Names and literals
    IDENTIFIER,
    INT_LITERAL,
    FLOAT_LITERAL,
    STRING_LITERAL,
    CHAR_LITERAL,

    // Keywords

    // Punctuation and operators

    // Characters the lexer could not make sense of. A diagnostic is recorded separately.
    ERROR,

    // Zero-width token at the end of the input.
    EOF,
  }
}
