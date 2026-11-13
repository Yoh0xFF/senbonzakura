package io.senbonzakura.lexer;

import static io.senbonzakura.lexer.Lexed.lex;
import static org.junit.jupiter.api.Assertions.assertEquals;
import static org.junit.jupiter.api.Assertions.assertTrue;

import io.senbonzakura.lexer.Token.Diagnostic;
import io.senbonzakura.lexer.Token.Kind;
import java.util.Arrays;
import java.util.List;
import java.util.stream.Stream;
import org.junit.jupiter.api.Test;
import org.junit.jupiter.params.ParameterizedTest;
import org.junit.jupiter.params.provider.Arguments;
import org.junit.jupiter.params.provider.MethodSource;
import org.junit.jupiter.params.provider.ValueSource;

class ScanOperatorTest {

  static Stream<Kind> operators() {
    return Arrays.stream(Kind.values())
        .filter(kind -> kind.spelling() != null && !kind.isKeyword());
  }

  @ParameterizedTest
  @MethodSource("operators")
  void everyOperatorIsOneToken(Kind operator) {
    var lexed = lex(operator.spelling());

    var token = lexed.single();
    assertEquals(operator, token.kind());
    assertEquals(operator.spelling(), lexed.text(token));
    assertEquals(0, token.flags());
    assertEquals(List.of(), lexed.messages());
  }

  @ParameterizedTest
  @MethodSource("operators")
  void spellingsAreAscii(Kind operator) {
    // ScanOperator skips over a match with Cursor.bump, which is only correct for ASCII.
    assertTrue(operator.spelling().chars().allMatch(ch -> ch < 0x80), operator::name);
  }

  static Stream<Arguments> longestMatch() {
    return Stream.of(
        Arguments.of("<=", List.of(Kind.LESS_EQ)),
        Arguments.of("< =", List.of(Kind.LESS, Kind.EQ)),
        Arguments.of("===", List.of(Kind.EQ_EQ, Kind.EQ)),
        Arguments.of("!==", List.of(Kind.BANG_EQ, Kind.EQ)),
        Arguments.of("!!", List.of(Kind.BANG, Kind.BANG)),
        Arguments.of("...", List.of(Kind.DOT_DOT, Kind.DOT)),
        Arguments.of("-->", List.of(Kind.MINUS, Kind.ARROW)),
        Arguments.of("->=", List.of(Kind.ARROW, Kind.EQ)),
        Arguments.of("+=+", List.of(Kind.PLUS_EQ, Kind.PLUS)),
        Arguments.of("<<", List.of(Kind.LESS, Kind.LESS)),
        Arguments.of(">>=", List.of(Kind.GREATER, Kind.GREATER_EQ)),
        Arguments.of("||||", List.of(Kind.OR_OR, Kind.OR_OR)));
  }

  @ParameterizedTest
  @MethodSource
  void longestMatch(String input, List<Kind> kinds) {
    var lexed = lex(input);

    assertEquals(kinds, lexed.kinds());
    assertEquals(List.of(), lexed.messages());
  }

  @ParameterizedTest
  @ValueSource(strings = {"&", "|", "#", "@", "?", "\\"})
  void charactersThatStartNoOperatorAreErrors(String input) {
    var lexed = lex(input);

    assertEquals(Kind.ERROR, lexed.single().kind());
    assertEquals(List.of(new Diagnostic(0, 1, "unexpected character")), lexed.diagnostics());
  }

  @Test
  void partialMatchFallsBackToAnError() {
    var lexed = lex("&&&");

    assertEquals(List.of(Kind.AND_AND, Kind.ERROR), lexed.kinds());
    assertEquals(List.of("&"), lexed.diagnosticTexts());
  }

  static Stream<Arguments> operatorsBetweenOtherTokens() {
    return Stream.of(
        Arguments.of(
            "a+=1",
            List.of(Kind.IDENTIFIER, Kind.PLUS_EQ, Kind.INT_LITERAL),
            List.of("a", "+=", "1")),
        Arguments.of(
            "f(x,y);",
            List.of(
                Kind.IDENTIFIER,
                Kind.LEFT_PAREN,
                Kind.IDENTIFIER,
                Kind.COMMA,
                Kind.IDENTIFIER,
                Kind.RIGHT_PAREN,
                Kind.SEMICOLON),
            List.of("f", "(", "x", ",", "y", ")", ";")),
        Arguments.of(
            "0..10",
            List.of(Kind.INT_LITERAL, Kind.DOT_DOT, Kind.INT_LITERAL),
            List.of("0", "..", "10")),
        Arguments.of(
            "x.len", List.of(Kind.IDENTIFIER, Kind.DOT, Kind.IDENTIFIER), List.of("x", ".", "len")),
        Arguments.of("-1", List.of(Kind.MINUS, Kind.INT_LITERAL), List.of("-", "1")),
        Arguments.of(
            "a[i]!=\"s\"",
            List.of(
                Kind.IDENTIFIER,
                Kind.LEFT_BRACKET,
                Kind.IDENTIFIER,
                Kind.RIGHT_BRACKET,
                Kind.BANG_EQ,
                Kind.STRING_LITERAL),
            List.of("a", "[", "i", "]", "!=", "\"s\"")));
  }

  @ParameterizedTest
  @MethodSource
  void operatorsBetweenOtherTokens(String input, List<Kind> kinds, List<String> texts) {
    var lexed = lex(input);

    assertEquals(kinds, lexed.kinds());
    assertEquals(texts, lexed.texts());
    assertEquals(List.of(), lexed.messages());
  }

  @Test
  void smallProgram() {
    var lexed = lex("fn add(a: Int, b: Int) -> Int {\n  return a + b;\n}\n");

    assertEquals(
        List.of(
            Kind.FN,
            Kind.IDENTIFIER,
            Kind.LEFT_PAREN,
            Kind.IDENTIFIER,
            Kind.COLON,
            Kind.IDENTIFIER,
            Kind.COMMA,
            Kind.IDENTIFIER,
            Kind.COLON,
            Kind.IDENTIFIER,
            Kind.RIGHT_PAREN,
            Kind.ARROW,
            Kind.IDENTIFIER,
            Kind.LEFT_BRACE,
            Kind.RETURN,
            Kind.IDENTIFIER,
            Kind.PLUS,
            Kind.IDENTIFIER,
            Kind.SEMICOLON,
            Kind.RIGHT_BRACE),
        lexed.kinds());
    assertEquals(List.of(), lexed.messages());
  }
}
