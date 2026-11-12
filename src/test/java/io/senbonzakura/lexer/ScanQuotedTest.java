package io.senbonzakura.lexer;

import static io.senbonzakura.lexer.Lexed.lex;
import static org.junit.jupiter.api.Assertions.assertEquals;
import static org.junit.jupiter.api.Assertions.assertTrue;

import io.senbonzakura.lexer.Token.Diagnostic;
import io.senbonzakura.lexer.Token.Kind;
import java.util.List;
import java.util.stream.Stream;
import org.junit.jupiter.api.Nested;
import org.junit.jupiter.api.Test;
import org.junit.jupiter.params.ParameterizedTest;
import org.junit.jupiter.params.provider.Arguments;
import org.junit.jupiter.params.provider.MethodSource;
import org.junit.jupiter.params.provider.ValueSource;

// Java source note: a backslash written as "\\" followed by 'u' is not a unicode escape in Java,
// so strings like "\\u{41}" below reach the lexer as backslash, 'u', '{', '4', '1', '}'.
class ScanQuotedTest {

  // The character after the backslash for every simple escape.
  private static final String[] SIMPLE_ESCAPES = {"n", "t", "r", "0", "\\", "\"", "'", "`"};

  // Escaped text the lexer rejects, the part of it the diagnostic points at, and the message.
  static Stream<Arguments> invalidEscapes() {
    return Stream.of(
        Arguments.of("\\q", "\\q", "unknown escape sequence"),
        Arguments.of("\\😀", "\\😀", "unknown escape sequence"),
        Arguments.of("\\u41", "\\u", "expected '{' after '\\u'"),
        Arguments.of("\\u{}", "\\u{}", "unicode escape must have 1 to 6 hex digits"),
        Arguments.of("\\u{1234567}", "\\u{1234567}", "unicode escape must have 1 to 6 hex digits"),
        Arguments.of("\\u{110000}", "\\u{110000}", "unicode escape is not a valid code point"),
        Arguments.of("\\u{D800}", "\\u{D800}", "unicode escape is not a valid code point"),
        Arguments.of("\\u{DFFF}", "\\u{DFFF}", "unicode escape is not a valid code point"),
        Arguments.of("\\u{12", "\\u{12", "unterminated unicode escape, expected '}'"),
        Arguments.of("\\u{12x}", "\\u{12", "unterminated unicode escape, expected '}'"));
  }

  static Stream<String> simpleEscapes() {
    return Stream.of(SIMPLE_ESCAPES);
  }

  @Nested
  class Strings {

    @ParameterizedTest
    @ValueSource(strings = {"\"\"", "\"hi\"", "\"a b\"", "\"😀\"", "\"`'\"", "\"${x}\""})
    void plain(String text) {
      var token = assertValid(text, Kind.STRING_LITERAL);
      assertEquals(0, token.flags());
    }

    @ParameterizedTest
    @MethodSource("io.senbonzakura.lexer.ScanQuotedTest#simpleEscapes")
    void simpleEscape(String escaped) {
      var token = assertValid("\"\\" + escaped + "\"", Kind.STRING_LITERAL);
      assertEquals(Token.HAS_ESCAPES, token.flags());
    }

    @ParameterizedTest
    @ValueSource(strings = {"0", "41", "e9", "1F600", "10FFFF", "000041"})
    void unicodeEscape(String hex) {
      var token = assertValid("\"\\u{" + hex + "}\"", Kind.STRING_LITERAL);
      assertEquals(Token.HAS_ESCAPES, token.flags());
    }

    @ParameterizedTest
    @MethodSource("io.senbonzakura.lexer.ScanQuotedTest#invalidEscapes")
    void invalidEscape(String escape, String reported, String message) {
      // 'z' after the escape: a hex letter there would extend an unterminated unicode escape.
      assertInvalidEscape("\"a" + escape + "z\"", Kind.STRING_LITERAL, reported, message);
    }

    @ParameterizedTest
    @ValueSource(strings = {"\n", "\r", "\r\n"})
    void lineBreakEndsAnUnterminatedString(String lineBreak) {
      var lexed = lex("\"abc" + lineBreak + "1");

      assertEquals(List.of(Kind.STRING_LITERAL, Kind.INT_LITERAL), lexed.kinds());
      assertEquals(List.of("\"abc", "1"), lexed.texts());
      assertEquals(Token.UNTERMINATED, lexed.tokens().getFirst().flags());
      assertEquals(
          List.of(new Diagnostic(0, 4, "unterminated string literal")), lexed.diagnostics());
    }

    @Test
    void endOfInputEndsAnUnterminatedString() {
      var lexed = lex("\"abc");

      var token = lexed.single();
      assertEquals("\"abc", lexed.text(token));
      assertEquals(Token.UNTERMINATED, token.flags());
      assertEquals(List.of("unterminated string literal"), lexed.messages());
    }

    @Test
    void backslashAtEndOfInput() {
      var lexed = lex("\"abc\\");

      var token = lexed.single();
      assertEquals("\"abc\\", lexed.text(token));
      assertEquals(Token.UNTERMINATED | Token.HAS_ESCAPES, token.flags());
      assertEquals(List.of("unterminated string literal"), lexed.messages());
    }

    @Test
    void backslashBeforeLineBreakDoesNotContinueTheLine() {
      var lexed = lex("\"a\\\nb\"");

      var first = lexed.tokens().getFirst();
      assertEquals("\"a\\", lexed.text(first));
      assertTrue(first.has(Token.UNTERMINATED));
      assertEquals("unterminated string literal", lexed.messages().getFirst());
    }
  }

