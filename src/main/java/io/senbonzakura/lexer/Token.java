package io.senbonzakura.lexer;

import static io.senbonzakura.lexer.Token.Kind.Category.KEYWORD;
import static io.senbonzakura.lexer.Token.Kind.Category.OPERATOR;

import java.util.Objects;

/**
 * A token: its kind, the source range it covers, and flags.
 *
 * <p>Tokens hold offsets, not text. {@link SourceFile#text(Token)} and {@link
 * SourceFile#locate(int)} turn them into text and positions.
 *
 * @param start offset of the first character
 * @param end offset just past the last character; equal to {@code start} for {@link Kind#EOF}
 * @param flags the flag constants below, combined with {@code |}
 */
public record Token(Kind kind, int start, int end, int flags) {

  /** A line break appears between the previous token and this one. */
  public static final int NEWLINE_BEFORE = 1;

  /** Whitespace or a comment appears between the previous token and this one. */
  public static final int SPACE_BEFORE = 1 << 1;

  /** The literal has no closing quote; a diagnostic has already been reported. */
  public static final int UNTERMINATED = 1 << 2;

  /** The literal contains escape sequences, so its value differs from its source text. */
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

  /**
   * A problem found while lexing, covering offsets {@code start} up to (not including) {@code end}.
   */
  public record Diagnostic(int start, int end, String message) {}

  /**
   * What a token is. Keywords and operators each get their own kind, so the parser never compares
   * strings.
   */
  public enum Kind {
    // Names and literals
    IDENTIFIER,
    INT_LITERAL,
    FLOAT_LITERAL,
    STRING_LITERAL,
    CHAR_LITERAL,

    // Keywords
    FN("fn", KEYWORD),
    LET("let", KEYWORD),
    MUT("mut", KEYWORD),
    IF("if", KEYWORD),
    ELSE("else", KEYWORD),
    WHILE("while", KEYWORD),
    FOR("for", KEYWORD),
    IN("in", KEYWORD),
    RETURN("return", KEYWORD),
    BREAK("break", KEYWORD),
    CONTINUE("continue", KEYWORD),
    TRUE("true", KEYWORD),
    FALSE("false", KEYWORD),

    // Punctuation
    LEFT_PAREN("(", OPERATOR),
    RIGHT_PAREN(")", OPERATOR),
    LEFT_BRACE("{", OPERATOR),
    RIGHT_BRACE("}", OPERATOR),
    LEFT_BRACKET("[", OPERATOR),
    RIGHT_BRACKET("]", OPERATOR),
    COMMA(",", OPERATOR),
    SEMICOLON(";", OPERATOR),
    COLON(":", OPERATOR),
    DOT(".", OPERATOR),
    DOT_DOT("..", OPERATOR),
    ARROW("->", OPERATOR),

    // Arithmetic
    PLUS("+", OPERATOR),
    MINUS("-", OPERATOR),
    STAR("*", OPERATOR),
    SLASH("/", OPERATOR),
    PERCENT("%", OPERATOR),

    // Assignment
    EQ("=", OPERATOR),
    PLUS_EQ("+=", OPERATOR),
    MINUS_EQ("-=", OPERATOR),
    STAR_EQ("*=", OPERATOR),
    SLASH_EQ("/=", OPERATOR),
    PERCENT_EQ("%=", OPERATOR),

    // Comparison
    EQ_EQ("==", OPERATOR),
    BANG_EQ("!=", OPERATOR),
    LESS("<", OPERATOR),
    LESS_EQ("<=", OPERATOR),
    GREATER(">", OPERATOR),
    GREATER_EQ(">=", OPERATOR),

    // Logical
    AND_AND("&&", OPERATOR),
    OR_OR("||", OPERATOR),
    BANG("!", OPERATOR),

    /** A character that starts no token; a diagnostic has already been reported. */
    ERROR,

    /** The zero-width token at the end of the input. */
    EOF;

    /** How a kind with a fixed spelling is scanned. */
    enum Category {
      /** A reserved word, scanned like an identifier. */
      KEYWORD,
      /** An operator or punctuation mark, scanned by longest match. */
      OPERATOR,
    }

    private final String spelling;
    private final Category category;

    Kind() {
      this(null, null);
    }

    Kind(String spelling, Category category) {
      this.spelling = spelling;
      this.category = category;
    }

    /**
     * Returns the source text every token of this kind has, like {@code "if"}, or {@code null} when
     * it varies, as for identifiers and literals.
     */
    public String spelling() {
      return spelling;
    }

    boolean isKeyword() {
      return category == KEYWORD;
    }

    boolean isOperator() {
      return category == OPERATOR;
    }
  }
}
