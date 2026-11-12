package io.senbonzakura.lexer;

import static io.senbonzakura.lexer.Lexed.lex;
import static org.junit.jupiter.api.Assertions.assertEquals;

import io.senbonzakura.lexer.Token.Kind;
import java.util.List;
import org.junit.jupiter.api.Test;
import org.junit.jupiter.params.ParameterizedTest;
import org.junit.jupiter.params.provider.CsvSource;
import org.junit.jupiter.params.provider.ValueSource;

class ScanNumberTest {

  @ParameterizedTest
  @ValueSource(
      strings = {
        "0",
        "7",
        "123",
        "007",
        "1_000",
        "1__0",
        "0x1F",
        "0XfF",
        "0xdead_BEEF",
        "0o17",
        "0O7",
        "0b1010",
        "0B1",
        "0b_1"
      })
  void integers(String text) {
    assertValid(text, Kind.INT_LITERAL);
  }

  @ParameterizedTest
  @ValueSource(
      strings = {"1.5", "0.0", "1_0.5_0", "1e10", "1E10", "1e+4", "1e-3", "1.5e-3", "2E+4"})
  void floats(String text) {
    assertValid(text, Kind.FLOAT_LITERAL);
  }

  @ParameterizedTest
  @CsvSource(
      quoteCharacter = '"',
      value = {
        "1.foo, 1",
        "1..2, 1",
        "1., 1",
        "0x1.5, 0x1",
      })
  void dotWithoutDigitAfterItIsNotPartOfTheNumber(String input, String number) {
    var lexed = lex(input);

    assertEquals(Kind.INT_LITERAL, lexed.tokens().getFirst().kind());
    assertEquals(number, lexed.texts().getFirst());
  }

  @Test
  void secondDotEndsTheFloat() {
    var lexed = lex("1.5.6");

    assertEquals(List.of(Kind.FLOAT_LITERAL, Kind.ERROR, Kind.INT_LITERAL), lexed.kinds());
    assertEquals(List.of("1.5", ".", "6"), lexed.texts());
  }

  @ParameterizedTest
  @CsvSource(
      quoteCharacter = '"',
      value = {
        "0x, 0x, expected hexadecimal digits after '0x'",
        "0X, 0X, expected hexadecimal digits after '0X'",
        "0x_, 0x_, expected hexadecimal digits after '0x'",
        "0xg, 0xg, expected hexadecimal digits after '0x'",
        "0o, 0o, expected octal digits after '0o'",
        "0b, 0b, expected binary digits after '0b'",
        "0b102, 2, invalid binary digit or suffix",
        "0o78, 8, invalid octal digit or suffix",
        "0x1g, g, invalid hexadecimal digit or suffix",
      })
  void malformedPrefixedIntegers(String input, String reported, String message) {
    assertMalformed(input, Kind.INT_LITERAL, reported, message);
  }

  @ParameterizedTest
  @CsvSource(
      quoteCharacter = '"',
      value = {
        "12abc, INT_LITERAL, abc, invalid suffix on number literal",
        "1_x, INT_LITERAL, x, invalid suffix on number literal",
        "1.5f, FLOAT_LITERAL, f, invalid suffix on number literal",
        "1e, FLOAT_LITERAL, 1e, expected digits in exponent",
        "1e+, FLOAT_LITERAL, 1e+, expected digits in exponent",
        "1.5E-, FLOAT_LITERAL, 1.5E-, expected digits in exponent",
        "1ex, FLOAT_LITERAL, x, invalid suffix on number literal",
      })
  void malformedDecimalNumbers(String input, Kind kind, String reported, String message) {
    assertMalformed(input, kind, reported, message);
  }

  @Test
  void onlyAsciiDigitsCount() {
    // U+0663 is ARABIC-INDIC DIGIT THREE, which Character.digit would accept.
    var lexed = lex("1٣ ٣");

    assertEquals(List.of(Kind.INT_LITERAL, Kind.ERROR), lexed.kinds());
    assertEquals(List.of("1٣", "٣"), lexed.texts());
    assertEquals(
        List.of("invalid suffix on number literal", "unexpected character"), lexed.messages());
    assertEquals(List.of("٣", "٣"), lexed.diagnosticTexts());
  }

  private static void assertValid(String text, Kind kind) {
    var lexed = lex(text);

    var token = lexed.single();
    assertEquals(kind, token.kind());
    assertEquals(text, lexed.text(token));
    assertEquals(0, token.flags());
    assertEquals(List.of(), lexed.messages());
  }

  // Malformed literals keep their kind and cover the whole literal; the diagnostic says what's
  // wrong.
  private static void assertMalformed(String input, Kind kind, String reported, String message) {
    var lexed = lex(input);

    var token = lexed.single();
    assertEquals(kind, token.kind());
    assertEquals(input, lexed.text(token));
    assertEquals(List.of(message), lexed.messages());
    assertEquals(List.of(reported), lexed.diagnosticTexts());
  }
}