  @Nested
  class MultilineStrings {

    @ParameterizedTest
    @ValueSource(
        strings = {
          "``",
          "`hi`",
          "`a\nb`",
          "`a\r\nb\rc`",
          "`\"'`",
          "`${x}`",
          "`😀`",
          "`\n\n`",
        })
    void plain(String text) {
      var token = assertValid(text, Kind.STRING_LITERAL);
      assertEquals(0, token.flags());
    }

    @ParameterizedTest
    @MethodSource("io.senbonzakura.lexer.ScanQuotedTest#simpleEscapes")
    void simpleEscape(String escaped) {
      var token = assertValid("`\\" + escaped + "`", Kind.STRING_LITERAL);
      assertEquals(Token.HAS_ESCAPES, token.flags());
    }

    @Test
    void unicodeEscape() {
      assertValid("`\\u{1F600}`", Kind.STRING_LITERAL);
    }

    @ParameterizedTest
    @ValueSource(strings = {"\n", "\r", "\r\n"})
    void backslashBeforeLineBreakContinuesTheLine(String lineBreak) {
      var token = assertValid("`a\\" + lineBreak + "b`", Kind.STRING_LITERAL);
      assertEquals(Token.HAS_ESCAPES, token.flags());
    }

    @ParameterizedTest
    @MethodSource("io.senbonzakura.lexer.ScanQuotedTest#invalidEscapes")
    void invalidEscape(String escape, String reported, String message) {
      assertInvalidEscape("`a" + escape + "z`", Kind.STRING_LITERAL, reported, message);
    }

    @Test
    void unterminatedRunsToEndOfInputAndPointsAtTheOpeningBacktick() {
      var lexed = lex("1 `abc\ndef");

      assertEquals(List.of(Kind.INT_LITERAL, Kind.STRING_LITERAL), lexed.kinds());
      assertEquals(List.of("1", "`abc\ndef"), lexed.texts());
      assertEquals(Token.UNTERMINATED, lexed.tokens().get(1).flags());
      assertEquals(
          List.of(new Diagnostic(2, 3, "unterminated multi-line string literal")),
          lexed.diagnostics());
    }

    @Test
    void escapedBacktickDoesNotCloseTheString() {
      var lexed = lex("`a\\`");

      var token = lexed.single();
      assertTrue(token.has(Token.UNTERMINATED));
    }
  }

  @Nested
  class Characters {

    @ParameterizedTest
    @ValueSource(strings = {"'a'", "' '", "'\"'", "'`'", "'😀'", "'\\''", "'\\n'", "'\\u{41}'"})
    void valid(String text) {
      assertValid(text, Kind.CHAR_LITERAL);
    }

    @ParameterizedTest
    @MethodSource("io.senbonzakura.lexer.ScanQuotedTest#simpleEscapes")
    void simpleEscape(String escaped) {
      var token = assertValid("'\\" + escaped + "'", Kind.CHAR_LITERAL);
      assertEquals(Token.HAS_ESCAPES, token.flags());
    }

    @Test
    void empty() {
      var lexed = lex("''");

      assertEquals(Kind.CHAR_LITERAL, lexed.single().kind());
      assertEquals(List.of(new Diagnostic(0, 2, "empty character literal")), lexed.diagnostics());
    }

    @ParameterizedTest
    @ValueSource(strings = {"'ab'", "'\\n\\t'", "'😀😀'"})
    void moreThanOneCharacter(String text) {
      var lexed = lex(text);

      assertEquals(text, lexed.text(lexed.single()));
      assertEquals(
          List.of("character literal must contain exactly one character"), lexed.messages());
      assertEquals(List.of(text), lexed.diagnosticTexts());
    }

    @ParameterizedTest
    @MethodSource("io.senbonzakura.lexer.ScanQuotedTest#invalidEscapes")
    void invalidEscape(String escape, String reported, String message) {
      var lexed = lex("'" + escape + "'");

      assertEquals(Kind.CHAR_LITERAL, lexed.tokens().getFirst().kind());
      assertEquals(message, lexed.messages().getFirst());
      assertEquals(reported, lexed.diagnosticTexts().getFirst());
    }

    @ParameterizedTest
    @ValueSource(strings = {"'", "'x", "'x\n1", "'x\r\n1"})
    void unterminated(String input) {
      var lexed = lex(input);

      var token = lexed.tokens().getFirst();
      assertEquals(Kind.CHAR_LITERAL, token.kind());
      assertEquals(input.lines().findFirst().orElse(""), lexed.text(token));
      assertTrue(token.has(Token.UNTERMINATED));
      assertEquals(List.of("unterminated character literal"), lexed.messages());
    }
  }

  private static Token assertValid(String text, Kind kind) {
    var lexed = lex(text);

    var token = lexed.single();
    assertEquals(kind, token.kind());
    assertEquals(text, lexed.text(token));
    assertEquals(List.of(), lexed.messages());
    return token;
  }

  // A bad escape is reported, but the literal around it is still one complete token.
  private static void assertInvalidEscape(String text, Kind kind, String reported, String message) {
    var lexed = lex(text);

    var token = lexed.single();
    assertEquals(kind, token.kind());
    assertEquals(text, lexed.text(token));
    assertTrue(token.has(Token.HAS_ESCAPES));
    assertEquals(List.of(message), lexed.messages());
    assertEquals(List.of(reported), lexed.diagnosticTexts());
  }
}
