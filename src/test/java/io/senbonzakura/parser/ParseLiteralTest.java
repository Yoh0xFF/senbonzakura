package io.senbonzakura.parser;

import static io.senbonzakura.parser.Parsed.parse;
import static io.senbonzakura.parser.Parsed.parseClean;
import static org.junit.jupiter.api.Assertions.assertEquals;
import static org.junit.jupiter.api.Assertions.assertFalse;
import static org.junit.jupiter.api.Assertions.assertTrue;

import io.senbonzakura.ast.Expr;
import io.senbonzakura.lexer.Lexer;
import io.senbonzakura.lexer.SourceFile;
import java.util.List;
import org.junit.jupiter.api.Test;
import org.junit.jupiter.params.ParameterizedTest;
import org.junit.jupiter.params.provider.CsvSource;
import org.junit.jupiter.params.provider.ValueSource;

class ParseLiteralTest {

  @ParameterizedTest
  @CsvSource(
      quoteCharacter = '"',
      value = {
        "0, 0",
        "42, 42",
        "007, 7",
        "1_000, 1000",
        "0x1F, 31",
        "0XfF, 255",
        "0xdead_BEEF, 3735928559",
        "0o17, 15",
        "0b1010, 10",
        "0b_1, 1",
        "9223372036854775807, 9223372036854775807",
        "0x7FFF_FFFF_FFFF_FFFF, 9223372036854775807",
      })
  void integers(String input, long value) {
    var parsed = parseClean(input);

    assertEquals(value, parsed.as(Expr.IntLiteral.class).value());
    assertEquals(input, parsed.exprText());
  }

  @ParameterizedTest
  @ValueSource(strings = {"9223372036854775808", "0x8000_0000_0000_0000", "99999999999999999999"})
  void integerTooLargeIsReported(String input) {
    var parsed = parse(input);

    assertEquals(0, parsed.as(Expr.IntLiteral.class).value());
    assertEquals(List.of("integer literal is too large"), parsed.parserMessages());
  }

  @ParameterizedTest
  @CsvSource(
      quoteCharacter = '"',
      value = {
        "1.5, 1.5",
        "0.0, 0.0",
        "1_0.5_0, 10.5",
        "1e10, 1e10",
        "1E+4, 1e4",
        "1.5e-3, 0.0015",
      })
  void floats(String input, double value) {
    var parsed = parseClean(input);

    assertEquals(value, parsed.as(Expr.FloatLiteral.class).value());
    assertEquals(input, parsed.exprText());
  }

  @Test
  void floatOutOfRangeIsReported() {
    var parsed = parse("1e999");

    assertEquals(0.0, parsed.as(Expr.FloatLiteral.class).value());
    assertEquals(List.of("float literal is out of range"), parsed.parserMessages());
  }

  @Test
  void booleans() {
    assertTrue(parseClean("true").as(Expr.BoolLiteral.class).value());
    assertFalse(parseClean("false").as(Expr.BoolLiteral.class).value());
  }

  @ParameterizedTest
  @CsvSource(
      quoteCharacter = '|',
      value = {
        "|\"\"|, ||",
        "|\"hello\"|, |hello|",
        "|\"a\\tb\"|, |a\tb|",
        "|\"q\\\"q\"|, |q\"q|",
        "|\"\\\\\"|, |\\|",
        "|`multi\nline`|, |multi\nline|",
        "|`no \\`break`|, |no `break|",
      })
  void strings(String input, String value) {
    var parsed = parseClean(input);

    assertEquals(value, parsed.as(Expr.StringLiteral.class).value());
    assertEquals(input, parsed.exprText());
  }

  @Test
  void allSimpleEscapes() {
    var parsed = parseClean("\"\\n\\t\\r\\0\\\\\\\"\\'\\`\"");

    assertEquals("\n\t\r\0\\\"'`", parsed.as(Expr.StringLiteral.class).value());
  }

  @Test
  void unicodeEscapes() {
    var parsed = parseClean("\"\\u{41}\\u{e9}\\u{1F600}\"");

    assertEquals("Aé😀", parsed.as(Expr.StringLiteral.class).value());
  }

  @ParameterizedTest
  @ValueSource(strings = {"`a\\\nb`", "`a\\\r\nb`", "`a\\\rb`"})
  void backslashBeforeLineBreakJoinsLines(String input) {
    assertEquals("ab", parseClean(input).as(Expr.StringLiteral.class).value());
  }

