package io.senbonzakura.lexer;

import static io.senbonzakura.lexer.Lexed.lex;
import static org.junit.jupiter.api.Assertions.assertEquals;

import io.senbonzakura.lexer.Token.Diagnostic;
import io.senbonzakura.lexer.Token.Kind;
import java.util.List;
import org.junit.jupiter.api.Test;
import org.junit.jupiter.params.ParameterizedTest;
import org.junit.jupiter.params.provider.ValueSource;

class ScanCommentTest {

  @ParameterizedTest
  @ValueSource(
      strings = {
        "// only a comment",
        "//",
        "// ends the line\n",
        "/* block */",
        "/**/",
        "/***/",
        "/* spans\nlines */",
        "/* outer /* inner */ still outer */",
        "/*/ not closed by its own slash */",
        "// a\n// b\r\n/* c */ // d",
      })
  void commentsProduceNoTokens(String input) {
    var lexed = lex(input);

    assertEquals(List.of(), lexed.kinds());
    assertEquals(List.of(), lexed.diagnostics());
  }

  @Test
  void lineCommentEndsAtTheLineBreak() {
    var lexed = lex("1 // two\n3 // four\r\n5 // six\r7");

    assertEquals(List.of("1", "3", "5", "7"), lexed.texts());
  }

  @Test
  void blockCommentCanSitBetweenTokens() {
    var lexed = lex("a/*x*/b /* y\n */ c");

    assertEquals(List.of("a", "b", "c"), lexed.texts());
  }

  @Test
  void nestedBlockCommentsEndAtTheirOwnClose() {
    var lexed = lex("/* a /* b */ c */ x /* d */ y");

    assertEquals(List.of("x", "y"), lexed.texts());
  }

  @Test
  void commentsHoldAnythingWithoutDiagnostics() {
    var lexed = lex("// \"no close 'x` # 😀 /*\n/* \"also\" '' # 😀 // */ 1");

    assertEquals(List.of("1"), lexed.texts());
    assertEquals(List.of(), lexed.diagnostics());
  }

  @Test
  void slashOutsideACommentIsStillAnOperator() {
    var lexed = lex("a / b /= c/d */ e");

    assertEquals(
        List.of(
            Kind.IDENTIFIER,
            Kind.SLASH,
            Kind.IDENTIFIER,
            Kind.SLASH_EQ,
            Kind.IDENTIFIER,
            Kind.SLASH,
            Kind.IDENTIFIER,
            Kind.STAR,
            Kind.SLASH,
            Kind.IDENTIFIER),
        lexed.kinds());
  }

  @Test
  void commentMarkersInsideStringsAreText() {
    var lexed = lex("\"// not a comment\" '/' `/* nor this */`");

    assertEquals(
        List.of(Kind.STRING_LITERAL, Kind.CHAR_LITERAL, Kind.STRING_LITERAL), lexed.kinds());
  }

  @ParameterizedTest
  @ValueSource(strings = {"/* x", "/*", "/* /* */ x", "/* x */ /* y"})
  void unterminatedBlockCommentIsReportedAtItsOpening(String input) {
    var lexed = lex(input);

    assertEquals(List.of(), lexed.kinds());
    assertEquals(List.of("unterminated block comment"), lexed.messages());
    assertEquals(List.of("/*"), lexed.diagnosticTexts());
  }

  @Test
  void unterminatedNestedCommentPointsAtTheOutermostOpening() {
    var lexed = lex("1 /* a /* b */");

    assertEquals(List.of("1"), lexed.texts());
    assertEquals(List.of(new Diagnostic(2, 4, "unterminated block comment")), lexed.diagnostics());
  }
}
