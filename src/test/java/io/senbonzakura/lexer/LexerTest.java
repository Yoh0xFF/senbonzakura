package io.senbonzakura.lexer;

import static io.senbonzakura.lexer.Lexed.lex;
import static org.junit.jupiter.api.Assertions.assertEquals;
import static org.junit.jupiter.api.Assertions.assertThrows;
import static org.junit.jupiter.api.Assertions.assertTrue;

import io.senbonzakura.lexer.Token.Diagnostic;
import io.senbonzakura.lexer.Token.Kind;
import java.util.List;
import org.junit.jupiter.api.Test;
import org.junit.jupiter.params.ParameterizedTest;
import org.junit.jupiter.params.provider.ValueSource;

class LexerTest {

  @Test
  void emptyInputIsJustEof() {
    var lexed = lex("");

    assertEquals(List.of(), lexed.kinds());
    assertEquals(new Token(Kind.EOF, 0, 0, 0), lexed.eof());
    assertEquals(List.of(), lexed.diagnostics());
  }

  @ParameterizedTest
  @ValueSource(strings = {" ", "\t", "\n", "\r", "\r\n", " \t\r\n \n"})
  void whitespaceIsSkipped(String whitespace) {
    var lexed = lex(whitespace);

    assertEquals(List.of(), lexed.kinds());
    assertEquals(whitespace.length(), lexed.eof().start());
  }

  @Test
  void eofIsReturnedForeverOnceInputIsUsedUp() {
    var lexer = new Lexer(new SourceFile("test", "1"));
    lexer.nextToken();

    var eof = lexer.nextToken();
    assertEquals(new Token(Kind.EOF, 1, 1, 0), eof);
    assertEquals(eof, lexer.nextToken());
    assertEquals(eof, lexer.nextToken());
  }

  @Test
  void tokensCoverTheirSourceText() {
    var lexed = lex("  12\t\n0x1F \"s\" `m` 'c'  ");

    assertEquals(
        List.of(
            Kind.INT_LITERAL,
            Kind.INT_LITERAL,
            Kind.STRING_LITERAL,
            Kind.STRING_LITERAL,
            Kind.CHAR_LITERAL),
        lexed.kinds());
    assertEquals(List.of("12", "0x1F", "\"s\"", "`m`", "'c'"), lexed.texts());
    assertEquals(2, lexed.tokens().getFirst().start());
    assertEquals(lexed.source().length(), lexed.eof().start());
  }

  @Test
  void unknownCharacterBecomesErrorTokenAndLexingContinues() {
    var lexed = lex("1 # 2");

    assertEquals(List.of(Kind.INT_LITERAL, Kind.ERROR, Kind.INT_LITERAL), lexed.kinds());
    assertEquals(List.of("1", "#", "2"), lexed.texts());
    assertEquals(List.of(new Diagnostic(2, 3, "unexpected character")), lexed.diagnostics());
  }

  @Test
  void unknownCharacterOutsideBmpIsOneToken() {
    var lexed = lex("😀");

    var token = lexed.single();
    assertEquals(Kind.ERROR, token.kind());
    assertEquals(2, token.length()); // a surrogate pair is not split
  }

  @Test
  void uFFFFIsAnOrdinaryCharacterNotEndOfInput() {
    var lexed = lex("￿ 1");

    assertEquals(List.of(Kind.ERROR, Kind.INT_LITERAL), lexed.kinds());
  }

  @Test
  void diagnosticsCanBeLocatedInTheSource() {
    var lexed = lex("1\r\n  #");

    var diagnostic = lexed.diagnostics().getFirst();
    assertEquals(new SourceFile.LineCol(2, 3), lexed.source().locate(diagnostic.start()));
  }

  @Test
  void diagnosticsAreReadOnly() {
    var lexer = new Lexer(new SourceFile("test", "#"));
    lexer.nextToken();

    assertThrows(UnsupportedOperationException.class, () -> lexer.diagnostics().clear());
    assertEquals(1, lexer.diagnostics().size());
  }

  @Test
  void validInputHasNoDiagnostics() {
    var lexed = lex("0 1.5 0b1 \"a\\n\" `x\ny` '\\''");

    assertTrue(lexed.diagnostics().isEmpty(), () -> lexed.messages().toString());
  }

  @Test
  void rejectsNullSource() {
    assertThrows(NullPointerException.class, () -> new Lexer(null));
  }
}