  @ParameterizedTest
  @CsvSource(
      quoteCharacter = '|',
      value = {
        "|'a'|, 97",
        "|'\\n'|, 10",
        "|'\\''|, 39",
        "|'é'|, 233",
        "|'😀'|, 128512",
        "|'\\u{1F600}'|, 128512",
      })
  void characters(String input, int codePoint) {
    var parsed = parseClean(input);

    assertEquals(codePoint, parsed.as(Expr.CharLiteral.class).codePoint());
    assertEquals(input, parsed.exprText());
  }

  // The lexer reports these; the parser builds a best-effort node and stays quiet.
  @ParameterizedTest
  @CsvSource(
      quoteCharacter = '|',
      value = {
        "|\"abc|, |abc|",
        "|\"abc\\|, |abc|",
        "|`abc|, |abc|",
        "|\"a\\qb\"|, |aqb|",
        "|\"\\u41\"|, |\uFFFD41|",
        "|\"\\u{41\"|, |\uFFFD|",
        "|\"\\u{D800}\"|, |\uFFFD|",
        "|\"\\u{110000}\"|, |\uFFFD|",
        "|\"\\u{}\"|, |\uFFFD|",
      })
  void malformedStringsGetBestEffortValues(String input, String value) {
    var parsed = parse(input);

    assertEquals(value, parsed.as(Expr.StringLiteral.class).value());
    assertFalse(parsed.lexerDiagnostics().isEmpty(), "the lexer reports the problem");
    assertEquals(List.of(), parsed.parserDiagnostics());
  }

  @ParameterizedTest
  @ValueSource(strings = {"''", "'ab'", "'"})
  void malformedCharactersBecomeTheReplacementCharacter(String input) {
    var parsed = parse(input);

    assertEquals(0xFFFD, parsed.as(Expr.CharLiteral.class).codePoint());
    assertFalse(parsed.lexerDiagnostics().isEmpty(), "the lexer reports the problem");
    assertEquals(List.of(), parsed.parserDiagnostics());
  }

  @ParameterizedTest
  @CsvSource(
      quoteCharacter = '"',
      value = {
        "0x, 0",
        "0b102, 2",
        "1abc, 1",
        "0x1g, 1",
      })
  void malformedIntegersGetBestEffortValues(String input, long value) {
    var parsed = parse(input);

    assertEquals(value, parsed.as(Expr.IntLiteral.class).value());
    assertFalse(parsed.lexerDiagnostics().isEmpty(), "the lexer reports the problem");
    assertEquals(List.of(), parsed.parserDiagnostics());
  }

  @ParameterizedTest
  @CsvSource(
      quoteCharacter = '"',
      value = {
        "1e, 0.0",
        "1.5x, 1.5",
        "2e+, 0.0",
      })
  void malformedFloatsGetBestEffortValues(String input, double value) {
    var parsed = parse(input);

    assertEquals(value, parsed.as(Expr.FloatLiteral.class).value());
    assertFalse(parsed.lexerDiagnostics().isEmpty(), "the lexer reports the problem");
    assertEquals(List.of(), parsed.parserDiagnostics());
  }

  @Test
  void nonLiteralIsReportedAndBecomesAnErrorNode() {
    var parsed = parse(",");

    assertEquals(new Expr.Error(0, 1), parsed.expr());
    assertEquals(List.of("expected expression, found ','"), parsed.parserMessages());
  }

  @Test
  void emptyInputIsReportedAtTheEnd() {
    var parsed = parse("  ");

    assertEquals(new Expr.Error(2, 2), parsed.expr());
    assertEquals(List.of("expected expression, found end of input"), parsed.parserMessages());
  }

  @Test
  void unknownCharacterIsLeftToTheLexer() {
    var parsed = parse("#");

    assertEquals(new Expr.Error(0, 1), parsed.expr());
    assertEquals(
        List.of("unexpected character"),
        parsed.lexerDiagnostics().stream().map(d -> d.message()).toList());
    assertEquals(List.of(), parsed.parserDiagnostics());
  }

  @Test
  void parsesOneExpressionPerCall() {
    var parser = new Parser(new Lexer(new SourceFile("test", "1 \"two\" 3.0 true")));

    var exprs =
        List.of(
            parser.parseExpression(),
            parser.parseExpression(),
            parser.parseExpression(),
            parser.parseExpression());

    assertTrue(parser.atEnd());
    assertEquals(
        List.of(
            new Expr.IntLiteral(1, 0, 1),
            new Expr.StringLiteral("two", 2, 7),
            new Expr.FloatLiteral(3.0, 8, 11),
            new Expr.BoolLiteral(true, 12, 16)),
        exprs);
  }
}
