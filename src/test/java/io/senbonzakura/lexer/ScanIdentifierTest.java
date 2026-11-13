package io.senbonzakura.lexer;

import static io.senbonzakura.lexer.Lexed.lex;
import static org.junit.jupiter.api.Assertions.assertEquals;

import io.senbonzakura.lexer.Token.Kind;
import java.util.Arrays;
import java.util.List;
import java.util.stream.Stream;
import org.junit.jupiter.api.Test;
import org.junit.jupiter.params.ParameterizedTest;
import org.junit.jupiter.params.provider.MethodSource;
import org.junit.jupiter.params.provider.ValueSource;

class ScanIdentifierTest {

  static Stream<Kind> keywords() {
    return Arrays.stream(Kind.values()).filter(Kind::isKeyword);
  }

  @ParameterizedTest
  @ValueSource(
      strings = {
        "x",
        "_",
        "__",
        "_x1",
        "a1_b2",
        "camelCase",
        "snake_case",
        "ünïcödé",
        "日本語",
        "x٣",
        "𝑥"
      })
  void identifiers(String text) {
    var lexed = lex(text);

    var token = lexed.single();
    assertEquals(Kind.IDENTIFIER, token.kind());
    assertEquals(text, lexed.text(token));
    assertEquals(0, token.flags());
    assertEquals(List.of(), lexed.messages());
  }

  @ParameterizedTest
  @MethodSource("keywords")
  void keywords(Kind keyword) {
    var lexed = lex(keyword.spelling());

    var token = lexed.single();
    assertEquals(keyword, token.kind());
    assertEquals(keyword.spelling(), lexed.text(token));
    assertEquals(List.of(), lexed.messages());
  }

  @ParameterizedTest
  @ValueSource(
      strings = {"If", "TRUE", "Fn", "iff", "fnord", "in_", "_if", "if2", "returns", "elseif"})
  void wordsThatOnlyResembleKeywordsAreIdentifiers(String text) {
    assertEquals(Kind.IDENTIFIER, lex(text).single().kind());
  }

  @Test
  void identifierEndsAtTheFirstCharacterThatCannotContinueIt() {
    var lexed = lex("let x\t= y1\"s\"'c'`m`");

    assertEquals(
        List.of(
            Kind.LET,
            Kind.IDENTIFIER,
            Kind.EQ,
            Kind.IDENTIFIER,
            Kind.STRING_LITERAL,
            Kind.CHAR_LITERAL,
            Kind.STRING_LITERAL),
        lexed.kinds());
    assertEquals(List.of("let", "x", "=", "y1", "\"s\"", "'c'", "`m`"), lexed.texts());
  }

  @Test
  void digitsCannotStartAnIdentifier() {
    var lexed = lex("1abc");

    assertEquals(List.of(Kind.INT_LITERAL), lexed.kinds());
    assertEquals(List.of("invalid suffix on number literal"), lexed.messages());
  }

  @Test
  void otherSymbolsCannotStartAnIdentifier() {
    // U+2160 ROMAN NUMERAL ONE is a letter-like number, not a letter; the emoji is a symbol.
    var lexed = lex("Ⅰ 😀 $x");

    assertEquals(List.of(Kind.ERROR, Kind.ERROR, Kind.ERROR, Kind.IDENTIFIER), lexed.kinds());
  }
}
