package io.senbonzakura;

import static org.junit.jupiter.api.Assertions.assertEquals;
import static org.junit.jupiter.api.Assertions.assertFalse;
import static org.junit.jupiter.api.Assertions.assertTrue;

import java.io.BufferedReader;
import java.io.ByteArrayOutputStream;
import java.io.PrintStream;
import java.io.StringReader;
import java.nio.charset.StandardCharsets;
import java.util.Arrays;
import java.util.List;
import org.junit.jupiter.api.Test;
import org.junit.jupiter.params.ParameterizedTest;
import org.junit.jupiter.params.provider.ValueSource;

class ReplTest {

  private final ByteArrayOutputStream out = new ByteArrayOutputStream();
  private final ByteArrayOutputStream err = new ByteArrayOutputStream();

  @Test
  void expressionWithoutSemicolonShowsItsValue() {
    assertEquals(List.of("3"), values("1 + 2\n"));
  }

  @Test
  void entryWithSemicolonShowsNothing() {
    assertEquals(List.of(), values("1 + 2;\nvar x = 1;\n"));
  }

  @Test
  void variablesAndFunctionsLastBetweenEntries() {
    assertEquals(
        List.of("42", "81"),
        values("var x = 2;\nx * 21\nfn sq(n: int): int { return n * n; }\nsq(9)\n"));
  }

  @Test
  void lastExpressionOfAnEntryIsShown() {
    assertEquals(List.of("6"), values("var a = 2; var b = 3; a * b\n"));
  }

  @Test
  void printIsNotShownTwice() {
    assertEquals(List.of("hi"), values("print(`hi`)\n"));
  }

  @Test
  void valuesWithoutAPrintedFormAreNotShown() {
    assertEquals(List.of(), values("fn pair(): int, int { return 1, 2; }\npair()\n"));
  }

  @Test
  void declaringANameAgainShadowsIt() {
    assertEquals(List.of("s"), values("var x = 1;\nvar x = `s`;\nx\n"));
  }

  @Test
  void entryContinuesWhileABraceIsOpen() {
    var values = values("fn f(): int {\n  return 5;\n}\nf()\n");

    assertEquals(List.of("5"), values);
    assertTrue(out().contains(Repl.MORE), out());
  }

  @Test
  void entryContinuesWhileAStringOrCommentIsOpen() {
    assertEquals(List.of("a", "b", "5"), values("`a\nb`\n/* a\ncomment */ 5\n"));
  }

  @Test
  void blankLineEndsAnUnfinishedEntry() {
    var values = values("{\n\n1\n");

    assertEquals(List.of("1"), values);
    assertTrue(err().contains("2:1: error: expected '}', found end of input"), err());
  }

  @Test
  void problemIsReportedAndTheEntryForgotten() {
    var values = values("var y: int = true;\ny\n");

    assertEquals(List.of(), values);
    assertEquals(
        "1:14: error: expected 'int', found 'bool'\n"
            + "    var y: int = true;\n"
            + "                 ^^^^\n"
            + "1:1: error: 'y' is not defined\n"
            + "    y\n"
            + "    ^\n",
        err());
  }

  @Test
  void runtimeErrorIsReportedAndTheEntryForgotten() {
    var values = values("var z = 1 / 0;\nz\n");

    assertEquals(List.of(), values);
    assertTrue(err().startsWith("1:9: runtime error: division by zero\n"), err());
    assertTrue(err().contains("'z' is not defined"), err());
  }

  @Test
  void failedEntryLeavesEarlierVariablesAlone() {
    assertEquals(List.of("1"), values("var a = 1;\na = a / 0;\na\n"));
  }

  @Test
  void stackOverflowIsReportedWithoutAPlace() {
    values("fn f(): int { return f() + 1; }\nf()\n");

    assertEquals("runtime error: stack overflow: the calls nest too deeply\n", err());
  }

  @Test
  void commentOnlyEntryDoesNothing() {
    assertEquals(List.of("1"), values("// just a comment\n/* and another */\n1\n"));
    assertEquals("", err());
  }

  @Test
  void endOfInputEndsThePrompt() {
    assertEquals(Cli.OK, repl("").run());
    assertTrue(out().startsWith(Repl.BANNER), out());
  }

  @ParameterizedTest
  @ValueSource(
      strings = {"{", "f(", "a[", "fn f() {\n  if x {", "`multi", "/* comment", "{ /* } */"})
  void openEntriesAreUnfinished(String text) {
    assertTrue(Repl.unfinished(text));
  }

  @ParameterizedTest
  @ValueSource(strings = {"", "1", "{}", "f()", "}", "`done`", "/* done */", "\"{\"", "// {"})
  void closedEntriesAreFinished(String text) {
    assertFalse(Repl.unfinished(text));
  }

  // Runs the REPL on the input, and returns what it showed, without the banner and prompts.
  private List<String> values(String input) {
    repl(input).run();
    String shown = out().substring(Repl.BANNER.length() + 1);
    return Arrays.stream(shown.replace(Repl.MORE, "").replace(Repl.PROMPT, "").split("\n"))
        .filter(line -> !line.isEmpty())
        .toList();
  }

  private Repl repl(String input) {
    return new Repl(
        new BufferedReader(new StringReader(input)),
        new PrintStream(out, true, StandardCharsets.UTF_8),
        new PrintStream(err, true, StandardCharsets.UTF_8));
  }

  private String out() {
    return out.toString(StandardCharsets.UTF_8).replace(System.lineSeparator(), "\n");
  }

  private String err() {
    return err.toString(StandardCharsets.UTF_8).replace(System.lineSeparator(), "\n");
  }
}
